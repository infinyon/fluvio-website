---
title: Build a data streaming App in Rust
menu: Build a Rust App
weight: 30
---

In this guide we’ll provide instructions on how to set up a {{< target-blank title="Rust" url="https://www.rust-lang.org" >}} environment and build a simple data streaming App.

{{< idea >}}
**Prerequisites:** The App we'll build in this section needs access to a Fluvio cluster and a topic the app can use to exchange data streams. If you need step-by-step instructions, the are available in [Quick Start]({{< relref "quick-start" >}}) at:

* [Create a cluster on Fluvio Cloud]({{< relref "quick-start/#create-a-fluvio-cloud-account" >}})
* [Add a topic]({{< relref "quick-start/#create-a-topic-and-stream-hello-world" >}})
{{< /idea >}}

## Setup a Rust Environment

Rust language utilizes an installer to download and provision Rust on your local system. Refer to {{< target-blank title="rustup" url="https://rustup.rs" >}} documentation to instructions.

##### Install Rust toolchain

Fluvio compiler uses the nightly toolchain. To install, run:

{{< code style="light">}}
$ rustup toolchain install nightly
 ...
nightly-x86_64-apple-darwin installed - rustc 1.44.0-nightly (f509b26a7 2020-03-18)
{{< /code >}}

Make nightly toolchain default:

{{< code style="light" >}}
$ rustup default nightly
info: using existing install for 'nightly-x86_64-apple-darwin'
info: default toolchain set to 'nightly-x86_64-apple-darwin'

nightly-x86_64-apple-darwin unchanged - rustc 1.44.0-nightly (f509b26a7 2020-03-18)
{{< /code >}}

## Build a simple data streaming App

This section provides a step-by-step on how to build a simple data streaming app using Rust. If you'd like to download the demo app instead, skip ahead to [Download Rust Demo Apps]({{< relref "#download-rust-demo-apps" >}}).

#### Implement Producer/Consumer exchange

Fluvio client needs a [default profile]({{< relref "profiles" >}}) to identify the location and the authorization token of the cluster. The file was generated during cluster setup. The file is available for download in your {{< target-blank title="Fluvio Cloud" url="https://app.fluvio.io" >}} account.

#### Create Producer Package

Rust uses {{< target-blank title="Cargo" url="https://doc.rust-lang.org/cargo/" >}} to create a **rust package** and generate a _Cargo.toml_ dependency file. Use _--bin_ flag to create an application rather than a library.

{{< code lang="json" style="light" >}}
$ cargo new fluvio-rust-produce --bin
Created binary (application) `fluvio-rust-produce` package
{{< /code >}}

Cargo generates a directory with a _Cargo.toml_ file an an _src_ subdirectory:

{{< code lang="json" style="light" >}}
$ cd fluvio-rust-produce/
$ tree
.
├── Cargo.toml
└── src
    └── main.rs
{{< /code >}}

Update _Cargo.toml_ file to add Fluvio dependencies and a [[bin]] section to shorten the binary name:

{{< code lang="json" style="light" >}}
$ cat Cargo.toml
[package]
name = "fluvio-rust-produce"
version = "0.1.0"
authors = ["user <user@fluvio.io>"]
edition = "2018"

[[bin]]
name = "produce"
path = "src/main.rs"
doc = false

[dependencies]
log = "0.4.8"
futures = { version = "0.3.4", features = ['async-await'] }
flv-future-aio = { version = "1.0.0" }
flv-client = { version = "1.0.0"}
kf-protocol = { version = "1.0.0 "}
{{< /code >}}

Build the default code to ensure the dependencies are in working order:

{{< code lang="json" style="light" >}}
$ cargo build
Updating crates.io index
...
Compiling fluvio-rust-produce v0.1.0 (/Users/user/fluvio-rust-produce)
Finished dev [unoptimized + debuginfo] target(s) in 55.91s
{{< /code >}}

Run a quick test:

{{< code lang="json" style="light" >}}
$ ./target/debug/produce
Hello, world!
{{< /code >}}

##### Producer Code

Add the following code in  _src/main.rs_ file:

{{< code lang="rust" >}}
use futures::stream::StreamExt;
use flv_future_aio::task::run_block_on;
use flv_client::{FluvioClient, ClientError};

fn main() {
    run_block_on(async move {

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
{{< /code >}}

In summary:

* *run_block_on* creates an async block
* *FluvioClient::new()* reads the profile and creates a client.
* *fluvio_client.connect()_ returns the connection to the cluster.
* *connection.get_replica(...)* looks-up _replica_ for the topic/partition.
* *replica.produce(...)* send a message to the _cluster_.

##### Build and Run Producer

Build the code again:

{{< code style="light" >}}
$ cargo build
   Compiling fluvio-rust-produce v0.1.0 (/Users/user/fluvio-rust-produce)
    Finished dev [unoptimized + debuginfo] target(s) in 2.83s
{{< /code >}}

Run _producer_ to send "test" to topic/partition _my-topic/0_ :

{{< code style="light" >}}
./target/debug/produce
OK: 4 bytes sent
{{< /code >}}

#### Create Consumer Package

Create a consumer package in parallel with the producer.

{{< code style="light" >}}
$ cd ..
$ cargo new fluvio-rust-consume --bin
Created binary (application) `fluvio-rust-consume` package
$ cd fluvio-rust-consume/
{{< /code >}}

Update _Cargo.toml_:

{{< code lang="json" style="light" >}}
$ cat Cargo.toml
[package]
name = "fluvio-rust-consume"
version = "0.1.0"
authors = ["user <user@fluvio.io>"]
edition = "2018"

[[bin]]
name = "consume"
path = "src/main.rs"
doc = false

[dependencies]
log = "0.4.8"
futures = { version = "0.3.4", features = ['async-await'] }
flv-future-aio = { version = "1.0.0" }
flv-client = { version = "1.0.0"}
kf-protocol = { version = "1.0.0 "}
{{< /code >}}

##### Consumer Code

Add the following code in  _src/main.rs_ file:

{{< code lang="rust" >}}
use futures::stream::StreamExt;
use flv_future_aio::task::run_block_on;
use flv_client::{FluvioClient, ClientError, FetchOffset, FetchLogOption};

fn main() {
    run_block_on(async move {

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
{{< /code >}}

In summary:

* *run_block_on* creates an async block
* *FluvioClient::new()* reads the profile and creates a client.
* *fluvio_client.connect()* returns the connection to the cluster.
* *connection.get_replica(...)* looks-up _replica_ for the topic/partition.
* *replica.get_stream(...)* opens a stream with _replica_ from 'earliest' offset.
  * *FetchOffset* has additional parameters, see [Replica.Consume]({{< relref "../rust-api/consume" >}}) API.
* *stream.consume()* reads messages as the become available.

##### Build and Run Consumer

Build the code again:

{{< code style="light" >}}
$ cargo build
   Compiling fluvio-rust-consume v0.1.0 (/Users/user/fluvio-rust-consume)
    Finished dev [unoptimized + debuginfo] target(s) in 2.33s
{{< /code >}}

Run _consumer_ to receive all messages from topic/partition _my-topic/0_ :

{{< code style="light" >}}
./target/debug/consume
test
^C
{{< /code >}}

Consumer listens continuously until &lt;CTRL&gt;-C is pressed.


## Download Rust Demo Apps

Fluvio published a series of examples in github at {{< target-blank title="rust-demo-apps" url="https://github.com/infinyon/rust-demo-apps" >}}. 

Clone the github repository and navigate to api-examples:

{{< code style="light" >}}
$ git clone https://github.com/infinyon/rust-demo-apps.git
Cloning into 'rust-demo-apps'
...
Unpacking objects: 100%, done.

$ cd rust-demo-apps/api-examples/
{{< /code >}}

The code is organized in a Cargo workspace where each core API has its own package:

{{< code style="light" >}}
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
{{< /code >}}

The benefit of this layout is that all packages can be built at the same time.

###### Compile workspace

Run _cargo build_ to generate binaries for all packages

{{< code style="light" >}}
$ cargo build
...
   Compiling flv-produce-example v0.1.0 (/Users/user/rust-demo-apps/api-examples/produce)
   Compiling flv-consume-example v0.1.0 (/Users/user/rust-demo-apps/api-examples/consume)
    Finished dev [unoptimized + debuginfo] target(s) in 1m 00s
{{< /code >}}

###### Run Producer

Run _producer_ to send messages to topic/partition _my-topic/0_ :

{{< code style="light" >}}
$ ./target/debug/flv-produce-example
Connected to SC:  127.0.0.1:9003
hello world from rust
ok!
bye
ok!
^C
{{< /code >}}

###### Run Consumer

Run _consumer_ to receive messages from topic/partition _my-topic/0_ :

{{< code style="light" >}}
$ ./target/debug/flv-consume-example
Connected to SC: 127.0.0.1:9003
hello world from rust
bye
^C
{{< /code >}}

The APIs are customizable. Checkout [Rust API]({{< relref "../rust-api/reference" >}}) for additional information.

{{< links "Related Topics" >}}
* [Topics CLI]({{< relref "topics" >}})
* [Rust API]({{< relref "../rust-api/reference" >}})
{{< /links >}}