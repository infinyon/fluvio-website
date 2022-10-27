---
title: Rust Examples
menu: Examples 
weight: 20
---

* The Rust client is the core client for all language clients.
  * New features arrive in the Rust client before any of the other clients
* Full support for the [Admin API](https://docs.rs/fluvio/latest/fluvio/struct.FluvioAdmin.html).
* This client uses [async Rust](https://rust-lang.github.io/async-book/) for all blocking calls.

Refer to the [fluvio docs.rs page] for full detail.
## Example Workflow

Follow the [installation instructions]({{< ref "installation.md" >}}) to run this example.

{{<code file="embeds/client-examples/rust/Cargo.toml" lang="toml" copy=true >}}

{{<code file="embeds/client-examples/rust/src/main.rs" lang="rust" copy=true >}}

### Run

%copy first-line%
```shell
$ cargo run
```
## Additional Producer options

Alternatively, we can [create a producer with custom configuration]:

Example:

This is how to configure a Producer with a 
`batch_size` of `500 bytes`, linger of `500ms` , and `Gzip` type compression.

%copy%
```rust
let config = TopicProducerConfigBuilder::default()
    		.batch_size(500)
    		.linger(std::time::Duration::from_millis(500))
    		.compression(Compression::Gzip)
    		.build().expect("Failed to create topic producer config");
let producer = fluvio.topic_producer_with_config("my-fluvio-topic", config).await.expect("Failed to create a producer");
```

[create a producer with custom configuration]: https://docs.rs/fluvio/latest/fluvio/struct.Fluvio.html#method.topic_producer_with_config

## Using a SmartModule with the Rust Consumer

Below is an example of how to use a SmartModule filter with the Rust consumer.

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



## Links to Docs:
* [Connect to Fluvio](https://docs.rs/fluvio/0.12.12/fluvio/struct.Fluvio.html#method.connect)
* [Get a Producer](https://docs.rs/fluvio/latest/fluvio/struct.Fluvio.html#method.topic_producer)
* [Send to Topic](https://docs.rs/fluvio/latest/fluvio/struct.TopicProducer.html#method.send)
* [Get a Consumer](https://docs.rs/fluvio/latest/fluvio/struct.Fluvio.html#method.partition_consumer)
* [Get a Stream](https://docs.rs/fluvio/latest/fluvio/consumer/struct.PartitionConsumer.html#method.stream)