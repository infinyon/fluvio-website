```rust
let fluvio_client = FluvioClient::new()?;
let mut connection = fluvio_client.connect().await?;
let mut replica = connection.get_replica("my-topic", 0).await?;
replica.produce("test").await?;
```