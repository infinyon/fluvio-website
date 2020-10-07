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

Now let's head on over to the `src/main.rs` file. This is where your `main` function
lives. Since we'll be using async code, let's set up a helper function called `run`
that will handle our async needs.

```rust
use fluvio::FluvioError;

fn main() {
    if let Err(error) = async_std::task::block_on(run()) {
        println!("Error: {}", error);
    }
}

async fn run() -> Result<(), FluvioError> {
    todo!()
}
```

This `run()` function is where we'll write the rest of our example code. Since it's an
`async` function, that means that we can just use the `.await` keyword whenever we need
to call another `async` function.

Now let's write the code to produce (send) a message to Fluvio

```rust
async fn run() -> Result<(), FluvioError> {
    let producer = fluvio::producer("hello-fluvio").await?;
    producer.send_record("Hello, Fluvio!", 0);
    Ok(())
}
```

We can already run this code and see it in action. We'll use the `fluvio` CLI to see
the event arrive at the "hello-fluvio" topic.

In one terminal window, run the following command to print out events in the "hello-fluvio"
topic

```bash
$ fluvio consume -B
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

async fn run() -> Result<(), FluvioError> {
    let producer = fluvio::producer("hello-fluvio").await?;
    producer.send_record("Hello, Fluvio!", 0);

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
}
```

This time when we run the code, it will send another "Hello, Fluvio!" message,
then it will print out all messages that have been sent to the topic.

```
$ cargo run
Got record: Hello, Fluvio!
Got record: Hello, Fluvio!
```

Note that your consumer program will hang in the terminal. This is because we
are consuming using a _stream_, which will continue watching for new events to
appear in the topic until we stop it. You can stop the consumer by pressing
`Ctrl-C`.
