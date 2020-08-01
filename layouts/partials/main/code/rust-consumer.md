```rust
let fetch = FetchOffset::Earliest;
let opt = FetchLogOption::default();

let mut conn = fluvio_client.connect().await?;
let mut replica = conn.get_replica("my-topic", 0).await?;
let mut stream = replica.get_stream(fetch, opt)?;

while let Some(msg) = stream.consume()?.await {
    println!("{}", msg);
}
```