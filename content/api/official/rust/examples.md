---
title: Rust Examples
menu: Examples 
weight: 20
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

---

### Create a Topic using the Fluvio CLI

In Fluvio, we send all of our messages to something called a Topic, which
is like a category for related messages. For this tutorial, we'll create
a topic called `hello-fluvio` using the following command:

%copy first-line%

```bash
$ fluvio topic create hello-fluvio
```

## Creating a new Cargo project

We'll be using Rust's package manager [Cargo] to set up our project. Cargo
is helpful because it manages compiling our code and dependencies.

[Cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

To create a new Rust project with cargo, run

%copy first-line%

```bash
$ cargo new --bin hello-fluvio
     Created binary (application) `hello-fluvio` package
```

Navigate to the directory and take a look at the Cargo file. It should look something like this:

%copy first-line%

```bash
$ cd hello-fluvio
```

{{<code firstLine="cat Cargo.toml" file="code/rust/cargo-original.toml" copy=true >}}

Clean-up cargo file and add `fluvio` dependencies:
- `fluvio` - to access fluvio libraraies
- `async-std` - because the Fluvio client is async.
    - If you're curious about how async code in Rust works, [check out the Async Rust book]!

[check out the Async Rust book]: https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html

Open your favorite editor and copy/paste:

{{<code file="code/rust/cargo-fluvio.toml" copy=true >}}

Great, it's time to write some code.

### Write Producer/Consumer

Now let's head on over to the `src/main.rs` file. This is where your `main` function
lives. Our "Hello World" will be composed of two halves: a `producer` function, and a
`consumer` function. Let's add those right next to our `main` function:

{{<code file="code/rust/hello-world.rs" lang="rust" copy=true >}}

##### Code Review

- `main()`
    - We don't want to run both the producer and the consumer at the same time, so we've set up some simple
command-line arguments so we can choose whether to run the producer or the consumer.

- `produce()`
    - The producer sends messages to our `hello-fluvio` topic.
    - First parameter is sent as `key`, and the second as `value`.
    - For performance reasons, Fluvio batches all records by default. We use `flush` to ensure our record is sent without delay.

- `consume()`
    - The consumer opens an async [stream] and listens for new events to appear on the `hello-world` topic. 
    - Then, it print out every message ever sent to the topic because we told it to start reading from the `Offset::beginning()`, or the beginning of the topic.

[stream]: https://rust-lang.github.io/async-book/05_streams/01_chapter.html


##### Compile and Run

We can run `cargo run -- produce` to send messages, or `cargo run -- consume`
to read them back. Let's run your producer:

%copy first-line%

```bash
$ cargo run -- produce
```

The producer generate a record and exited.

In a new terminal, let's try out our consumer code:

%copy first-line%

```bash
$ cargo run -- consume
Consumed record: Key=Hello, value=Fluvio!
```

Run the `producer` again to generate additional records. You should see another record appear in your consumer window!

You may also want to view records as they are generate from the CLI. Open another terminal window and run the following command to print out events in the "hello-fluvio" topic

%copy first-line%

```bash
$ fluvio consume hello-fluvio -B --key-value
```

Congratulations! You've successfully communicated messages between two processes by streaming them with Fluvio.


### Bonus: Send your own messages

Let's do something fun and send custom messages with our producer!
We can make the producer send any text that was typed after the `produce`: 

{{<code-highlight file="code/rust/hello-world-custom.rs" lang="rust" lines="15-18" copy=true >}}

Now you can send whatever messages you like! Let's try it out

%copy first-line%

```bash
$ cargo run -- produce Hello, World! 🎉
```

And in your consumer window, you should see it appear!

%copy first-line%

```bash
$ cargo run -- consume
Consumed record: Key=Hello, value=Fluvio!
Consumed record: Key=Custom, value=Hello, World! 🎉
```

