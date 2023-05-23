use std::time::Duration;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

async fn produce(server: &str, topic: &str) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", server)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation failure");

    let futures = (0..5)
        .map(|i| async move {
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic)
                        .payload(&format!("Message Chezburger{}", i))
                        .key(&format!("Key {}", i))
                    // .headers(OwnedHeaders::new().insert(Header {
                    //     key: "header_key",
                    //     value: Some("header_value")
                    // }))
                    ,
                    Duration::from_secs(0)
                )
                .await;
            delivery_status
        })
        .collect::<Vec<_>>();

    for future in futures {
        println!("Future completed. Result: {:?}", future.await);
    }
}

#[tokio::main]
async fn main() {
    let kafka = "localhost:19092";
    let topic = "test";
    produce(kafka, topic).await;
}