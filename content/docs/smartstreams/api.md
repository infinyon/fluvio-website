---
title: Rust Filter API
weight: 20
toc: false
---

You can pass the SmartStream filter when creating consumer thru API.  Here, we are using the Rust client library.

This guide assumes you are familiar with [Rust Client API]({{< ref "/docs/interfaces/apis" >}})

### Write Filter

Follow steps in [filter]({{< ref "/docs/smartstreams/filter" >}}) guide to write your own SmartStream filter.


### Create Partition Consumer with filter

To create consumer with filter, you must configure `ConsumerConfig` with WASM bytes.  Typically WASM bytes are loaded from file.  Assume WASM file are stored in separate project in `/my_projects/example_filter/target/wasm32-unknown-unknown/release/example_filter.wasm`


%copy first-line%
```rust

use fluvio::{Fluvio, Offset, PartitionConsumer};

// create consumer config

let buffer = std::fs::read("/my_projects/example_filter/target/wasm32-unknown-unknown/release/example_filter.wasm").expect("wasm file is missing");
let filter_config = ConsumerConfig::default().with_wasm_filter(buffer);

// create partition consumer
let consumer = fluvio.partition_consumer("my-topic", 0).await.expect("failed to create consumer");
// stream from beginning
let mut stream = consumer.stream_with_config(Offset::beginning(),filter_config).await.expect("Failed to create stream");

use futures::StreamExt;
while let Some(Ok(record)) = stream.next().await {
    let key = record.key().map(|key| String::from_utf8_lossy(key).to_string());
    let value = String::from_utf8_lossy(record.value()).to_string();
    println!("Got filter event: key={:?}, value={}", key, value);
}

```
