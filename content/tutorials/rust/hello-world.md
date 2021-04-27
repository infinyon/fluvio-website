---
title: '"Hello, World! 🎉" in Rust'
toc: true
---

{{< lang-selector >}}

In this guide we’ll provide instructions on how to set up a
<a href="https://www.rust-lang.org" target="_blank">Rust</a>
project and build a simple data streaming app. By the end of this
tutorial, you'll be able to send a message to Fluvio and receive it
back.

## Prerequisites

Before starting on this tutorial, you'll need to have completed the following

- Install the [Rust programming language]
- Have the Fluvio CLI (version  `0.7.0` or greater) installed <sup>[1]</sup>
- Have access to a Fluvio cluster.

See our [getting started] guide for more details on getting set up.

[Rust programming language]: https://rustup.rs
[getting started]: /docs/getting-started

-> [1]: If you need to, you can update the Fluvio CLI by using `fluvio update`.

### Create a Topic using the Fluvio CLI

In Fluvio, we send all of our messages to something called a Topic, which
is like a category for related messages. For this tutorial, we'll create
a topic called `hello-fluvio` using the following command:

```bash
$ fluvio topic create hello-fluvio
```

## Creating a new Cargo project

We'll be using Rust's package manager [Cargo] to set up our project. Cargo
is helpful because it manages compiling our code and dependencies.

[Cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

To create a new Rust project with cargo, run

```bash
$ cargo new --bin hello-fluvio
     Created binary (application) `hello-fluvio` package
```

One of the first things we need to do is add our dependencies. We're going to need
the `fluvio` crate as well as `async-std` because the Fluvio client is async.
In your project folder, edit your `Cargo.toml` and add lines under your `[dependencies]`
section to import `fluvio` and `async-std`. It should look something like this:

```bash
$ cd hello-fluvio
$ cat Cargo.toml
[package]
name = "hello-fluvio"
version = "0.1.0"
authors = ["Your name <your_email@example.com>"]
edition = "2018"

[dependencies]
fluvio = "0.7.0"
async-std = { version = "1", features = ["attributes"] }
```

### Create Producer/Consumer

Now let's head on over to the `src/main.rs` file. This is where your `main` function
lives. Our "Hello World" will be composed of two halves: a producer function, and a
consumer function. Let's add those right next to our `main` function:

```rust
use fluvio::FluvioError;

#[async_std::main]
async fn main() {
    println!("Hello, world!");
}

async fn produce(key: &str, value: &str) -> Result<(), FluvioError> {
    todo!()
}

async fn consume() -> Result<(), FluvioError> {
    todo!()
}
```

Notice that our new functions start with the `async` keyword. We need this because the
Fluvio client library is built with asynchronous code. If you're curious about how async
code in Rust works, [check out the Async Rust book]!

[check out the Async Rust book]: https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html

Run the following command from within your project
directory

```bash
$ cargo run
```

In your consumer window, you should see a message with `Hello, world!` appear!
Ignore the warnings, we'll fix those soon.

We'll start out by writing our producer code, which will send messages
to our Topic.

```rust
async fn produce(key: &str, value: &str) -> Result<(), FluvioError> {
    let producer = fluvio::producer("hello-fluvio").await?;
    producer.send(key, value).await?;
    Ok(())
}
```

That's it for the producer! Let's hook up some code in `main` to call it and test it out.

```rust
#[async_std::main]
async fn main() {
    let _result = produce("Hello", "Fluvio!").await;
}
```

We can now run this code and see it in action. We'll use the `fluvio` CLI to see
the message arrive at the "hello-fluvio" topic.

In one terminal window, run the following command to print out events in the "hello-fluvio"
topic

```bash
$ fluvio consume hello-fluvio -B --key-value
```

Then in another terminal window, run the following command from within your project
directory

```bash
$ cargo run
```

In your consumer window, you should see a message with `[Hello] Fluvio!` appear!
The `[Hello]` is in square brackets to show that it is the key for that record.

Now let's write some code in Rust to do the consuming for us.

```rust
use fluvio::Offset;
use async_std::stream::StreamExt;

async fn consume() -> Result<(), FluvioError> {
    let consumer = fluvio::consumer("hello-fluvio", 0).await?;
    let mut stream = consumer.stream(Offset::beginning()).await?;

    // Iterate over all events in the topic
    while let Some(Ok(record)) = stream.next().await {
        let key_bytes = record.key().unwrap();
        let key = String::from_utf8_lossy(key_bytes).to_string();
        let value = String::from_utf8_lossy(record.value()).to_string();
        println!("Consumed record: Key={}, value={}", key, value);
    }
    Ok(())
}
```

This consumer opens an async [stream] and listens for new events to appear on
the `hello-world` topic. When we run it, it will print out every message
ever sent to the topic because we told it to start reading from the
`Offset::beginning()`, or the beginning of the topic.

[stream]: https://rust-lang.github.io/async-book/05_streams/01_chapter.html

Let's hook up our consumer code into our main function. We don't want to run
both the producer and the consumer at the same time, so let's set up some simple
command-line arguments so we can choose whether to run the producer or the consumer.

```rust
#[async_std::main]
async fn main() {
    // Collect our arguments into a slice of &str
    let args: Vec<String> = std::env::args().collect();
    let args_slice: Vec<&str> = args.iter().map(|s| &**s).collect();

    let result = match &*args_slice {
        [_, "produce"] => {
            produce("Hello", "Fluvio!").await
        },
        [_, "consume"] => {
            consume().await
        },
        _ => {
            println!("Usage: hello-fluvio [produce|consume]");
            return;
        },
    };

    if let Err(err) = result {
        println!("Got error: {}", err);
    }
}
```

Now we can run `cargo run -- produce` to send messages, or `cargo run -- consume`
to read them back. Let's try out our consumer code now:

```bash
$ cargo run -- consume
Consumed record: Key=Hello, value=Fluvio!
```

In another terminal window, let's run your producer one more time

```bash
$ cargo run -- produce
```

You should see another record appear in your consumer window!
You've successfully communicated messages between two processes by streaming
them with Fluvio.

### Bonus: Send your own messages

Let's do something fun and send custom messages with our producer!
We can make the producer send any text that was typed after the `produce`
command like this:

```rust
#[async_std::main]
async fn main() {
    // Collect our arguments into a slice of &str
    let args: Vec<String> = std::env::args().collect();
    let args_slice: Vec<&str> = args.iter().map(|s| &**s).collect();

    let result = match &*args_slice {
        [_, "produce"] => {
            produce("Hello", "Fluvio!").await
        },
        [_, "produce", rest @ ..] => {
            let message = rest.join(" ");
            produce("Custom", &message).await
        },
        [_, "consume"] => {
            consume().await
        },
        _ => {
            println!("Usage: hello-fluvio [produce|consume]");
            return;
        },
    };

    if let Err(err) = result {
        println!("Got error: {}", err);
    }
}
```

Now you can send whatever messages you like! Let's try it out

```bash
$ cargo run -- produce Hello, World! 🎉
```

And in your consumer window, you should see it appear!

```bash
$ cargo run -- consume
Consumed record: Key=Hello, value=Fluvio!
Consumed record: Key=Custom, value=Hello, World! 🎉
```

## Congratulations!

You've now completed the Fluvio "Hello, World! 🎉" tutorial!

Check out the [Fluvio Rust API on docs.rs] to learn more about writing your own Fluvio applications in Rust.

[Fluvio Rust API on docs.rs]: https://docs.rs/fluvio
