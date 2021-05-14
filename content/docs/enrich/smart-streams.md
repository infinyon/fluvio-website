---
title: Fluvio SmartStreams
menu: SmartStreams
weight: 10
---

SmartStreams are a feature of Fluvio that allow you to write code that filters and
transforms records as they propagate through Fluvio. SmartStreams are compiled
WASM modules that are uploaded by clients to the Fluvio cluster, where they are
applied to streams of records.

We provide support crates that help to write SmartStreams in Rust, though they
may technically be written in any language that can compile to WASM. There are
currently two types of SmartStreams, filters and mappers. Let's take a look at
an example of a filtering SmartStream:

```rust
use fluvio_smartstream::{smartstream, DefaultRecord};

#[smartstream]
fn my_filter(record: &DefaultRecord) -> bool {
    record.value.as_ref().contains('a')
}
```

This 


- SmartStreams are a feature of Fluvio that allow you dynamically edit streams inline
- SmartStream modules are WASM code that can filter or transform records in a topic
- Users write custom WASM SmartStreams to send to the cluster and execute inline
- There are two primary types of SmartStreams, filters and maps
- The easiest way to write SmartStreams is using our Rust project template
- As of today, SmartStreams are applied at consume-time
  - In the future, they may be applied at produce-time?
