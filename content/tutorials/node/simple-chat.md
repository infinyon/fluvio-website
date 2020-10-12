---
title: Build a Chat Application Using Fluvio Node.js Client
toc: true
---


This guide provides an example of using `@fluvio/client` to write a chat application using Node.js.

## Prerequisites

Before starting on this tutorial, you'll need to have completed the following

- Install [Node.js](#check-nodejs) (**v12.11.0** or above);
- Have the Fluvio CLI installed and have access to a Fluvio cluster. See our [getting started] guide;
- Clone the `fluvio-demo-apps-node` repository;

[getting started]: /docs/getting-started

### Cloning the Repository

```bash
git clone https://github.com/infinyon/fluvio-demo-apps-node.git
```

Once cloned, navigate to the `chat-app` directory by running the following command:

```bash
cd ./fluvio-demo-apps-node/chat-app
```

All commands will be run from the `chat-app` directory in this repository.

## Application Overview

This demo consists of a WebSocket proxy server relaying messages from a React client web application to the `@fluvio/client`, sending `user` and `chat` topic events to the Fluvio cluster.

`@fluvio/client` is currently not supported in the browser, therefore the WebSocket server is required to proxy the client events. An example of this proxy service can be viewed at [`./chat-server/src/fluvio-lib/dataStream/index.ts`](https://github.com/infinyon/fluvio-demo-apps-node/blob/master/chat-app/chat-server/src/fluvio-lib/dataStreams/index.ts#L26).

Read the [API docs](https://infinyon.github.io/fluvio-client-node/) for more information.
<hr/>
<br/>

## Building the Demo App

From the `chat-app` directory, run `npm run build`. This will run the `./build.sh` script, which will install the build dependencies and build the client and server applications.
<hr/>
<br/>

## Running the Demo App

<br/>

### **Run the Server**

Open a new terminal window, navigate to the `chat-server` directory and start the application:

```bash
cd chat-server && npm run start
```

On a freshly installed cluster with no prior events, you should see the following message if the server successfully started.


```bash
> chat-server@1.0.0 start /Users/ryantate/Projects/InfinyOn/fluvio-demo-apps-node/chat-app/chat-server
> npx ts-node ./src/chat-server.ts

requiring platform specific module
init users ...
Topic 'nsc-user-events' created
Topic 'nsc-chat-events' created
┌─────────┐
│ (index) │
├─────────┤
└─────────┘
Loaded (0) chat messages
...init done
Chat server is running at http://localhost:5050...
```

<br/>

### **Run the Client**

Open a new terminal window, navigate to the `chat-client` directory and start the application:

```bash
cd chat-client && npm run start
```

If everything was installed and built successfully, you should see the following message when starting the application.

Visit the application at [`http://localhost:5051`](http://localhost:5051)

Your network address will be different than the example shown below.

```bash
> chat-client@1.0.0 start /Users/ryantate/Projects/InfinyOn/fluvio-demo-apps-node/chat-app/chat-client
> npx serve ./dist -l 5051 -s

npx: installed 78 in 3.108s

   ┌──────────────────────────────────────────────────┐
   │                                                  │
   │   Serving!                                       │
   │                                                  │
   │   - Local:            http://localhost:5051      │
   │   - On Your Network:  http://192.168.0.24:5051   │
   │                                                  │
   │   Copied local address to clipboard!             │
   │                                                  │
   └──────────────────────────────────────────────────┘


```
<hr/>
<br/>

## Conclusion

With the client and server applications running, you can now visit the client application and test the chap application.