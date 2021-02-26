```rust
let consumer = fluvio::consumer("greetings", 0).await?;
let mut stream = consumer.stream(Offset::beginning()).await?;

while let Some(Ok(record)) = stream.next().await {
    let string = String::from_utf8_lossy(&record.as_ref());
    println!("Got record: {}", string);
}
```
