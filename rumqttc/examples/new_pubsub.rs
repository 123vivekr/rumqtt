use tokio::{task, time};

use rumqttc::{self, MqttOptions, Publisher, QoS, ReqHandler};
use std::error::Error;
use std::time::Duration;

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    // color_backtrace::install();

    let mut mqttoptions = MqttOptions::new("test-1", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut sub_handler) = ReqHandler::new(mqttoptions, 10);
    task::spawn(async move {
        sub_handler.start().await.unwrap();
    });

    let publisher = client.publisher("hello/world");
    let subscriber = client
        .subscriber("hello/world", QoS::AtMostOnce)
        .await?;
    task::spawn(async move {
        publish(publisher).await;
        time::sleep(Duration::from_secs(3)).await;
    });

    loop {
        let publish = subscriber.next().await?;
        println!("{:?}", publish);
        if publish.topic.contains("10") {
            subscriber.unsubscribe().await?;
        } 
    }
}

async fn publish(mut publisher: Publisher) {
    publisher.qos = QoS::ExactlyOnce;
    publisher.retain = false;
    for i in 1..=10 {
        publisher.publish(vec![1; i]).await.unwrap();

        time::sleep(Duration::from_secs(1)).await;
    }

    time::sleep(Duration::from_secs(120)).await;
}
