---
title: Fluvio SmartStreams
menu: Quick start
weight: 10
toc: false
---

SmartStreams are Fluvio data streams that can be customized with user-defined
functions to perform inline operations such as filtering. SmartStreams are compiled
WASM modules written in Rust that get uploaded by clients to the Fluvio cluster,
where they are applied to streams of records.

Currently, SmartStreams are "consumer facing", meaning that all records that are
produced to a topic are stored as-is, and SmartStreams may be applied by consumers.
When a consumer applies a SmartStream to a topic, they receive a stream of records
that have been processed by the SmartStream on the server side (however, the records
persisted in the Topic are not modified). For use-cases like
filtering, this means that consumers can define functions to select only particular
records from the topic that they are interested in, and save bandwidth and latency
by not over-fetching records they don't need.

#### Create a new SmartStream

We've put together some `cargo-generate` templates for creating SmartStreams. To
get started with your own SmartStreams, you can run the following:

%copy first-line%
```bash
$ cargo install cargo-generate
```

%copy first-line%
```bash
$ cargo generate --git https://github.com/infinyon/fluvio-smartstream-template
🤷   Project Name : example-filter
🔧   Creating project called `example-filter`...
🤷   Which type of SmartStream would you like? [filter] [default: filter]: filter
✨   Done! New project created /home/user/example-filter
```

The `cargo generate` command prompts you about which type of SmartStream you'd
like to create. Start by selecting "filter", the simplest type of Smartstream.
After the command completes, you will have a new project folder with a Rust
project set up. This project folder contains some special configurations that
help with building for WASM and integrating with the SmartStream system.

Let's look at the sample code that the template generated for us.

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

The function with the `#[smartstream(filter)]` tag is the entrypoint to the
SmartStream. The SPU that processes our stream will send each Record to this
function and, based on whether the function returns true or false, either send
the record to our consumer or not. This sample SmartStream will check whether
the record's contents are a UTF-8 string and whether that string contains the
letter `a`.

#### Building our SmartStream

To build the SmartStream, just use `cargo build`. We recommend using release
mode in order to make the binary smaller and faster.

%copy first-line%
```bash
$ cargo build --release
```

%copy first-line%
```bash
$ ls -la target/wasm32-unknown-unknown/release/your-package-name.wasm
.rwxr-xr-x  135Ki user 19 May 16:32   example_filter.wasm
```

#### Running our SmartStream

Let's start by [creating a new topic] to test this SmartStream with.

[creating a new topic]: {{< ref "/cli/commands/topic#fluvio-topic-create" >}}

%copy first-line%
```bash
$ fluvio topic create hello-smartstreams
topic "hello-smartstreams" created
```

{{<idea>}}
If you get a connection error while trying to create a topic, make sure you have
followed the Getting Started guide for your OS to set up a Fluvio cluster.
{{</idea>}}

In order to see our filter in effect, we're going to want to open two terminal
windows and run consumers in them. One will be a plain consumer that streams all
the records, and the other will use our SmartStream to filter the records before
it receives them.

In the first terminal window, run the following:

%copy first-line%
```bash
$ fluvio consume hello-smartstreams -B
```

This command will stay open while it waits for records to arrive. In the second
terminal window, run this command in order to set up a consumer with our
SmartStream.

%copy first-line%
```bash
$ fluvio consume hello-smartstreams -B --smart-stream="target/wasm32-unknown-unknown/release/example_filter.wasm"
```

Now, with both of our consumer windows open, let's open one last terminal and
produce some records to our topic. As we send data, we should see records matching
the filter criteria appear in both consumer windows, and records that don't match
the filter criteria should only appear in the plain consumer, not the SmartStream
consumer. Let's send some records.

%copy first-line%
```bash
$ fluvio produce hello-smartstreams
> Apple
Ok!
> Banana
Ok!
> Cabbage
Ok!
> Date
Ok!
> Egg
Ok!
```

If everything worked as expected, we should see all five records appear in the
plain consumer, but only `Banana`, `Cabbage`, and `Date` should appear in the
consumer with our SmartStream filter.

```bash
$ fluvio consume hello-smartstreams -B
Apple
Banana
Cabbage
Date
Egg
```

```bash
$ fluvio consume hello-smartstreams -B \
    --smart-stream="target/wasm32-unknown-unknown/release/example_filter.wasm"
Banana
Cabbage
Date
```

### Read next

- [Writing a JSON filter SmartStream]({{< ref "/docs/smartstreams/filter" >}})
