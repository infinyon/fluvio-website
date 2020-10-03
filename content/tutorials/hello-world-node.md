---
title: '"Hello, World! 🎉" in Node.js + TypeScript'
hidden: true
group: hello-world
tag: node
weight: 10
toc: true
---

{{< lang-selector >}}

In this guide we’ll provide instructions on how to set up a Fluvio <a href="https://nodejs.org" target="_blank">Node.js</a> environment and build a simple data streaming App using the [Fluvio NodeJS client](https://www.npmjs.com/package/@fluvio/client).

{{< idea >}}

**Prerequisites:** Examples in this section require an existing Fluvio cluster.
<br />
<br /> Step-by-step instructions to install a cluster are available in the [Getting Started](/docs/getting-started/) guide. 

* [Create a local Fluvio cluster](/docs/getting-started/install-local/)

{{< /idea >}}

## Check Node.js

A Fluvio environment for Node requires Node.js **version 13** or above.

#### Install Node.js

Node.js installation varies depending on your operating system.

|   Operating System     |         Instructions           |
|------------------------|--------------------------------|
| MacOS                  | Use the official installer from <a href="https://nodejs.org" target="_blank">Node.js</a> to install on **macOS**.  |
| Windows                | Use the official installer from <a href="https://nodejs.org" target="_blank">Node.js</a> to install on **Windows**. |
| Linux                  | Use the instructions provided by your **Linux** package manager. <br/> Node.js maintains a list of <a href="https://nodejs.org/en/download/package-manager" target="_blank">supported packages</a>.  |



## Application Development

This section provides a step-by-step on how to build a simple data streaming app using Node.js. If you'd like to download the demo app instead, skip ahead to [Download Fluvio Demo Apps](#download-node-demo-apps).


### Initialize a New Node.js Project

Create a directory for **fluvio-node-app**:

```bash
$ mkdir fluvio-node-app
$ cd fluvio-node-app
```

Run npm to create a **node project** and generate package.json file:

```bash
$ npm init -y
```

### Updating the Project for TypeScript

The Fluvio Node.js client provides TypeScript definitions. Let's setup our demo application to use TypeScript.

```bash
# Install TypeScript locally to the project as a development dependency;
$ npm i typescript -D
```

### Create a Source Directory

Create a `src/` directory where we will write our TypeScript files.

```bash
# Create a `src/` directory
mkdir src

# Create an index.ts file
touch ./src/index.ts
```

### Add Fluvio Client library

The client library exports TypeScript definitions and Node.js bindings to the native Fluvio client.

Use npm install to add <a href="https://www.npmjs.com/package/@fluvio/client" target="_blank">@fluvio/client</a> to the project:

```bash
$ npm install @fluvio/client --save
```
Check to make sure the `@fluvio/client` module is added to package.json. 

### Update `package.json` Scripts and Main Entry File

```json
{
  "name": "fluvio-node-app",
  "version": "1.0.0",
  "description": "",
  "main": "./dist/index.js",
  "types": "./dist/index.d.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1",
    "build": "npx tsc ./src/index.ts --outDir ./dist",
    "producer": "node ./bin/index.js -- producer",
    "consumer": "node ./bin/index.js -- consumer"
  },
  "keywords": [],
  "author": "",
  "license": "ISC",
  "devDependencies": {
    "@types/node": "^14.11.2",
    "typescript": "^4.0.3"
  },
  "dependencies": {
    "@fluvio/client": "^5.0.0"
  }
}
```

{{< idea >}}

Your directory should now contain the following:

```bash
$ tree . -L 2
.
├── node_modules
│   ├── @fluvio
│   ├── @types
│   └── typescript
├── package-lock.json
├── package.json
├── src
│   └── index.ts
└── tsconfig.json

5 directories, 4 files
```

{{< /idea >}}


### Build "Hello, World!" Streaming Terminal App

We're going to write a small application, `StreamingApp`, that we will use to build a streaming interface from the terminal's standard input.

The following changes are made to the _./src/index.ts_ file. [TL;DR](#the-final-streamingapp-file)

### Importing Dependencies

```TypeScript
// Import Fluvio class and library resources;
import Fluvio, { FluvioAdmin, Options, TopicProducer, PartitionConsumer } from "@fluvio/client";

// Use Node.js `readline` module to read from command line;
import { createInterface, Interface } from "readline";


```

### Create the `StreamingApp` Class

```TypeScript

// Create a StreamingApp Class to encapsulate our logic;
export default class StreamingApp {
  // The StreamingApp class has a `fluvio` client property
  // to produce and consumer topics;
  fluvio: Fluvio;

  // The StreamingApp class has an `admin` FluvioAdmin
  // to manage topics;
  admin?: FluvioAdmin;

  // Use Node.js' `readline` to relay messages to the topic producer
  // from the terminal;
  rl: Interface;

  // Setup a topic producer;
  producer?: TopicProducer;

  // Setup a topic consumer;
  consumer?: PartitionConsumer;

  constructor(options: { fluvio: Options }) {
    // Create a new Fluvio Client;
    this.fluvio = new Fluvio(options.fluvio);

    // Setup readline
    this.rl = createInterface({
      input: process.stdin,
      output: process.stdout,
    });
  }
}

```

### Connect to the Fluvio Cluster & Setup Topic

```TypeScript
// Create connection to cluster and setup fluvio clients;
  private async configure(
    topicName: string,
    partition?: number,
    isProducer?: boolean
  ) {
    console.log("Connecting to Fluvio Cluster");
    // Setup the connection for the fluvio client;
    await this.fluvio.connect();

    if (isProducer) {
      console.log("Configuring Streaming App for Producer");
      // Create fluvio admin client to create a new topic;
      this.admin = await this.fluvio.admin();
      const foundTopic = await this.admin.findTopic(topicName);

      if (!foundTopic) {
        await this.admin?.createTopic(topicName);
      }

      // Setup a producer for the topic;
      this.producer = await this.fluvio.topicProducer(topicName);
    } else {
      console.log("Configuring Streaming App for Consumer");
      // Setup a consumer for the topic;
      this.consumer = await this.fluvio.partitionConsumer(
        topicName,
        partition || 0
      );
    }
  }
```

### Relay Terminal Standard Input to Fluvio Topic Producer

```TypeScript
  // send a message from the command line to the producer
  // Wait for the consumer to listen for the message;
  // Paste Inside StreamingApp {} class
  private async sendMessage(partition?: number) {
    console.log(
      "Started Fluvio Producer\n\nStart Typing Your Message in the Terminal\n\n>"
    );
    this.rl.on("line", async (input: string) => {
      // Send the standard input to fluvio's Topic Producer;
      await this.producer?.sendRecord(input, partition || 0);
    });
  }
```

### Write a Topic Consumer to Listen for Streaming Messages

```TypeScript
  // Paste Inside StreamingApp {} class
  private async listen() {
    console.log("Started Fluvio Consumer");
    // Setup a streaming consumer for Fluvio;
    await this.consumer?.stream(
      {
        // Don't skip any messages;
        index: 0,
        // Retrieve all messages from the beginning of the topic;
        from: OffsetFrom.Beginning,
      },
      async (msg: string) => {
        console.log(`Received Message:\n\n${msg}`);
      }
    );
  }
```

### Create a Public `run()` Entry Point

```TypeScript
  // Paste Inside StreamingApp {} class
  public async run(topicName: string, partition?: number) {
    const cmd = process.argv[3];
    switch (cmd) {
      case "producer":
        // Configure for the producer;
        await this.configure(topicName, partition, true);
        return await this.sendMessage();
      case "consumer":
        // Configure for the consumer;
        await this.configure(topicName, partition);
        return await this.listen();
      default:
        console.log("Unknown command: ", cmd);
        return;
    }
  }
```

### The Final `StreamingApp` File

The final _./src/index.ts_ file should look like the following;

```TypeScript
// Import Fluvio
import Fluvio, {
  FluvioAdmin,
  OffsetFrom,
  Options,
  PartitionConsumer,
  TopicProducer,
  Topic,
} from "@fluvio/client";

// Use Node.js `readline` module to read from command line;
// Ensure
import { createInterface, Interface } from "readline";

// Create a StreamingApp Class to encapsulate our logic;
export default class StreamingApp {
  // The StreamingApp class has a `fluvio` client property
  // to produce and consumer topics;
  fluvio: Fluvio;

  // The StreamingApp class has an `admin` FluvioAdmin
  // to manage topics;
  admin?: FluvioAdmin;

  // Use Node.js' `readline` to relay messages to the topic producer
  // from the terminal;
  rl: Interface;

  // Setup a topic producer;
  producer?: TopicProducer;

  // Setup a topic consumer;
  consumer?: PartitionConsumer;

  constructor(options: { fluvio: Options }) {
    // Create a new Fluvio Client;
    this.fluvio = new Fluvio(options.fluvio);

    // Setup readline
    this.rl = createInterface({
      input: process.stdin,
      output: process.stdout,
    });
  }

  // Create connection to cluster and setup fluvio clients;
  private async configure(
    topicName: string,
    partition?: number,
    isProducer?: boolean
  ) {
    console.log("Connecting to Fluvio Cluster");
    // Setup the connection for the fluvio client;
    await this.fluvio.connect();

    if (isProducer) {
      console.log("Configuring Streaming App for Producer");
      // Create fluvio admin client to create a new topic;
      this.admin = await this.fluvio.admin();
      const foundTopic = await this.admin.findTopic(topicName);

      if (!foundTopic) {
        await this.admin?.createTopic(topicName);
      }

      // Setup a producer for the topic;
      this.producer = await this.fluvio.topicProducer(topicName);
    } else {
      console.log("Configuring Streaming App for Consumer");
      // Setup a consumer for the topic;
      this.consumer = await this.fluvio.partitionConsumer(
        topicName,
        partition || 0
      );
    }
  }

  // send a message from the command line to the producer
  // Wait for the consumer to listen for the message;
  private async sendMessage(partition?: number) {
    console.log(
      "Started Fluvio Producer\n\nStart Typing Your Message in the Terminal\n\n>"
    );
    this.rl.on("line", async (input: string) => {
      await this.producer?.sendRecord(input, partition || 0);
    });
  }

  private async listen() {
    console.log("Started Fluvio Consumer");
    await this.consumer?.stream(
      {
        index: 0,
        from: OffsetFrom.Beginning,
      },
      async (msg: string) => {
        console.log(`Received Message:\n\n${msg}`);
      }
    );
  }

  public async run(topicName: string, partition?: number) {
    const cmd = process.argv[3];
    switch (cmd) {
      case "producer":
        // Configure for the producer;
        await this.configure(topicName, partition, true);
        return await this.sendMessage();
      case "consumer":
        // Configure for the consumer;
        await this.configure(topicName, partition);
        return await this.listen();
      default:
        console.log("Unknown command: ", cmd);
        return;
    }
  }
}

```

### Using the Streaming Terminal App;

Create a directory `bin/` and a file `./bin/index.js` where we will write the following:

```js
const StreamingApp = require("../dist").default;

// Use the newly created StreamingApp class;
const app = new StreamingApp({
  fluvio: {
    host: "127.0.0.1",
    port: 9003,
  },
});

const TOPIC_NAME = "my-topic";

// Configure the application for topic;
app.run(TOPIC_NAME).catch((error) => {
  console.log(`Streaming App Exited with: ${error}`);
});

```

## Running the Demo

Now that the code is written, we're ready to run our `Hello, World!` example. Run the following commands in separate terminals.

Terminal 1
```bash
# Run in first terminal
$ npm run producer
```

```bash
Connecting to Fluvio Cluster
Configuring Streaming App for Producer
Started Fluvio Producer

Start Typing Your Message in the Terminal

> Hello, World! 🎉
```

<br/>
<hr/>

Terminal 2
```bash
# Run in second terminal
$ npm run consumer
```

```bash
Connecting to Fluvio Cluster
Configuring Streaming App for Consumer
Started Fluvio Consumer

> Received Message:

Hello, World! 🎉
```

# Read the `@fluvio/client` Docs

Checkout [Node API](/docs/node-api/reference) for additional information.

#### Related Topics
-------------------
* [Topics CLI](/docs/cli/topics)
* [Node API](/docs/node-api)
