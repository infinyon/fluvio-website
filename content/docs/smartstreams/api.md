---
title: Rust Filter
weight: 20
toc: false
---

SmartStream filter can be applied when consuming using client libraries.  Here, we are using the Rust client library. This guide assumes you are familiar with [Rust Client API]: {{< ref "/docs/interfaces/apis" >}}

### Write Filter

Follow steps in [filter]: {{< ref "/docs/smartstreams/quick-start" >}} guide to write your own filter


#### Create consumer with filter

To create consumer with filter, you must configure `ConsumerConfig` with WASM bytes.  Typically WASM bytes are loaded from file.  Assume WASM file are stored in separate project in `/my_projects/example_filter/target/wasm32-unknown-unknown/release/example_filter.wasm`


%copy first-line%
```rust

use fluvio::{Fluvio, Offset, PartitionConsumer};

// create consumer config
let mut config = ConsumerConfig::default();
let buffer = std::fs::read("/my_projects/example_filter/target/wasm32-unknown-unknown/release/example_filter.wasm").expect("wasm file is missing");
consume_config = consume_config.with_wasm_filter(buffer);

// create partition consumer
let consumer = fluvio.partition_consumer("my-topic", 0).await.expect("failed to create consumer");
// stream from beginning
let mut stream = consumer.stream(Offset::beginning()).await.expect("Failed to create stream");

use futures::StreamExt;
while let Some(Ok(record)) = stream.next().await {
    let key = record.key().map(|key| String::from_utf8_lossy(key).to_string());
    let value = String::from_utf8_lossy(record.value()).to_string();
    println!("Got filter event: key={:?}, value={}", key, value);
}

```
