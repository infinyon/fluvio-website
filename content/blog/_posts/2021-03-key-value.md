---
title: Key/Value records in Fluvio
author:
    name: "The Fluvio Team"
description: Learn how to easily partition data with key/value records in Fluvio.
date: 2021-03-24
slug: key-value-records
url: /blog/2021/03/key-value-records
img: blog/images/key-value/social/florian-berger-keys.jpg
img-credit:
    link: https://unsplash.com/photos/SzG0ncGBOeo
    author: Florian Berger
    site: Upsplash
twitter-card: summary_large_image
---

This week, we're happy to announce the addition of a Key/Value API for
Fluvio producers and consumers! The ability to define a Key for your records
gives you more control over how your data is distributed and stored within
Fluvio. In this blog, we'll talk more about the guarantees that key/value
records give you, as well as how to use key/value records from Fluvio's
various producers and consumers.

## What are Key/Value records and why use them?

Key/Value records are all about determining which partition each record gets sent to.
The golden rule is: **records with the same key always go to the same partition**.
This is great, because we also know that all records that go to the same partition
will be well-ordered, and will be consumed in the same order they were produced.
Generally, you would pick some property of your data to use as the key, such as an
account ID or a username, so that all records belonging to the same user will be
delivered in order. This also means that records belonging to different users may be
distributed across different partitions, making the system free to spread traffic
out across multiple servers and increase throughput.

## Using Key/Value records with the Fluvio CLI

In this section, we'll be showing `fluvio` commands for producing and consuming
key/value records. If you want to follow along, make sure you've followed the
[getting started] guide and set up a Fluvio cluster, either locally or with a
[free Fluvio Cloud account].

[getting started]: /docs/getting-started
[free Fluvio Cloud account]: https://cloud.fluvio.io/signup

Once the cluster is set up, create a fresh topic to use for this example:

```bash
$ fluvio topic create bank-transactions
```

#### Producing key/value records from the CLI

The producer and consumer built into Fluvio's CLI can send and receive key/value
records. Let's look at a quick example of producing data from a text file.

```bash
$ cat transactions.txt
alice=Deposit 100.00
bob=Withdraw 50.00
bob=Withdraw 25.00
```

Here we have a file, `transactions.txt`, with keys and values separated by a `=` and with
one record on each line of the file. We can use the following command to send
each line as a key/value record:

```bash
$ fluvio produce bank-transactions -v --key-separator "=" -f transactions.txt
[alice] Deposit 100.00
[bob] Withdraw 50.00
[bob] Withdraw 25.00
Ok!
```

Let's break down this command:

- `fluvio produce` is how we start up the producer
- `key-value-text` is the name of the topic we want to produce to
- `-v` or (`--verbose`) tells the producer to print each record after it's sent
- `--key-separator "="` tells the producer to split each line on an `=`, using the
left side as the key and the right side as the value
- `-f transactions.txt` tells the producer to read data from the `transactions.txt` file
  
We can tell that the producer recognized the keys correctly because it prints them
back out in square brackets. Next, let's look at how to use a consumer to read back
records that have been stored.

#### Consuming key/value records with the CLI

Let's get right to it and consume our records:

```bash
$ fluvio consume bank-transactions -B -d
Deposit 100.00
Withdraw 50.00
Withdraw 25.00
```

By default, the consumer does not print the keys of each record. This highlights the
fact that key/value records are the same as regular records, they just happen to have keys.
We can tell the consumer to print the keys that belong to each record with `--key-value`:

```bash
$ fluvio consume bank-transactions -B -d --key-value
[alice] Deposit 100.00
[bob] Withdraw 50.00
[bob] Withdraw 25.00
```

## Key/Value records using the Rust API

If you're writing an application in Rust and want to send key/value records to Fluvio,
you can use the new key/value APIs of the `fluvio` crate. Let's set up a project with
everything we need.

```bash
$ cargo new rusty-streams && cd rusty-streams
```

For this project, we'll need the `fluvio` crate, as well as an async runtime and some
Futures helpers. Update your `Cargo.toml` to include these dependencies:

```toml
# Cargo.toml
[dependencies]
fluvio = "0.6.0"
async-std = { version = "1", features = ["attributes"] }
```

The `attributes` feature from `async_std` will let us write async code directly in main!

#### Producing from Rust

Alright, let's write a function that counts from 0 to 4, sending a key/value record
where the key is the number, and the value is a string with the number in it.

```rust
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let producer = fluvio::producer("rusty-topic").await?;
    
    for i in 0..5 {
        producer.send(i.to_string(), format!("This is rusty record {}", i)).await?;
        println!("Sent record {}", i);
    }
    Ok(())
}
```

For this new example, we're using a new topic name, so let's not forget to
create the topic!

```bash
$ fluvio topic create rusty-topic
```

Let's run our producer and check that we get the expected output:

```bash
$ cargo run
Sent record 0
Sent record 1
Sent record 2
Sent record 3
Sent record 4
```

To check if the records were sent, let's use the handy-dandy CLI consumer.

```bash
$ fluvio consume rusty-topic -B -d --key-value
[0] This is rusty record 0
[1] This is rusty record 1
[2] This is rusty record 2
[3] This is rusty record 3
[4] This is rusty record 4
```

Hooray, our producer worked! Let's rewrite main to test out the consumer API in Rust:

```rust
use async_std::stream::StreamExt;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let consumer = fluvio::consumer("rusty-topic", 0).await?;
    let mut stream = consumer.stream(fluvio::Offset::beginning()).await?;
    
    while let Some(Ok(record)) = stream.next().await {
        // Let's convert the key and value into Strings
        let key: Option<String> = record.key().map(|key| String::from_utf8_lossy(key).to_string());
        let value: String = String::from_utf8_lossy(record.value()).to_string();
        println!("Consumed record! Key={:?}, value={}", key, value);
    }
    Ok(())
}
```

Let's run our consumer and see what we get.

```bash
$ cargo run
Consumed record! Key=Some("0"), value=This is rusty record 0
Consumed record! Key=Some("1"), value=This is rusty record 1
Consumed record! Key=Some("2"), value=This is rusty record 2
Consumed record! Key=Some("3"), value=This is rusty record 3
Consumed record! Key=Some("4"), value=This is rusty record 4
^C
```

Sweet! We've now seen how we can quickly and easily write code to produce
and consume key/value records from our Rust applications. Next, let's take a
look at how we can do the same with Fluvio's Node.js API.

## Key/Value records using the Node.js API

For our Node.js app, we'll set up a simple typescript project and pull
in the Fluvio library from `npm`.

```bash
$ mkdir nodejs-streams
$ cd nodejs-streams
$ npm init -y
$ npm install -D typescript ts-node @types/node
$ npm install -S @fluvio/client
$ touch producer.ts consumer.ts
```

Now we have our project set up, and we're ready to write the code for our
`producer.ts` and `consumer.ts` files. Let's start out by writing our
producer.

#### Producing from Node.js

```typescript
// producer.ts
import Fluvio from "@fluvio/client";

const fluvio = new Fluvio();

const produce = async () => {
    await fluvio.connect();
    
    const producer = await fluvio.topicProducer("nodejs-topic");
    for (let i = 0; i < 5; i++) {
        let key = i.toString();
        let value = `This is nodejs record ${i}`;
        await producer.send(key, value);
        console.log(`Sent record ${i}`);
    }
};

produce();
```

Before we run it, let's remember to create our new topic:

```bash
$ fluvio topic create nodejs-topic
```

And let's take our producer for a spin!

```bash
$ npx ts-node producer.ts
Sent record 0
Sent record 1
Sent record 2
Sent record 3
Sent record 4
```

Awesome, let's check that we received everything using the CLI

```bash
$ fluvio consume nodejs-topic -B -d --key-value
[0] This is nodejs record 0
[1] This is nodejs record 1
[2] This is nodejs record 2
[3] This is nodejs record 3
[4] This is nodejs record 4
```

We're nailing it 😎. Only one more example to go, the Node.js consumer.

#### Consuming from Node.js

Our node project is already set up, let's just go ahead and write our
`consumer.ts` code.

```typescript
// consumer.ts
import Fluvio, { Offset } from "@fluvio/client";

const fluvio = new Fluvio();

const consume = async () => {
    await fluvio.connect();
    const consumer = await fluvio.partitionConsumer("nodejs-topic", 0);
    const stream = await consumer.createStream(Offset.FromBeginning());
    
    for await (const record of stream) {
        const key = record.keyString();
        const value = record.valueString();
        console.log(`Consumed record! Key=${key}, value=${value}`);
    }
};

consume();
```

And the moment of truth 🤞

```bash
$ npx ts-node consumer.ts
Consumed record! Key=0, value=This is nodejs record 0
Consumed record! Key=1, value=This is nodejs record 1
Consumed record! Key=2, value=This is nodejs record 2
Consumed record! Key=3, value=This is nodejs record 3
Consumed record! Key=4, value=This is nodejs record 4
^C
```

# Summary

We hope you enjoyed this quick tour of key/value records in Fluvio. Feel free
to [check out our Github], file any feature requests, or ask us questions
[in our community Discord]. We'd love to hear about your streaming use-cases
and help to make Fluvio the best streaming platform it can be!

[check out our Github]: https://github.com/infinyon/fluvio
[in our community Discord]: https://discordapp.com/invite/bBG2dTz

Until next time!

#### Quick links:

- [Getting started with Fluvio](/docs/getting-started/)
- [Fluvio CLI reference](/docs/cli-reference/)
- [Fluvio Architecture](/docs/architecture/)
