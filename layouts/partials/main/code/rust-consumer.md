```rust
let consumer = fluvio::consumer("greetings", 0).await?;
let mut stream = consumer.stream(Offset::beginning()).await?;

while let Ok(event) = stream.next().await {
    for batch in event.partition.records.batches {
        for record in batch.records {
            if let Some(record) = record.value.inner_value() {
                let string = String::from_utf8(record).unwrap();
                println!("Got record: {}", string);
            }
        }
    }
}
```