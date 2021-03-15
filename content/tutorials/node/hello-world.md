---
title: '"Hello, World! 🎉" in Node.js + TypeScript'
toc: true
---
{{< lang-selector >}}

In this tutorial, you will learn how to create a topic, build a producer/consumer in Typescript, and sends a "Hello, World! 🎉" message.

## Prerequisites

Before starting on this tutorial, you'll need to have completed the following

- Install [Node.js](#check-nodejs) (**v12.11.0** or above) 
- Have the Fluvio CLI (version  `0.7.0` or greater) installed <sup>[1]</sup>
- Have access to a Fluvio cluster.

See our [getting started] guide for more details on getting set up.

[getting started]: /docs/getting-started

-> [1]: If you need to, you can update the Fluvio CLI by using `fluvio update`.

### Create a Topic using the Fluvio CLI

In Fluvio, we send all of our messages to something called a Topic, which
is like a category for related messages. For this tutorial, we'll create
a topic called `hello-fluvio` using the following command:

```bash
$ fluvio topic create hello-fluvio
```

### Check Node.js

A Fluvio environment for Node requires Node.js **v12.11.0** or above.

#### Install Node.js

Node.js installation varies depending on your operating system.

|   Operating System     |         Instructions           |
|------------------------|--------------------------------|
| MacOS                  | Use the official installer from <a href="https://nodejs.org" target="_blank">Node.js</a> to install on **macOS**.  |
| Linux                  | Use the instructions provided by your **Linux** package manager. <br/> Node.js maintains a list of <a href="https://nodejs.org/en/download/package-manager" target="_blank">supported packages</a>.  |

## Writing the Application

The following sections will setup your project and walk through writing the application files.

### Installing Project Dependencies

Run the following commands to set up your project for development:

```bash
$ mkdir fluvio-demo
$ cd fluvio-demo
$ npm init -y
$ npm install typescript ts-node @types/node -D
$ npm install @fluvio/client -S
$ touch producer.ts consumer.ts
```

Your working directory should now contain the following files:

```bash
$ tree -L 1
.
├── consumer.ts
├── node_modules
├── package-lock.json
├── package.json
└── producer.ts

1 directory, 4 files

```

### Writing the `producer.ts` File

Write the following code in your `producer.ts` file.

```TypeScript
import Fluvio from "@fluvio/client";
import { createInterface } from "readline";

// Create Fluvio Client Instance
const fluvio = new Fluvio();

// Create Readline Instance
const rl = createInterface({
  input: process.stdin,
  output: process.stdout,
});

const produce = async () => {
    // Connect the fluvio cluster;
    await fluvio.connect();

    // Create a topic producer;
    const producer = await fluvio.topicProducer("hello-fluvio");
    console.log("Fluvio Producer created, waiting for input:\n\n>")
    // Relay terminal input to fluvio topic producer;
    rl.on("line", async (input) => {
        await producer.send("line", input);
    });
};

produce();
```

##### This code performs the following actions:

- _Import `@fluvio/client` and Node.js' `readline` modules;_
- _Create a new Fluvio Client Instance;_
- _Create a connection to a local Fluvio Cluster;_
- _Create a new topic producer for `hello-fluvio`;_
- _Listen for input typed into the terminal;_
- _Send typed input to the fluvio cluster;_


### Writing the `consumer.ts` File

Write the following code in your `consumer.ts` file.

```TypeScript
import Fluvio, { Offset } from "@fluvio/client";

// Create Fluvio Client Instance
const fluvio = new Fluvio();

const consume = async () => {
    // Connect the fluvio cluster;
    await fluvio.connect();

    const partition = 0;

    // Create Topic Consumer
    const consumer = await fluvio.partitionConsumer("hello-fluvio", partition);
    console.log("Fluvio Consumer created, listening for events:\n\n")
    const stream = await consumer.createStream(Offset.FromBeginning());
    
    for await (const record of stream) {
        const key = record.keyString();
        const value = record.valueString();
        console.log(`Received record: Key=${key}, value=${value}`);
    }
};

consume();
```

##### This code performs the following actions:

- _Import `@fluvio/client` module;_
- _Create a new Fluvio Client Instance;_
- _Create a connection to a local Fluvio Cluster;_
- _Create a new topic consumer for `hello-fluvio`;_
- _Listen for events sent by a topic producer;_

## Running the Demo

Now that the code is written, we're ready to run our `Hello, World! 🎉` example. Run the following commands in separate terminals.

### Running the Producer

Run the following command in the working directory:

```bash
$ npx ts-node ./producer.ts
```

```bash
Fluvio Producer created, waiting for input:

>
```

Great! Now type `Hello, World! 🎉` into your terminal window:

```bash
Hello, World! 🎉
```

<br/>
<hr/>

### Running the Consumer

Open a new terminal and run the following command:

```bash
$ npx ts-node ./consumer.ts
```

```bash
Fluvio Consumer created, listening for events:

Received record: Key=line, value=Hello, World! 🎉
```

## Congratulations!

You've now completed the Fluvio "Hello, World! 🎉" tutorial. 

Head over to the Fluvio Node documentation to learn more about the library and available options.

## Read the `@fluvio/client` Docs

Checkout <a href="https://infinyon.github.io/fluvio-client-node/" target="_blank">Node API</a> reference guide for additional usage information and documentation.
