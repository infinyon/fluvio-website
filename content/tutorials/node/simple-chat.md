---
title: Build a Chat Application Using Fluvio Node.js Client
toc: true
---


This guide provides an example of using `@fluvio/client` to write a chat application using Node.js. 

<!--
To build the chat app one step at a time, checkout our blog:
* [Build a Persistent Chat App without a Database](2020/12/persistent-chat-app/).
-->

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

With the client and server applications running, you can now visit the client website and test the chat application.

### Registering a Chat User

When you visit the client application, you need to register a new user. Click the [`Register`](http://localhost:5051/register) button
to create a new chat user. In a full application, you'd also have a password but this is just a tutorial.

{{< image src="tutorials/chat-app/register.svg" alt="register" justify="center" width="360" type="scaled-75">}}

After the user has been created, enter the username in the [`Login`](http://localhost:5051/login) dialog.

{{< image src="tutorials/chat-app/login.svg" alt="Login" justify="center" width="360" type="scaled-75">}}

Once logged in, you are now able to send a message in the chat room.

{{< image src="tutorials/chat-app/chat.svg" alt="chat" justify="center" width="860" type="scaled-99">}}

### Adding More Users

To add more users to the application, open the application in another browser window or use chrome's incognito mode. This will create a fresh local storage for testing the application.

Follow the same steps as above in the newly opened browser window, but this time change your username.

Once you have added more users to the chat window, you can test sending messages between the users by typing in each of the browser windows.

## Fluvio Data Streaming

Fluvio stores messages/events in three different topics:
* **chat-app-users**
* **chat-app-sessions**
* **chat-app-messages**

While Chat app uses these events for persistence, they are also available for other purpose. For example if another service is interested in user logins, the service can hook-up a consumer and receive login events in real-time.

### User Events

User events are generated by registration, login, and logout:

* **AddUser** - generated by registration.
* **AddToken** - generated by login.
* **RemoveToken** - generated by logout.
* **RemoveUser** - generated by unregistration.

When a user registers, the server assigns him a color code; when he logs-in, the server sends an authorization tokens.

User messages can be viewed in Fluvio:

```bash
fluvio consume chat-app-users -B
```

Returns all messages in the users topic:

```json
{"kind":"AddUser","content":{"user":"alice","colorCode":"lime"}}
{"kind":"AddToken","content":{"user":"alice","token":"7b0164dbde7c92a0d9d9cf9eafe7e4a63ce0bb3c"}}
{"kind":"AddUser","content":{"user":"bob","colorCode":"purple"}}
{"kind":"AddToken","content":{"user":"bob","token":"0737d893bc368fb8e02a88bd1479287b75dc7082"}}
{"kind":"RemoveToken","content":{"user":"jack","token":"03d492c63e44c3c61829dffb89e19cf5b34432df"}}
```

### Session Events

Chat App supports multiple sessions for the same user. For instance the user may login at the same time from a web server and a mobile device. User sessions capture the beginning of the session and the online/offline state.

A user is marked `online` when it is connected on at least one device and `offline` when it is disconnected from all devices.

Session messages can be viewed in Fluvio:

```bash
 fluvio consume chat-app-sessions -B
```

Returns all messages in the sessions topic:

```json
{"kind":"Started","content":{"user":"alice","sid":"c41e59f83f713fdcf25affabf458a594"}}
{"kind":"UserOnline","content":{"user":"alice"}}
{"kind":"Started","content":{"user":"bob","sid":"6a9a7ec4831e8291f78aec8c6e2b738c"}}
{"kind":"UserOnline","content":{"user":"bob"}}
{"kind":"Started","content":{"user":"charlie","sid":"5d4973b4bf39e182020174daecca1459"}}
{"kind":"UserOnline","content":{"user":"charlie"}}
```

### Chat Messages

Chat messages store all user messages published by all users.

```bash
fluvio consume chat-app-messages -B
```

Returns all messages in the messages topic:

```json
{"sid":"5d4973b4bf39e182020174daecca1459","message":{"user":"charlie","message":"Hey guys","timestamp":"2021-01-01T11:04:16.792"}}
{"sid":"c41e59f83f713fdcf25affabf458a594","message":{"user":"alice","message":"What's going on charlie?","timestamp":"2021-01-01T11:04:40.161"}}
{"sid":"c41e59f83f713fdcf25affabf458a594","message":{"user":"charlie","message":"Not much. Just pretty excited about using fluvio!","timestamp":"2021-01-01T11:05:15.755"}}
```

Each messages has a user name, a messages and the timestamp.

## Conclusion

Fluvio data streaming platform can turn simple prototypes into powerful event drive applications. In this example, we used Fluvio for persistence and inter service communication. When services communicate with each other in real-time, it opens the door for a new class of services beyond the capability of any database.


For additional information, checkout the following related articles:
<!-- 
* [Build a Persistent Chat App without a Database](2020/12/persistent-chat-app/)
-->
* [Node API documentation](https://infinyon.github.io/fluvio-client-node/)