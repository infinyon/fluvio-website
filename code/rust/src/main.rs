use async_std::stream::StreamExt;
use chrono::Local;
use fluvio::metadata::topic::TopicSpec;
use fluvio::{Fluvio, RecordKey};

const TOPIC_NAME: &str = "hello-rust";
const PARTITION_NUM: i32 = 0;
const PARTITIONS: i32 = 1;
const REPLICAS: i32 = 1;

#[async_std::main]
async fn main() {
    // Connect to Fluvio cluster
    let fluvio = Fluvio::connect().await.unwrap();

    // Create a topic
    let admin = fluvio.admin().await;
    let topic_spec = TopicSpec::new_computed(PARTITIONS, REPLICAS, None);
    let _topic_create = admin
        .create(TOPIC_NAME.to_string(), false, topic_spec)
        .await;

    // Produce to a topic
    let value = format!("Hello World! - Time is {}", Local::now().to_rfc2822());
    let producer = fluvio::producer(TOPIC_NAME).await.unwrap();
    producer.send(RecordKey::NULL, value).await.unwrap();
    producer.flush().await.unwrap();

    // Consume last record from topic
    let consumer = fluvio::consumer(TOPIC_NAME, PARTITION_NUM).await.unwrap();
    let mut stream = consumer.stream(fluvio::Offset::from_end(1)).await.unwrap();
    if let Some(Ok(record)) = stream.next().await {
        let string = String::from_utf8_lossy(record.value());
        println!("{}", string);
    }
}
