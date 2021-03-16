```rust
let consumer = fluvio::consumer("greetings", 0).await?;
let mut stream = consumer.stream(Offset::beginning()).await?;

while let Some(Ok(record)) = stream.next().await {
    let key_bytes = record.key().unwrap();
    let key = String::from_utf8_lossy(key_bytes).to_string();
    let value = String::from_utf8_lossy(record.value()).to_string();
    println!("Consumed record: Key={:?}, value={}", key, value);
}
```
