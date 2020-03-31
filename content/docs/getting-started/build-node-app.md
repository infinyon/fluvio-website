---
title: Build a Node App
weight: 20
---

In this guide we’ll cover: how to set up your {{< target-blank title="Node.js" url="https://nodejs.org" >}} environment, and how to build a simple data streaming App.

## Setup Node Environment

A Fluvio environment for Node requires: Node.js and Rust development environments, and a build tool that generates a Node.js library from Rust code. If you have Node installed it should be **version 13** or above.

#### Install Node.js

Node.js installation varies depending on your operating system.

{{< api-table >}}
|   Operating System     |         Instructions           |
|---|---|
| MacOS      | Use the official installer from {{< target-blank title="Node.js" url="https://nodejs.org" >}} to install on **macOS**.  |
| Windows    | Use the official installer from {{< target-blank title="Node.js" url="https://nodejs.org" >}} to install on **Windows**. |
| Linux     | Use the instructions provided by your **Linux** package manager. <br/> Node.js maintains a list of supported packages {{< target-blank title="here" url="https://nodejs.org/en/download/package-manager" >}}.  |
{{< /api-table >}}

#### Install Rust

Rust language utilizes an installer to download and provision Rust on your local system. Refer to {{< target-blank title="rustup" url="https://rustup.rs" >}} documentation to instructions.

##### Install Rust toolchain

Fluvio compiler uses nightly toolchain. To install, run:

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


##### Install Node build tool

The **nj-cli** is a tool that generates native Node interfaces for Rust APIs. Use  **cargo** to install nj-cli tool:

{{< code style="light" >}}
$ cargo install nj-cli
Updating crates.io index
...
Installing /Users/user/.cargo/bin/nj-cli
Installed package `nj-cli v0.1.2` (executable `nj-cli`)    
{{< /code >}}

**Congratulations**, your environment is ready for use!


## Build a simple data streaming App

This section provides a step-by-step on how to build a simple data streaming app. If you'd like to download the app instead, skip ahead to [Download Fluvio data streaming App]({{< relref "#download-fluvio-data-streaming-app" >}}).


#### Start a new Node project

Create a directory for **fluvio-app**:

{{< code style="light" >}}
$ mkdir fluvio-app
$ cd fluvio-app
{{< /code >}}

Run npm to create a **node project** and generate package.json file:

{{< code lang="json" style="light" >}}
$ npm init -y
Wrote to /Users/user/fluvio-app/package.json:
{
  "name": "fluvio-app",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [],
  "author": "fluvio <user@fluvio.io> (fluvio.io)",
  "license": "Apache 2.0"
}
{{< /code >}}


#### Add Fluvio Client library

The client library exports Fluvio data streaming APIs to the Node App.

Use npm install to add {{< target-blank title="@fluvio/client" url="https://www.npmjs.com/package/@fluvio/client" >}} to the project:

{{< code style="light" >}}
$ npm install @fluvio/client --save

> @fluvio/client@2.0.2 install /Users/user/fluvio-app/node_modules/@fluvio/client
> nj-cli build

    Updating crates.io index
    ...
    Finished dev [unoptimized + debuginfo] target(s) in 59.80s

+ @fluvio/client@2.0.2
added 1 package from 1 contributor and audited 1 package in 61.137s
found 0 vulnerabilities
{{< /code >}}

A dependency to @fluvio/client is added to package.json. 

#### Implement Producer/Consumer exchange

Fluvio client looks for the [default profile]({{< relref "profiles" >}}) to identify the location and the authorization token of the cluster. The client connects to the cluster to produce and consume messages.

##### Implement Producer

Inside your node project, create a _src_ directory, and add a _producer.js_ file with the folloing content:

{{< code lang="js" >}}
const FluvioClient = require('@fluvio/client');
 
async function produceMessage() {
  try {
    const flvConnection = await FluvioClient.connect();
    let replica = await flvConnection.replica("my-test", 0);
    let len = await replica.produce("test");
    
    console.log("OK: %d bytes sent" len);
  } catch (e) {
    console.log("error: ", e.msg());
  }
}

await produceMessage();
{{< /code >}}

In summary, the code does the following:

* __require(@fluvio/client)_ loads library into _FluvioClient_ constant. 
* _FluvioClient.connect()_ returns the connection to the cluster.
  * connect reads cluster parameters from _default profile_.
* _flvConnection.replica(...)_ looks-up _replica_ for the topic/partition.
* _replica.produce(...)_ send a message to the _cluster_.

Compile run _producer.js_:

{{< code style="light" >}}
$ node src/produce.js  
.... 

....
{{< /code >}}

##### Implement Consumer

{{< code lang="js" >}}
const FluvioClient = require('@fluvio/client');
const EventEmitter = require('events').EventEmitter;
const emitter = new EventEmitter();

async function consumeMessages() {
  emitter.on('data', (msg) => {
      console.log(msg);
  })

  try {
    const flvConnection = await FluvioClient.connect();
    let replica = await flvConnection.replica("my-test", 0);
    
    replica.consume(emitter.emit.bind(emitter));
  } catch (e) {
    console.log("error: ", e.msg());
  }
}

await consumeMessages();
{{< /code >}}


## Download Fluvio data streaming App

The app is available for download on {{< target-blank title="github" url="https://github.com/infinyon/node-demo-apps" >}}. 



# ----

Congratulation! You have successfully sent your first message!

{{< links "Related Topics" >}}

{{< /links >}}