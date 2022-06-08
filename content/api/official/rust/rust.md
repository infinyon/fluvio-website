---
title: Rust
menu: SDK
weight: 10
---

The rust client is the core client for all language clients. It has all the
features before any of the other clients as well as having good support for the
[Admin API]. This client uses [async rust] for all blocking calls.

## Connect

To get a [fluvio connection] do:

%copy%
```rust
let fluvio = Fluvio::connect().await.expect("Failed to connect to fluvio");
```

[fluvio connection]: https://docs.rs/fluvio/latest/fluvio/struct.Fluvio.html#method.connect

## Produce

To [create a producer] do:

%copy%
```rust
let producer = fluvio.topic_producer("my-fluvio-topic").await.expect("Failed to create a producer");
```

Alternatively, we can [create a producer with custom configuration] with:

%copy%
```rust
let config = TopicProducerConfigBuilder::default()
    		.batch_size(500)
    		.linger(std::time::Duration::from_millis(500))
    		.compression(Compression::Gzip)
    		.build().expect("Failed to create topic producer config");
let producer = fluvio.topic_producer_with_config("my-fluvio-topic", config).await.expect("Failed to create a producer");
```

[create a producer]: https://docs.rs/fluvio/latest/fluvio/struct.Fluvio.html#method.topic_producer
[create a producer with custom configuration]: https://docs.rs/fluvio/latest/fluvio/struct.Fluvio.html#method.topic_producer_with_config

### Send

Once you've got a producer, [send to this topic] via:

%copy%
```rust
producer.send("my-key", "my-value").await.expect("Failed to send into a record");
producer.flush().await.expect("Flush failed");
```

[send to this topic]: https://docs.rs/fluvio/latest/fluvio/struct.TopicProducer.html#method.send

## Consume

To [create a consumer] do:

%copy%
```rust
let consumer = fluvio.partition_consumer("my-topic", 0).await.expect("failed to create consumer");
```

[create a consumer]: https://docs.rs/fluvio/latest/fluvio/struct.Fluvio.html#method.partition_consumer

### Stream

To [create a stream] do:

%copy%
```rust
let mut stream = consumer.stream(Offset::beginning()).await.expect("Failed to create stream");
```

[create a stream]: https://docs.rs/fluvio/latest/fluvio/consumer/struct.PartitionConsumer.html#method.stream

To use the stream do:

%copy%
```rust
use futures::StreamExt;
while let Some(Ok(record)) = stream.next().await {
    let key = record.key().map(|key| String::from_utf8_lossy(key).to_string());
    let value = String::from_utf8_lossy(record.value()).to_string();
    println!("Got event: key={:?}, value={}", key, value);
}
```

### Using a SmartModule with the Rust Consumer

Below is an example of how to use a SmartModule filter with the Rust
programmatic consumer.

```rust
use std::io::Read;
use flate2::bufread::GzEncoder;
use flate2::Compression;
use fluvio::{Fluvio, Offset, PartitionConsumer};
use fluvio::consumer::{
    SmartModuleInvocation,
    SmartModuleInvocationWasm,
    SmartModuleKind,
    ConsumerConfig
};
use async_std::stream::StreamExt;

let raw_buffer = std::fs::read("/my_projects/example_filter/target/wasm32-unknown-unknown/release/example_filter.wasm").expect("wasm file is missing");
let mut encoder = GzEncoder::new(raw_buffer.as_slice(), Compression::default());
let mut buffer = Vec::with_capacity(raw_buffer.len());
encoder.read_to_end(&mut buffer);

let mut builder = ConsumerConfig::builder();
builder.smartmodule(Some(SmartModuleInvocation {
    wasm: SmartModuleInvocationWasm::AdHoc(buffer),
    kind: SmartModuleKind::Filter,
    params: Default::default()
}));
let filter_config = builder.build().expect("Failed to create config");

// create partition consumer
let consumer = fluvio.partition_consumer("my-topic", 0).await.expect("failed to create consumer");
// stream from beginning
let mut stream = consumer.stream_with_config(Offset::beginning(),filter_config).await.expect("Failed to create stream");

while let Some(Ok(record)) = stream.next().await {
    let key = record.key().map(|key| String::from_utf8_lossy(key).to_string());
    let value = String::from_utf8_lossy(record.value()).to_string();
    println!("Got filter event: key={:?}, value={}", key, value);
}
```

Refer to the [fluvio docs.rs page] for full detail.

[Admin Api]: https://docs.rs/fluvio/latest/fluvio/struct.FluvioAdmin.html
[async rust]: https://rust-lang.github.io/async-book/
[fluvio docs.rs page]: https://docs.rs/fluvio/
