---
title: Build a Chat Application Using Fluvio Node.js Client
toc: true
---


This guide provides an example of using `@fluvio/client` to write a chat application using Node.js.

## Prerequisites

Before starting on this tutorial, you'll need to have completed the following

- Install [Node.js](#check-nodejs) (**v13** or above).
- Have the Fluvio CLI installed and have access to a Fluvio cluster. See [getting started] guide.
- Clone the `fluvio-demo-apps-node` repository.

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

This demo consists of a WebSocket proxy server relaying messages from a React client web application to the `@fluvio/client`, sending `user`, `chat`, and `session` messages to the Fluvio cluster in real-time.

{{< image src="tutorials/chat-app/simple-chat.svg" alt="register" justify="center" width="780" type="scaled-98">}}

The node API reference guide is available [here](https://infinyon.github.io/fluvio-client-node/).

Checkout our the article [Build a Persistent Chat App without a Database](2020/12/persistent-chat-app/) if you'd like to build the chat app one step at a time.

## Running the Demo App

In a terminal window, from `chat-app` directory, run the following command:

```bash
npm run build
```

This will run the `./build.sh` script, which will install the build dependencies and build the client and server applications.

### **Run the Server**

In the terminal window, from `chat-app` directory, run the following command:

```bash
cd chat-server && npm run setup && npm run start
```

The script `npm run setup` will provision `fluvio` topics. 

On a freshly installed cluster with no prior events, you should see the following message if the server successfully started.


```bash
> chat-server@1.0.0 setup /fluvio-demo-apps-node/chat-app/chat-server
> sh ./setup.sh

topic "chat-app-users" created
topic "chat-app-messages" created
topic "chat-app-sessions" created

> chat-server@1.0.0 start /fluvio-demo-apps-node/chat-app/chat-server
> npx ts-node ./src/chat-server.ts

requiring platform specific module
Users
┌─────────┐
│ (index) │
├─────────┤
└─────────┘
ChatMessages
┌──────────┬────────┐
│ (index)  │ Values │
├──────────┼────────┤
│ messages │   0    │
└──────────┴────────┘
----
Chat server is running at http://localhost:5050...
```

### **Run the Client**

Open a new terminal window, navigate to the `chat-app` directory, and start the application:

```bash
cd chat-client && npm run start:dev
```

If everything was installed and built successfully, you should see the following message when starting the application.
If everything was installed and built successfully, you should see the following message when starting the application.

```bash
...
Child HtmlWebpackCompiler:
     1 asset
    Entrypoint HtmlWebpackPlugin_0 = __child-HtmlWebpackPlugin_0
    [./node_modules/html-webpack-plugin/lib/loader.js!./public/index.html] 989 bytes {HtmlWebpackPlugin_0} [built]
ℹ ｢wdm｣: Compiled successfully.
```

Open the web browser at `http://localhost:5051/` and we are ready to use the App.

## Using the Application

With the client and server applications running, you can now visit the client website and test the chap application.

### Registering a Chat User

When you visit the client application, you need to register a new user. Click the [`Register`](http://localhost:5051/register) button
to create a new chat user. In a full application, you'd also have a password but this is just a tutorial.

{{< image src="tutorials/chat-app/register.svg" alt="register" justify="center" width="360" type="scaled-75">}}

After the user has been created, enter the username in the [`login`](http://localhost:5051/login) form.

{{< image src="tutorials/chat-app/login.svg" alt="Login" justify="center" width="360" type="scaled-75">}}

Once logged in, you are now able to send a message in the chat room.

{{< image src="tutorials/chat-app/chat.svg" alt="chat" justify="center" width="860" type="scaled-99">}}

### Adding More Users

To add more users to the application, open the application in another browser window or use chrome's incognito mode. This will create a fresh local storage for testing the application.

Follow the same steps as above in the newly opened browser window, but this time change your username.

Once you have added more users to the chat window, you can test sending messages between the users by typing in each of the browser windows.


## Conclusion

Congratulations! You've completed the chat-app example and now have an example for using `@fluvio/client` in an application.

Read the [API docs](https://infinyon.github.io/fluvio-client-node/) for more information on how to use `@fluvio/client` for your project.
