```rust
mod config;
use config::CustomConfig;


use fluvio::{RecordKey, TopicProducer};
use fluvio_connector_common::{
    connector,
    Result
};

#[connector(source)]
async fn start(config: CustomConfig, producer: TopicProducer) -> Result<()> {
    println!("Starting my-connector source connector with {config:?}");
    for i in 1..1000 {
        let value = format!("Hello, Fluvio - {i}");
        producer.send(RecordKey::NULL, value).await?;
        producer.flush().await?;
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    Ok(())
}
```