---
title: Fluvio SmartStreams
menu: SmartStreams
weight: 10
---

- SmartStreams are a feature of Fluvio that allow you dynamically edit streams inline
- SmartStream modules are WASM code that can filter or transform records in a topic
- Users write custom WASM SmartStreams to send to the cluster and execute inline
- There are two primary types of SmartStreams, filters and maps
- The easiest way to write SmartStreams is using our Rust project template
- As of today, SmartStreams are applied at consume-time
  - In the future, they may be applied at produce-time?
