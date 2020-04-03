---
title: Build a data streaming up App in Node.js
menu: Build a Node App
toc: true
weight: 20
---

In this guide we’ll provide instructions on how to set up a {{< target-blank title="Node.js" url="https://nodejs.org" >}} environment and build a simple data streaming App.

{{< idea >}}
**Prerequisites:** Examples in this section require an existing Fluvio cluster and a topic named "my-topic".<br> Step-by-step instructions are available in [Quick Start]({{< relref "quick-start" >}}) at:

* [Create a cluster on Fluvio Cloud]({{< relref "quick-start/#create-a-fluvio-cloud-account" >}})
* [Add a topic]({{< relref "quick-start/#create-a-topic-and-stream-hello-world" >}})
{{< /idea >}}

## Setup a Node Environment

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

This section provides a step-by-step on how to build a simple data streaming app using Node.js. If you'd like to download the demo app instead, skip ahead to [Download Fluvio Demo Apps]({{< relref "#download-fluvio-demo-apps" >}}).


#### Start a new Node project

Create a directory for **fluvio-node-app**:

{{< code style="light" >}}
$ mkdir fluvio-node-app
$ cd fluvio-node-app
{{< /code >}}

Run npm to create a **node project** and generate package.json file:

{{< code lang="json" style="light" >}}
$ npm init -y
Wrote to /Users/user/fluvio-node-app/package.json:
{
  "name": "fluvio-node-app",
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

> @fluvio/client@2.0.2 install /Users/user/fluvio-node-app/node_modules/@fluvio/client
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

Fluvio client needs a [default profile]({{< relref "profiles" >}}) to identify the location and the authorization token of the cluster. The file was generated during cluster setup and it is available for download in your {{< target-blank title="Fluvio Cloud" url="https://app.fluvio.io" >}} account.

##### Create Producer

Inside your node project, create a _src_ directory, and add a _produce.js_ file: 

{{< code style="light" >}}
$ tree -L 2
.
├── node_modules
│   └── @fluvio
├── package-lock.json
├── package.json
└── src
    └── produce.js
{{< /code >}}

###### Producer Code

Add the following code in the _produce.js_ file:

{{< code lang="js" >}}
const FluvioClient = require('@fluvio/client');
 
async function produceMessage() {
  try {
    const flvConnection = await FluvioClient.connect();
    let replica = await flvConnection.replica("my-topic", 0);
    let len = await replica.produce("test");
    
    console.log("OK: %d bytes sent", len);
  } catch (e) {
    console.log("error: ", e.msg());
  }
}

await produceMessage();
{{< /code >}}

In summary:

* _require(@fluvio/client)_ loads library into _FluvioClient_ constant. 
* _FluvioClient.connect()_ returns the connection to the cluster.
  * connect reads the cluster parameters from _default profile_.
* _flvConnection.replica(...)_ looks-up _replica_ for the topic/partition.
* _replica.produce(...)_ send a message to the _cluster_.

###### Run Producer

Run _produce.js_ to send "test" to topic/partition _my-topic/0_ :

{{< code style="light" >}}
$ node ./src/produce.js  
OK: 4 bytes sent
{{< /code >}}

To generate additional data entries, call _node ./src/produce.js_ multiple times.


##### Create Consumer

Inside your _src_ directory, and add a _consume.js_ file: 

{{< code style="light" >}}
$ tree -L 2
.
├── node_modules
│   └── @fluvio
├── package-lock.json
├── package.json
└── src
    ├── consume.js
    └── produce.js
{{< /code >}}

###### Consumer Code

Add the following code in the _consume.js_ file:

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
    let replica = await flvConnection.replica("my-topic", 0);
    
    replica.consume({ 
        offset: "earliest"
      },
      emitter.emit.bind(emitter)
    );

  } catch (e) {
    console.log("error: ", e.msg());
  }
}

await consumeMessages();
{{< /code >}}

In summary:

* _require(@fluvio/client)_ loads library into _FluvioClient_ constant. 
* _emitter.on('data')_ creates an emitter that is invoked by _replica.consume(..)_ when new messages arrive.
* _FluvioClient.connect()_ returns the connection to the cluster.
  * connect reads the cluster parameters from _default profile_.
* _flvConnection.replica(...)_ looks-up _replica_ for the topic/partition.
* _replica.consume(...)_ reads messages from the 'earliest' offset in real-time.
  * _consume_ has additional parameters, see [Replica.Consume]({{< relref "../node-api/consume" >}}) API.

###### Run Consumer

Run _consume.js_ to receive all messages from topic/partition _my-topic/0_ :

{{< code style="light" >}}
$ node ./src/consume.js 
test
test
^C
{{< /code >}}

Consumer listens continuously until &lt;CTRL&gt;-C is pressed.


## Download Node Demo Apps

Fluvio published a series of examples in github at {{< target-blank title="node-demo-apps" url="https://github.com/infinyon/node-demo-apps" >}}. 

Clone the github repository and navigate to api-examples:

{{< code style="light" >}}
$ git clone https://github.com/infinyon/node-demo-apps.git
Cloning into 'node-demo-apps'
...
Unpacking objects: 100%, done.

$ cd node-demo-apps/api-examples/
{{< /code >}}

This repository has working examples centered around the core APIs:

{{< code style="light" >}}
$ tree -L 2
.
├── README.md
├── package-lock.json
├── package.json
└── src
    ├── consume.js
    ├── produce.js
    └── utils
{{< /code >}}

The directory structure has the following components:

* **consume.js** - consumer example
* **produce.js** - producer example
* **utils** - utility functions to support the APIs such as CLI.

###### Compile api-examples

Run npm install to download dependencies such as @fluvio/client:

{{< code style="light" >}}
$ npm install
> @fluvio/client@0.1.2 install /Users/user/node-demo-apps/api-examples/node_modules/@fluvio/client
> nj-cli build
...
  Finished dev [unoptimized + debuginfo] target(s) in 59.22s

added 3 packages from 3 contributors and audited 3 packages in 59.756s
found 0 vulnerabilities
{{< /code >}}

###### Run Producer

Run _produce.js_ to send messages to topic/partition _my-topic/0_ :

{{< code style="light" >}}
$ node src/produce.js --topic my-topic --partition 0
SC server (from profile - default.toml): 127.0.0.1:9003 
Connected to SC:  127.0.0.1:9003
test
ok!
hello world
ok!
bye
ok!
^C
{{< /code >}}

###### Run Consumer

Run _consume.js_ to receive messages from topic/partition _my-topic/0_ :

{{< code style="light" >}}
$ node src/consume.js --topic my-topic --partition 0
SC server (from profile - default.toml): 127.0.0.1:9003 
Connected to SC: 127.0.0.1:9003
test
hello world
bye
^C
{{< /code >}}

The APIs are customizable. Checkout [Node API]({{< relref "../node-api/reference" >}}) for additional information.

{{< links "Related Topics" >}}
* [Topics CLI]({{< relref "topics" >}})
* [Node API]({{< relref "../node-api/reference" >}})
{{< /links >}}