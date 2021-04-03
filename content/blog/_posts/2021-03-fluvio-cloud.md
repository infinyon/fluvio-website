---
title: Announcing Fluvio Cloud Platform
author:
    name: "Sebastian Imlay"
    github: "simlay"
description: Fluvio Cloud, a low latency data streaming platform for App developers like us.
metadata: NEWS
date: 2021-03-07
slug: announcing-fluvio-cloud-platform
url: /blog/2021/03/announcing-fluvio-cloud-platform
img: blog/images/fluvio-cloud/social/cloud.jpg
tile: blog/images/fluvio-cloud/social/cloud-tile.svg
twitter-card: summary_large_image
hidden: false
---


Today we are pleased to announce The Fluvio Cloud Platform, the fastest and easiest way to get started with Fluvio.

Fluvio Cloud is now in `alpha`, and you can create a free account using the link below:

<center><a class="btn btn-primary" href="https://cloud.fluvio.io/signup" target="_blank" role="button">Sign Up for Fluvio Cloud</a></center>

## About Fluvio

Our research has shown that modern businesses require real-time collaboration, analysis, and adaptation. Yet, building real-time infrastructure is a painful, expensive, and error-prone endeavor. This is why we built Fluvio - an open-source, high-performance distributed data streaming platform for real-time apps written in Rust.

Building a streaming app or data pipeline is just half the battle; the other is getting it deployed. We created Fluvio Cloud to provision and manage the Fluvio cluster for you. Getting started is as simple as creating an account and installing the [Fluvio CLI] - our all-in-one tool for working with Fluvio.

Fluvio allows developers to build real-time quickly applications using native language libraries such as <a href="https://infinyon.github.io/fluvio-client-node/" target="_blank">Node.js</a> and <a href="https://docs.rs/fluvio/" target="_blank">Rust</a>. Fluvio clients uses modern programming language constructs such as `async streams` for speed of development and ease of use.

There are many reasons why Fluvio is awesome, but this is a post about using Fluvio Cloud. :wink:

[Fluvio CLI]: /docs/getting-started/
[node-js client]: https://github.com/infinyon/fluvio-client-node

## Using The Fluvio Cloud Platform

There are several [blog posts](/blog) and [tutorials](/tutorials) that show the power of Fluvio, so let's get setup.

### Setup

Setting up the Fluvio Cloud is straightforward; <a href="https://cloud.fluvio.io/signup" target="_blank" role="button">sign up</a> for an account and check your email for a confirmation. Fluvio cluster will be automatically provisioned for you after confirming the email.

Then to make use of it, just run

```bash
fluvio cloud login
```

The command will prompt you for the credentials you used to sign up in the web form. Upon validation, Fluvio retrieves your cloud settings and saves them in your local profile. 

Check out your available profiles at:

```bash
fluvio profile view
```

If you already have a local instance of Fluvio running, you can use the CLI to switch from one Fluvio cluster to another. The following command informs the Fluvio client to use Fluvio Cloud:

```bash
fluvio profile switch cloud
```

### Produce/Consume using CLI

On a fresh Fluvio installation, you'll need to create topics:

```bash
fluvio topic create hello-fluvio-cloud
```

Getting data in and out of the cloud is easy - open two terminals, one for the producer and the other for the consumer. 

In one terminal run:

```bash
fluvio consume hello-fluvio-cloud
```

and in the another:

```bash
fluvio produce hello-fluvio-cloud
```

The produce command listens on `stdin` messages and sends them to Fluvio Cloud on the specified topic. In our case, the `topic` is `hello-fluvio-cloud`. Now, type `Hello fluvio cloud!` (or whatever you'd like) into the producer terminal.

Your two terminals should now look like this:

```bash
$ fluvio produce hello-fluvio-cloud
Hello fluvio cloud!
Ok!
```

```bash
$ fluvio consume hello-fluvio-cloud
Hello fluvio cloud!
```

To exit, press `<CTRL>-C`.

### Produce/Consume using Rust/Node.JS

You can also use `produce` and `consume` programmatically, using our [rust
client](https://crates.io/crates/fluvio) or [nodejs
client](https://www.npmjs.com/package/@fluvio/client). The profile ensures Fluvio clients will use the appropriate cluster, in this case, Fluvio Cloud.

The following code shows a `producer` written in Node.js and a `consumer` written in Rust. (see our [tutorials](/tutorials) for a complete step-by-step):

```javascript
const fluvio = await Fluvio.connect();
const producer = await fluvio.topicProducer('hello-fluvio-cloud');
await producer.sendRecord("Hello Fluvio Cloud! 🎉");
```
and

```rust
let consumer = fluvio::consumer("hello-fluvio-cloud", 0).await?;
let mut stream = consumer.stream(Offset::beginning()).await?;

while let Some(Ok(record)) = stream.next().await {
    let string = String::from_utf8_lossy(&record.as_ref());
    println!("Got record: {}", string);
}
```

Run the rust version in one terminal and the node version in another. The rust consumer will print:

```bash
Got Record: Hello Fluvio Cloud! 🎉
```

All it takes is a few lines of code, and traffic is flowing. All your cluster maintenance headaches are gone.

## Pricing and AWS Region Availability

Fluvio Cloud is currently available in one AWS region in the U.S. (N. Virginia). We plan to add additional region options in the U.S. and Europe and expanding to other parts of the world in the near future.

Fluvio Cloud is in alpha, and it is free to use. We will update pricing information in the near future. Fluvio users that sign-up for our Beta program will receive additional discounts (connect to us in [Discord](https://discordapp.com/invite/bBG2dTz) for details).

## Fluvio Platform Highlights

The following is a shortlist of benefits you gain by using Fluvio:

* **Declarative Management**: Fluvio allows operators to declare the desired state,
and the system will do the rest. No resource available, no worries, the objects
are shown `in progress` until the resource constraints are resolved.

* **Low Latency**: Fluvio takes advantage of Rust’s async runtime and all
available cores. It also interacts directly with hardware I/O to achieve
predictable ultra-low latency.

* **Low Memory Footprint**: Fluvio is a highly optimized machine code, and it does
not require an intermediary virtual machine. Fluvio can run anywhere from
Raspberry Pi to multi-core systems.

* **Built-in Retention**: Fluvio uses long-lived immutable storage to persist
data streams. Each data stream is replicated for persistence and reliability.

* **Guaranteed Message Ordering**: Fluvio guarantees partition-level message
ordering. Messages are stored and forwarded to consumers in the order they are
received from the producers.

* **Cloud Native by Design**: Fluvio is designed to work natively with
Kubernetes. It uses Helm charts for installation, CRDs for provisioning, and
Operators to interact with the KV store.

* **Developer Friendly**: Fluvio offers native language bindings in many modern
programming languages such as Rust and Node (Python, Java, and Swift will be available in future releases).

* **Powerful CLI:** Fluvio CLI can manage all your clusters whether
installed locally, in your data center, or a public cloud.


# Summary

Setting up a real-time data streaming app should be easy and painless. Hopefully, this
post shows you just how simple yet powerful our platform is.

Don't forget to join the conversation on
[Discord](https://discordapp.com/invite/bBG2dTz), follow [the project on
github](https://github.com/infinyon/fluvio/watchers) or open an
[issue](https://github.com/infinyon/fluvio/issues) if you find a bug or have a
feature request.
