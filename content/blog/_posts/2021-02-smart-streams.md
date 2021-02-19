---
title: Fluvio SmartStreams
author: 
    name: "The Fluvio Team"
description: Fluvio SmartStreams allows users to apply custom logic to real-time traffic.
metadata: TECH
date: 2021-02-18
slug: smart-streams
url: /blog/2021/02/smart-streams
hidden: true
---

Fluvio SmartStreams allows users to apply custom logic to real-time traffic. This powerful capability opens the product to a variety of new use cases such as: real-time filtering to eliminate irrelevant messages, payload modifications to remove personal identity information (PII), or any other business logic triggered by the content of the message.

## SmartStream Programming Language

While SmartStreams can be written in any programming language that compiles to WebAssembly, inline SmartStreams must be written in Rust. 

The following example is a SmartStream filter. 

```rust
#[wasm_bindgen(type="filter")]
/// Filter to check if the message contains a number
pub fn has_number(value: &[u8]) -> bool {
    let message = str::from_utf8(value).unwrap();
    return !message.chars().all(char::is_numeric);
}
```

Filters are expected to return a boolean value:
* **true** sends to send message to consumer
* **false** false to ignore message

Future releases will allow SmartStreams to be written in alternative programming languages such as JavaScript and Python.

## SmartStream Types

This version restricts SmartStreams to consumers, where each consumer may have at most one SmartStream. 

<img src="/blog/images/smart-streams/smart-stream.svg"
     alt="SmartStream Example"
     style="justify: center; max-width: 850px" />

Future releases will allows SmartStreams can be applied to producers as well. Also, producers and consumers will have the ability to chain multiple SmartStreams together in arbitrary sequences.

SmartStreams can support a broad range of operations such as: 
* filters
* maps 
* computations
* and more ...

Initial release will implement filtering.

## SmartStream Provisioning

There are two types of SmartStreams, inline and stored. Inline SmartStreams are a code file read from the command line in the producer CLI. Stored SmartStreams are binary files compiled outside of Fluvio and loaded in the SmartStream Store inside Fluvio. 

### Inline SmartStreams (Phase 1)

Inline SmartStreams are read from the consumer command line. 

Let's assume that the filter above is stored in a file called `sf1.rs`, The following ClI applies the SmartStream to the consumer:

```bash
fluvio consume tp1 smartstream --inline-filter sf1.rs
```

This command loads the SmartStream, compiles the file into binary code, and loads the binary to the SPUs assigned to the topic. Any messages on topic `tp1` that match the filter are sent to the producer. All other messages are ignored. 


### Stored SmartStreams (Phase 2)

Stored SmartStreams are binary files compiled outside of Fluvio and loaded in the SmartStream Store inside Fluvio. 

#### Compile to Binary Format

In this example we are compiling the `sf1.rs` rust file to binary format.

```bash
cargo build --bin sf1.rs --target wasm32-undefined-undefined --release
```

We are using `cargo build` provided by Rust.

<img src="/blog/images/smart-streams/code-2-bin.svg"
     alt="Bot Assistant Example"
     style="justify: center; max-width: 280px" />

Alternatively, you may use your preferred programming language and WASM compiler to build the binary.


#### Add SmartStream to Fluvio Store

Add the SmartStream `sf1` to fluvio store

```bash
fluvio smartstream add sf1 --file sf1.bin
```

The file is stored in Fluvio and ready for use in the consume command.

<img src="/blog/images/smart-streams/add-smart-stream.svg"
     alt="Bot Assistant Example"
     style="justify: center; max-width: 500px" />

Use fluvio CLI to add SmartStream:

The SmartStream is added to the SPU leader and all of its followers.


#### Apply SmartStream to Consumer

The SmartStream can be applied to any of the consumers

<img src="/blog/images/smart-streams/produce-consume.svg"
     alt="Bot Assistant Example"
     style="justify: center; max-width: 850px" />

Apply with SmartStream filter `sf1` to `tp1` :

```bash
fluvio consume tp1 smartstream --use-filter sf1
```

Note, multiple consumers may retrieve messages from the same topic at the same time. In this example, the second consumer retrieves all messages.

```bash
fluvio consume tp1
```

## Conclusion

As the need for real-time data computation increases, SmartStream provides a powerful interface to build highly customizable data centric applications.