---
title: Filter
weight: 20
---

The simplest type of SmartStream is a filter, which can examine each record in
a stream and decide whether to accept or reject it. All records that are accepted
by a filter will be delivered down the pipeline to the consumer, but records that
are rejected will be discarded from the stream. Note that this does not mean that
records are deleted from the partition they are persisted in, it simply means that
those records are not delivered to the consumer.

### Hands-on: How do I write a Filter?

The easiest way to write a filter is to use `cargo-generate` with our SmartStream
template and select the "filter" option.

```bash
$ cargo install cargo-generate
$ cargo generate --git https://github.com/infinyon/fluvio-smartstream-template
🤷   Project Name : example-filter
🔧   Creating project called `example-filter`...
🤷   Which type of SmartStream would you like? [filter] [default: filter]: filter
✨   Done! New project created /home/user/example-filter
```

This will create a project directory called `example-filter` with a typical Rust
crate setup, but with a few special configurations set up to help with compiling
to WASM. If you look at the `src/lib.rs` file, you should see a `filter` function
with some sample code.

```rust
use fluvio_smartstream::{smartstream, Record};

#[smartstream(filter)]
pub fn filter(record: &Record) -> bool {
    let str_result = std::str::from_utf8(record.value.as_ref());
    let string = match str_result {
        Ok(s) => s,
        _ => return false,
    };

    string.contains('a')
}
```

Notice the `#[smartstream(filter)]` attribute. You can think of the function with this
attribute as the entrypoint of your SmartStream, almost like a "main" function, but
you can name it whatever you like. When you run a consumer using this SmartStream,
the Fluvio cluster will call this function once for every record that you would consume.
If your function returns `true`, then Fluvio sends the record to your consumer,
but if it returns `false` then it will not.

You may only have one `#[smartstream]` function in your SmartStream crate, however you
may write any number of supporting functions and types as plain-old Rust. In the example
above, we are taking the byte contents of the Record, interpreting them as a UTF-8 string,
then checking whether any of the characters in that string are the lowercase `a`. You
can of course use any Rust logic you would like here, this is just simple to demonstrate.

### Using a Filter with a Consumer

Since SmartStreams are consumer-facing, we apply them when we go to run our consumer.
In the Fluvio CLI, we do this using the [`fluvio consume --smart-stream`] command.

[`fluvio consume --smart-stream`]: /cli/commands/consume

```bash
$ fluvio consume my-topic --smart-stream="target/wasm32-unknown-unknown/release/example-filter.wasm"
```
