---
title: '"Hello, World! 🎉" in Rust'
weight: 20
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
- Have the Fluvio CLI installed and have access to a Fluvio cluster. See our [getting started] guide.

[Rust programming language]: https://rustup.rs
[getting started]: /docs/getting-started

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
fluvio = "0.1.0"
async-std = "1.0.0"
```

### Create Producer/Consumer

Now let's head on over to the `src/main.rs` file. This is where your `main` function
lives. Our "Hello World" will be composed of two halves: a producer function, and a
consumer function. Let's add those right next to our `main` function:

```rust
use fluvio::FluvioError;

fn main() {
    println!("Hello, world!");
}

async fn produce(message: &str) -> Result<(), FluvioError> {
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
async fn produce(message: &str) -> Result<(), FluvioError> {
    let producer = fluvio::producer("hello-fluvio").await?;
    producer.send_record(message, 0).await?;
    Ok(())
}
```

That's it for the producer! Let's hook up some code in `main` to call it and test it out.

```rust
use async_std::task::block_on;
fn main() {
    let _result = block_on(produce("Hello, Fluvio!"));
}
```

Notice that we had to use the `block_on` function from `async_std`. This `block_on` function
is acting as our [executor], and is part of the machinery that makes async code in Rust work.

[executor]: https://rust-lang.github.io/async-book/02_execution/04_executor.html

We can now run this code and see it in action. We'll use the `fluvio` CLI to see
the message arrive at the "hello-fluvio" topic.

In one terminal window, run the following command to print out events in the "hello-fluvio"
topic

```bash
$ fluvio consume hello-fluvio -B
```

Then in another terminal window, run the following command from within your project
directory

```bash
$ cargo run
```

In your consumer window, you should see a message with `Hello, Fluvio!` appear!

Now let's write some code in Rust to do the consuming for us.

```rust
use fluvio::Offset;

async fn consume() -> Result<(), FluvioError> {
    let consumer = fluvio::consumer("hello-fluvio", 0).await?;
    let mut stream = consumer.stream(Offset::beginning()).await?;

    // Iterate over all events in the topic
    while let Ok(event) = stream.next().await {
        for batch in event.partition.records.batches {
            for record in batch.records {
                if let Some(record) = record.value.inner_value() {
                    let string = String::from_utf8(record).unwrap();
                    println!("Got record: {}", string);
                }
            }
        }
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
fn main() {
    // Collect our arguments into a slice of &str
    let args: Vec<String> = std::env::args().collect();
    let args_slice: Vec<&str> = args.iter().map(|s| &**s).collect();

    let result = match &*args_slice {
        [_, "produce"] => {
            block_on(produce("Hello, Fluvio!"))
        },
        [_, "consume"] => {
            block_on(consume())
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
Hello, Fluvio!
```

In another terminal window, let's run your producer one more time

```bash
$ cargo run -- produce
```

You should see another `Hello, Fluvio!` message appear in your consumer window!
You've successfully communicated messages between two processes by streaming
them with Fluvio.

### Bonus: Send your own messages

Let's do something fun and send custom messages with our producer!
We can make the producer send any text that was typed after the `produce`
command like this:

```rust
fn main() {
    // Collect our arguments into a slice of &str
    let args: Vec<String> = std::env::args().collect();
    let args_slice: Vec<&str> = args.iter().map(|s| &**s).collect();

    let result = match &*args_slice {
        [_, "produce"] => {
            block_on(produce("Hello, Fluvio!"))
        },
        [_, "produce", rest @ ..] => {
            let message = rest.join(" ");
            block_on(produce(&message))
        },
        [_, "consume"] => {
            block_on(consume())
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
Hello, Fluvio!
Hello, World! 🎉
```

## Congratulations!

You've now completed the Fluvio "Hello, World! 🎉" tutorial! If you want to learn
more about writing Fluvio applications in Rust, be sure to check out the
[Fluvio Rust API on docs.rs].

[Fluvio Rust API on docs.rs]: https://docs.rs/fluvio
