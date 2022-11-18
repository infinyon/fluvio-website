use async_std::stream::StreamExt;
use chrono::Local;
use fluvio::metadata::topic::TopicSpec;
use fluvio::{Fluvio, RecordKey};

const TOPIC_NAME: &str = "hello-rust";
const PARTITION_NUM: u32 = 0;
const PARTITIONS: u32 = 1;
const REPLICAS: u32 = 1;

/// This is an example of a basic Fluvio workflow in Rust
///  
/// 1. Establish a connection to the Fluvio cluster
/// 2. Create a topic to store data in
/// 3. Create a producer and send some bytes
/// 4. Create a consumer, and stream the data back
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

    // Create a record
    let record = format!("Hello World! - Time is {}", Local::now().to_rfc2822());

    // Produce to a topic
    let producer = fluvio::producer(TOPIC_NAME).await.unwrap();
    producer.send(RecordKey::NULL, record).await.unwrap();

    // Consume last record from topic
    let consumer = fluvio::consumer(TOPIC_NAME, PARTITION_NUM).await.unwrap();
    let mut stream = consumer.stream(fluvio::Offset::from_end(1)).await.unwrap();
    if let Some(Ok(record)) = stream.next().await {
        let string = String::from_utf8_lossy(record.value());
        println!("{}", string);
    }
}
