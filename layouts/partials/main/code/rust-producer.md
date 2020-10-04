```rust
let producer = fluvio::producer("greetings").await?;
producer.send_record("Hello, Fluvio!", 0).await?;
```