---
title: Announcing Fluvio Cloud Platform
author:
    name: "The Fluvio Team"
description: Today we are pleased to announce Fluvio Cloud, the easiest way to get started with Fluvio.
metadata: NEWS
date: 2021-02-26
slug: announcing-fluvio-cloud-platform
url: /blog/2021/02/announcing-fluvio-cloud-platform
img: blog/images/fluvio-cloud/social/cloud.jpg
tile: blog/images/fluvio-cloud/social/cloud-tile.svg
twitter-card: summary_large_image
hidden: false
---


Today we are pleased to announce The Fluvio Cloud Platform, the fastest and easiest way to get started using Fluvio.
Fluvio Cloud is now in alpha, and you can create a free account using the link below:

<center><a class="btn btn-primary" href="https://cloud.fluvio.io/signup" target="_blank" role="button">Sign Up for Fluvio Cloud</a></center>

# About Fluvio

Our business research has show that modern businesses require real-time
collaboration, analysis, and adaptation. Yet, building real-time
infrastructure is a painful, expensive, and error-prone endeavor. This is why
we built Fluvio - the open-source, high-performance distributed data streaming
platform for real-time apps that's written in Rust.

Building a streaming tool is just have just half the battle so we built Fluvio
Cloud which provisions and manages your Fluvio cluster for you, letting you get
started right away.  Getting started is as simple as creating an account and
installing the [Fluvio CLI], our all-in-one tool for working with Fluvio.

One of the cool things about Fluvio is that we leverage [Rust's FFI] allowing
us to reuse core compenents in our non-rust client libraries. This is how we
were able to build a [node-js client] without too much work.

There are tons of reasons why Fluvio is awesome but this is a post about using
Fluvio Cloud. :)

[Rust's FFI]: https://doc.rust-lang.org/nomicon/ffi.html#calling-foreign-functions
[Fluvio CLI]: /docs/getting-started/
[node-js client]: https://github.com/infinyon/fluvio-client-node

# Using The Fluvio Cloud Platform

We've written plenty of [Fluvio tutorials](/tutorials) but we'd like to show
how Fluvio cloud acts the same as if you were using it locally or if you host
it yourself.

## Setup
Setting up the cloud is very straight forward as mentioned in the introduction
but we'll reiterate those steps anyway. Once you've verified your account (link
        in the email), a fluvio instance will be automatically provisioned for
you. Then to make use of it, just do
```bash
fluvio cloud login
```
This will propt you for the credentials you used to sign up in the web
form. It will then store this as a profile setting. You can see this setting by
doing:
```bash
fluvio profile view
```

If you've already got a local instance of fluvio running, you can easily switch
the instance the client is using by doing:
```bash
fluvio profile switch cloud
```

## Producing and Consuming a datastream

On a fresh Fluvio instance, you'll need to create some topics:
```bash
fluvio topic create hello-fluvio-cloud
```

Getting data in and out of the cloud is just as easy as all our other tutorials:

In one terminal do:
```bash
fluvio consume hello-fluvio-cloud
```

and in another terminal do:

```bash
fluvio produce hello-fluvio-cloud
```

The produce command listens to `stdin` and sends it to fluvio cloud on the
specified topic. In our case, the topic is `hello-fluvio-cloud`. Now, type
`Hello fluvio cloud!` (or whatever you'd like) into the producer terminal.

Your two terminals should now look like:
```bash
$ fluvio produce hello-fluvio-cloud
Hello fluvio cloud!
Ok!
```

```bash
$ fluvio consume hello-fluvio-cloud
Hello fluvio cloud!
```

Similar to using the commandline, using the [rust
client](https://crates.io/crates/fluvio) and [nodejs
client](https://www.npmjs.com/package/@fluvio/client) requires no addional
steps to use Fluvio Cloud.

We'll leave out the project setup steps but if you have:

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

And run the rust version in one terminal and the node version in the other
terminal, the rust consumer will print:
```
Got Record: Hello Fluvio Cloud! 🎉
```


# Summary

Setting up a real-time data streaming app should be easy and painless. Hopefully this
post shows you just how simple yet powerful our platform is.

Don't forget to join the conversation on
[Discord](https://discordapp.com/invite/bBG2dTz), follow [the project on
github](https://github.com/infinyon/fluvio/watchers) or open an
[issue](https://github.com/infinyon/fluvio/issues) if you find a bug or have a
feature request.
