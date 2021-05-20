---
title: Rust
weight: 10
---

The rust client is the core client for all language clients. It has all the
features before any of the other clients as well as having good support for the
[Admin API]. This client uses [async rust] for all blocking calls.

## Connect

To get a [fluvio connection] do:

```rust
let fluvio : = Fluvio::connect().await.expect("Failed to connect to fluvio");
```

[fluvio connection]: https://docs.rs/fluvio/0.8.0/fluvio/struct.Fluvio.html#method.connect

## Produce

To [create a producer](https://docs.rs/fluvio/0.8.0/fluvio/struct.Fluvio.html#method.topic_producer) do:
```rust
let producer = fluvio.topic_producer("my-fluvio-topic").await.expect("Failed to create a producer");
```

### Send

Once you've got a producer, [send to this topic](https://docs.rs/fluvio/0.8.0/fluvio/struct.TopicProducer.html#method.send)via:
```rust
producer.send("my-key", "my-value").await.expect("Failed to send into a record");
```

## Consume

To [create a consumer](https://docs.rs/fluvio/0.8.0/fluvio/struct.Fluvio.html#method.partition_consumer) do:
```rust
let consumer = fluvio.partition_consumer("my-topic", 0).await.expect("failed to create consumer");
```

### Stream

To [create a stream](https://docs.rs/fluvio/0.8.0/fluvio/consumer/struct.PartitionConsumer.html#method.stream) do:
```rust
let mut stream = consumer.stream(Offset::beginning()).await.expect("Failed to create stream");
```

To use the stream do:
```rust
use futures::StreamExt;
while let Some(Ok(record)) = stream.next().await {
    let key = record.key().map(|key| String::from_utf8_lossy(key).to_string());
    let value = String::from_utf8_lossy(record.value()).to_string();
    println!("Got event: key={:?}, value={}", key, value);
}
```



Refer to the [fluvio docs.rs page] for full detail.

[Admin Api]: https://docs.rs/fluvio/0.8.0/fluvio/struct.FluvioAdmin.html
[async rust]: https://rust-lang.github.io/async-book/
[fluvio docs.rs page]: https://docs.rs/fluvio/
