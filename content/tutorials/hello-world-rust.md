---
title: '"Hello, World!" in Rust'
hidden: true
group: hello-world
tag: rust
weight: 20
toc: true
---

{{< lang-selector >}}

In this guide we’ll provide instructions on how to set up a <a href="https://www.rust-lang.org" target="_blank">Rust</a> environment and build a simple data streaming App.

{{< idea >}}

**Prerequisites:** Examples in this section require an existing Fluvio cluster and a topic named "my-topic".<br> Step-by-step instructions are available in [Getting Started](/docs/getting-started) at:

* [Create a cluster on Fluvio Cloud](/docs/getting-started/#create-a-fluvio-cloud-account)
* [Add a topic](/docs/getting-started/#create-a-topic-and-stream-hello-world)

{{< /idea >}}

## Setup a Rust Environment

Rust language utilizes an installer to download and provision Rust on your local system. Refer to <a href="https://rustup.rs" target="_blank">rustup</a> documentation to instructions.

##### Install Rust toolchain

Fluvio compiler uses the nightly toolchain. To install, run:

```bash
$ rustup toolchain install nightly
 ...
nightly-x86_64-apple-darwin installed - rustc 1.44.0-nightly (f509b26a7 2020-03-18)
```

Make nightly toolchain default:

```bash
$ rustup default nightly
info: using existing install for 'nightly-x86_64-apple-darwin'
info: default toolchain set to 'nightly-x86_64-apple-darwin'

nightly-x86_64-apple-darwin unchanged - rustc 1.44.0-nightly (f509b26a7 2020-03-18)
```

## Build a simple data streaming App

This section provides a step-by-step on how to build a simple data streaming app using Rust. If you'd like to download the demo app instead, skip ahead to [Download Rust Demo Apps](#download-rust-demo-apps).

#### Implement Producer/Consumer exchange

Fluvio client needs a [Profile](/docs/cli/profiles) to identify the location and the authorization token of the cluster. The file was generated during cluster setup and it is available for download in your <a href="https://app.fluvio.io" target="_blank">Fluvio Cloud</a> account.

#### Create Producer Package

Use <a href="https://doc.rust-lang.org/cargo/" target="_blank">Cargo</a> _--bin_ flag to create an application:

```bash
$ cargo new fluvio-rust-produce --bin
Created binary (application) `fluvio-rust-produce` package
```

This results in following directory layout:

```bash
$ cd fluvio-rust-produce/
$ tree
.
├── Cargo.toml
└── src
    └── main.rs
```

Update _Cargo.toml_ file to add dependencies:

```bash
$ cat Cargo.toml
[package]
name = "fluvio-rust-produce"
version = "0.1.0"
authors = ["user <user@fluvio.io>"]
edition = "2018"

[dependencies]
log = "0.4.8"
futures = { version = "0.3.4", features = ['async-await'] }
flv-future-aio = { version = "1.0.0" }
flv-client = { version = "1.0.0"}
kf-protocol = { version = "1.0.0 "}
```


##### Producer Code

Add the following code in  _src/main.rs_ file:

```rust
use futures::stream::StreamExt;
use flv_future_aio::task::run_block_on;
use flv_client::{FluvioClient, ClientError};

fn main() {
    run_block_on(async {

        match run_producer().await {
            Ok(len) => println!("OK: {} bytes sent", len);
            Err(err) => eprintln!("{}", err);
        }

    })
}

async fn run_producer() -> Result<i32, ClientError> {
    let fluvio_client = FluvioClient::new()?;
    let mut connection = fluvio_client.connect().await?;
    let mut replica = connection.get_replica("my-topic", 0).await?;
    let len = replica.produce("test").await?;

    Ok(len)
}
```

In summary:

* *run_block_on* creates an async block
* *FluvioClient::new()* reads the profile and creates a client.
* *fluvio_client.connect()_ returns the connection to the cluster.
* *connection.get_replica(...)* looks-up _replica_ for the topic/partition.
* *replica.produce(...)* send a message to the _cluster_.

##### Build and Run Producer

Run _cargo build_ to generate binary:

```bash
$ cargo build
   Compiling fluvio-rust-produce v0.1.0 (/Users/user/fluvio-rust-produce)
    Finished dev [unoptimized + debuginfo] target(s) in 2.83s
```

Run _producer_ to send "test" to topic/partition _my-topic/0_ :

```bash
./target/debug/fluvio-rust-produce
OK: 4 bytes sent
```

#### Create Consumer Package

Create a consumer package in parallel with the producer.

```bash
$ cd ..
$ cargo new fluvio-rust-consume --bin
Created binary (application) `fluvio-rust-consume` package
$ cd fluvio-rust-consume/
```

Update _Cargo.toml_:

```bash
$ cat Cargo.toml


[package]
name = "fluvio-rust-consume"
version = "0.1.0"
authors = ["user <user@fluvio.io>"]
edition = "2018"

[dependencies]
log = "0.4.8"
futures = { version = "0.3.4", features = ['async-await'] }
flv-future-aio = { version = "1.0.0" }
flv-client = { version = "1.0.0"}
kf-protocol = { version = "1.0.0 "}
```

##### Consumer Code

Add the following code in  _src/main.rs_ file:

```rust
use futures::stream::StreamExt;
use flv_future_aio::task::run_block_on;
use flv_client::{FluvioClient, ClientError, FetchOffset, FetchLogOption};

fn main() {
    run_block_on(async {

        if let Err(err) = run_consumer.await {
            eprintln!("{}", err);
        }

    })
}

async fn run_consumer() -> Result<(), ClientError> {
    let fluvio_client = FluvioClient::new()?;
    let mut connection = fluvio_client.connect().await?;
    let mut replica = connection.get_replica("my-topic", 0).await?;
    let mut stream = replica.get_stream(FetchOffset::Earliest, FetchLogOption::default())?;

    while let Some(msg) = stream.consume()?.await {
        println!("{}", msg);
    }

    Ok(())
}
```

In summary:

* *run_block_on* creates an async block
* *FluvioClient::new()* reads the profile and creates a client.
* *fluvio_client.connect()* returns the connection to the cluster.
* *connection.get_replica(...)* looks-up _replica_ for the topic/partition.
* *replica.get_stream(...)* opens a stream with _replica_ from 'earliest' offset.
  * *FetchOffset* has additional parameters, see [Replica.Consume](/docs/rust-api/consume) API.
* *stream.consume()* reads messages as the become available.

##### Build and Run Consumer

Run _cargo build_ to generate binary:

```bash
$ cargo build
   Compiling fluvio-rust-consume v0.1.0 (/Users/user/fluvio-rust-consume)
    Finished dev [unoptimized + debuginfo] target(s) in 2.33s
```

Run _consumer_ to receive all messages from topic/partition _my-topic/0_ :

```bash
./target/debug/fluvio-rust-consume
test
^C
```

Consumer listens continuously until &lt;CTRL&gt;-C is pressed.


## Download Rust Demo Apps

Fluvio published a series of examples in github at <a href="https://github.com/infinyon/rust-demo-apps" target="_blank">rust-demo-apps</a>. 

Clone the github repository and navigate to api-examples:

```bash
$ git clone https://github.com/infinyon/rust-demo-apps.git
Cloning into 'rust-demo-apps'
...
Unpacking objects: 100%, done.

$ cd rust-demo-apps/api-examples/
```

The code is organized in a Cargo workspace where each core API has its own package:

```bash
$ tree -L 2
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── consume
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── produce
    ├── Cargo.toml
    └── src
        └── main.rs
```

The benefit of this layout is that all packages can be built at the same time.

###### Compile workspace

Run _cargo build_ to generate binaries for all packages

```bash
$ cargo build
...
   Compiling flv-produce-example v0.1.0 (/Users/user/rust-demo-apps/api-examples/produce)
   Compiling flv-consume-example v0.1.0 (/Users/user/rust-demo-apps/api-examples/consume)
    Finished dev [unoptimized + debuginfo] target(s) in 1m 00s
```

###### Run Producer

Run _producer_ to send messages to topic/partition _my-topic/0_ :

```bash
$ ./target/debug/flv-produce-example
Connected to SC:  127.0.0.1:9003
hello world from rust
ok!
bye
ok!
^C
```

###### Run Consumer

Run _consumer_ to receive messages from topic/partition _my-topic/0_ :

```bash
$ ./target/debug/flv-consume-example
Connected to SC: 127.0.0.1:9003
hello world from rust
bye
^C
```

The APIs are customizable. Checkout <a href="https://docs.rs/flv-client/" target="_blank">Rust API</a>  for additional information.
