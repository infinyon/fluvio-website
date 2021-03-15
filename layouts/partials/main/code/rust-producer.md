```rust
let producer = fluvio::producer("greetings").await?;
producer.send("Hello", "Fluvio!").await?;
```