```rust
let consumer = fluvio::consumer("greetings", 0).await?;
let mut stream = consumer.stream(Offset::beginning()).await?;

while let Some(Ok(record)) = stream.next().await {
    if let Some(bytes) = record.try_into_bytes() {
        let string = String::from_utf8_lossy(&bytes);
        println!("Got record: {}", string);
    }
}
```