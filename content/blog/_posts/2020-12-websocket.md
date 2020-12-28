---
title: Websocket Boilerplate for Data Streaming Apps
author: 
    name: "The Fluvio Team"
description: Step-by-step tutorial on how to create a session aware websocket boilerplate for data streaming applications.
date: 2020-12-26
slug: websocket-boilerplate-for-streaming-apps
url: /blog/2020/12/websocket-boilerplate-for-streaming-apps
---

Modern applications need to interact with their users in real-time. Here at Fluvio, we built several real-time data streaming samples apps, such as Bot Assistant and Chat App, and we found ourselves looking for a websocket boilerplate. We couldn't find one that suited our purpose and we ended up building it ourselves. 

##### Overview

Websocket is a well established client/server communication mechanism for real-time data exchange. Most modern web browsers have built-in websocket libraries ready for use by front-end clients. There are also well established websocket libraries for most programming language which gives us a variety of choices for the backend server. 

This project is using the following technologies:
* <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript" target="_blank">Javascript</a> - for the front end client
* <a href="https://nodejs.org/">Node.js</a> and <a href="https://www.typescriptlang.org/docs/" target="_blank">TypeScript</a> for backend server.
* <a href="https://developer.mozilla.org/en-US/docs/Web/API/WebSocket" target="_blank">WebSocket</a> - for real-time communication

This blog is written in a set-by-step tutorial format:
* [Step 1: Add backend server](#step-1-add-backend-server)
* [Step 2: Add frontend client](#step-2-add-frontend-client)
* [Step 3: Add websocket communication](#step-3-add-websocket-communication)
* [Step 4: Add multi-session support](#step-4-add-multi-session-support)

The project is also available for download in github.

##### Architecture

The goals is to create a simple client/server application that we can use as boilerplate for any number of real-time data streaming apps. 

<img src="/blog/images/websocket/ping-pong.svg"
     alt="WebSocket Ping/Pong"
     style="justify: center; max-width: 440px" />

Let's get started.


## Step 1: Add backend server

The backend websocket server uses `Node.js` and `Typescript`. First we setup the environment, then we implement the websocket connection handler.

### Add Node.js server

Let's create a directory `websocket-boilerplate`:

```bash
mkdir websocket-boilerplate && cd websocket-boilerplate
```

Initialize node (this example uses Node v13.5.0):

```bash
npm init --yes
```

```bash
{
  "name": "websocket-boilerplate",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [],
  "author": "fluvio <admin@fluvio.io> (fluvio.io)",
  "license": "Apache 2.0"
}
```

Install `express`, `typescript` and `ts-node` services:

```bash
npm install typescript ts-node express && \
npm install -D tsc-watch @types/node @types/express
```

* **typescript**: package to add static typing and strict syntactical check to JavaScript.
* **ts-node**: package for using TypeScript with Node.js. For example: `ts-node app.ts`.
* **tsc-watch**: a development tool to restart the server on code changes.
* **express**: web application framework for routing, cookies, and more.
* **@types/node***: typescript library for `Node.js`.
* **@types/express**: typescript library `express` application server.


### Add typescript configuration

Next, create a typescript configuration file:

```bash
touch tsconfig.json
```

Paste the following content in `tsconfig.json` file:

```json
{
	"compilerOptions": {
		"target": "es6",
		"module": "commonjs",
		"lib": [
			"dom",
			"ES2017",
			"ES2015"
		],
		"outDir": "dist",
		"rootDir": "./src",
		"strict": true,
		"moduleResolution": "node",
		"esModuleInterop": true,
	}
}
```

For additional information on the typescript configuration parameters, checkout the documentation <a href="https://www.typescriptlang.org/tsconfig" target="_blank">here</a>.


### Add server.ts file

It's time to add the `src` directory and provision the `server.ts` file:

```bash
mkdir src && touch src/server.ts
```

Paste the following content in `src\server.ts` file:

```ts
import http from "http";
import express from "express";

const PORT = 9998;

const startServer = async () => {
    const app = express();
    const Server = http.createServer(app);

    Server.listen(PORT, () => {
        console.log(
            `started websocket server at ws://localhost:${PORT}...`
        );
    });
};

startServer();
```

In summary, the code:
* creates an express application framework.
* attaches express to the http webserver.
* starts the webserver on port 9998. 


#### Update Node.js configuration

Updated `package.json` configuration file to run `server.ts` file:

{{< highlight json "hl_lines=5 7-8" >}}
{
  "name": "websocket-boilerplate",
  "version": "1.0.0",
  "description": "",
  "main": "server.js",
  "scripts": {
    "start:dev": "tsc-watch --onSuccess \"node ./dist/server.js\"",
    "start": "npx ts-node ./src/server.ts"
  },
  "keywords": [],
  "author": "fluvio <admin@fluvio.io> (fluvio.io)",
  "license": "Apache 2.0",
  "dependencies": {
    "express": "^4.17.1",
    "ts-node": "^9.1.1",
    "typescript": "^4.1.3"
  },
  "devDependencies": {
    "@types/express": "^4.17.9",
    "@types/node": "^14.14.16",
    "tsc-watch": "^4.2.9"
  }
}
{{< /highlight >}}

* update `main` to server.js
* add 2 scripts: `start:dev` for development and `start` for production.

### Test backend server

Let's sanity check the directory hierarchy:

```bash
tree -I 'node_modules|dist'
```

```bash
.
├── package-lock.json
├── package.json
├── src
│   └── server.ts
└── tsconfig.json
```

Run the server:

```bash
npm run start:dev
```

```bash
7:28:31 AM - Starting compilation in watch mode...
7:28:34 AM - Found 0 errors. Watching for file changes.
started websocket server at ws://localhost:9998...
```

Congratulations, the server is up and running.

## Step 2: Add frontend client

The frontend client is a simple html and javascript file that establishes a websocket connection with the server. In this section we'll hook-up the html file to the web server.

### Add index.html file

The webserver we have implemented for the backend will also serve the frontend. We'll add the frontend code in a `public` directory.

Let's create the public directory and add `index.html` file:

```bash
mkdir public && touch public/index.html
```

Paste the following code in `index.html` file:

```html
<html>
    <head>
        <meta charset="utf-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <style>body {background-color:#f4f6f5;} </style>
    </head>

    <body>
        <p>
            <button id="connect">Connect</button> 
            <button id="disconnect">Disconnect</button>
        </p>
        <p>
            <input id="message" type="text" size="20" maxlength="240" placeholder="ping">
            <button id="sendMessage">SendMessage</button> 
        </p>
        <textarea id="output" rows="20" cols="60" readonly></textarea>
    </body>
</html>
```

The code has a standard HTML header with some background color and a body with the following elements:
* `connect` and `disconnect` buttons to connect with websocket server.
* `message` input and `sendMessage` button to send messages to the server.
* `output` textarea to log the progress of the communication exchange.

### Hookup index.html to server

To hook-up `index.html` file we need to update `src\server.ts`:

{{< highlight ts "hl_lines=3 10-14" >}}
import http from "http";
import express from "express";
import path from "path";

const PORT = 9998;

const startServer = async () => {
    const app = express();
    const Server = http.createServer(app);
    const publicPath = path.join(__dirname, '..', 'public');

    app.get('/', (req, res) => {
        res.sendFile(path.join(publicPath, 'index.html'));
    });

    Server.listen(PORT, () => {
        console.log(
            `started websocket server at ws://localhost:${PORT}...`
        );
    });
};

startServer();
{{< /highlight >}}

* cache `publicPath`, the path to the public directory.
* add a route to retrieve `index.html` for the root URL.

-> Note: **tsc-watch** automatically refreshed the server.


### Test frontend client

Open a web browser and navigate to `ws://localhost:9998/`.

<img src="/blog/images/websocket/frontend.svg"
     alt="WebSocket Frontend"
     style="justify: center; max-width: 540px" />

Congrats! Client/server scaffolding is done. Next we'll focus on the websocket communication.


## Step 3: Add websocket communication

In this section, we'll implement the client and the server side of the WebSocket (WS) protocol. Let's start with the server side.

### Implement websocket server

Server side websocket implementation has has multiple steps:
* [Install ws package](#install-ws-package)
* [Add incoming proxy](#add-incoming-proxy)
* [Attach proxy to server](#attach-proxy-to-server)

#### Install `ws` package

In node there is a Websocket package, called `ws`, available for download through **npm**. 

Install `ws` package and the typescript definition file:

```bash
npm install ws && \
npm install -D @types/ws
```

#### Add incoming proxy

Our websocket implementation is a proxy server that intermediates the communication between clients and the server business logic. As both, the the clients and the server, can initiate requests, we'll create two separate files **proxy-in** and **proxy-out**. This separation gives us better division of responsibility between incoming and outgoing requests. 

Incoming proxy is responsible for the websocket negotiation and message handling for incoming requests. Let's add the file:

```bash
touch src/proxy-in.ts
```

Paste the following content in the `src/proxy-in.ts` file:

```ts
import WS from "ws";
import http from "http";

export class WsProxyIn {
    private static wss: WS.Server;

    constructor() {
        WsProxyIn.wss = new WS.Server({ clientTracking: false, noServer: true });
    }

    public init(server: http.Server) {
        this.onUpgrade(server);
        this.onConnection();
    }

    private onUpgrade(server: http.Server) {
        server.on("upgrade", function (request, socket, head) {
            WsProxyIn.wss.handleUpgrade(request, socket, head, function (ws: WS) {
                WsProxyIn.wss.emit("connection", ws, request);
            });
        });
    }

    private onConnection() {
        WsProxyIn.wss.on("connection", function (ws, req) {
            console.log("session opened");

            ws.on("close", function () {
                console.log("session closed");
            });

            ws.on("message", (clientMsg: string) => {
                console.log(`<== ${clientMsg}`);

                var response = "ok";
                if (clientMsg == "ping") {
                    response = "pong";
                }

                ws.send(response);
                console.log("==> ", response);
            });
        });
    }
}
```

In summary, the code:
* accepts new websocket connections in the `upgrade` event.
* emits a `connection` event. 
* captures the `connection` event in a new thread.
* checks the data in the `message` event and replies with `pong` for `ping` and `ok` for everything else.

For addition information on how websocket package works, checkout the documentation in <a href="https://github.com/websockets/ws" target="_blank">github</a>.

#### Attach proxy to server

We'll hook-up the incoming proxy in the `src/server.ts` file:

{{< highlight ts "hl_lines=4 12-14" >}}
import http from "http";
import express from "express";
import path from "path";
import { WsProxyIn } from "./proxy-in";

const PORT = 9998;

const startServer = async () => {
    const app = express();
    const Server = http.createServer(app);
    const publicPath = path.join(__dirname, '..', 'public');
    const wsProxyIn = new WsProxyIn();

    wsProxyIn.init(Server);

    app.get('/', (req, res) => {
        res.sendFile(path.join(publicPath, 'index.html'));
    });

    Server.listen(PORT, () => {
        console.log(
            `started websocket server at ws://localhost:${PORT}...`
        );
    });
};

startServer();
{{< /highlight >}}

The code imports and initializes the incoming proxy with the websocket server instance.


### Implement websocket client

The client utilizes the <a href="https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API" target="_blank">Websocket API</a> available in most modern web browsers. 

Client side websocket implementation has several steps:
* [Add websocket script file](#add-script-file)
* [Load script to html file](#load-script-to-html-file)
* [Add scripts route to server](#add-scripts-route-to-serve)


#### Add websocket script file

The script implements websocket connection and messages exchanges. It also attaches the DOM buttons we defined in the `index.html` file.

In a terminal, create the `ws-script.js` file:

```bash
mkdir public/scripts/
touch public/scripts/ws-script.js
```

Paste the following code in `public/scripts/ws-script.js`:

```javascript
const SERVER_HOST = "localhost:9998";

window.onload = () => {
    var webSocket = null;

    function init() {
        var connectBtn = document.getElementById("connect");
        var disconnectBtn = document.getElementById("disconnect");
        var sendMessageBtn = document.getElementById("sendMessage");
        var messageEditor = document.getElementById("message");

        connectBtn.onclick = onConnectClick;
        disconnectBtn.onclick = onDisconnectClick;
        sendMessageBtn.onclick = onSendMessageClick;

        messageEditor.addEventListener("keydown", onEditorKeys, false);
    }

    function openWSConnection() {
        try {
            if (webSocket != null) {
                return;
            }

            const server = "ws://" + SERVER_HOST;
            logOutput("Connecting to: " + server);

            webSocket = new WebSocket(server);

            webSocket.onopen = function (openEvent) {
                logOutput("Connected!");
            };

            webSocket.onclose = function (closeEvent) {
                webSocket = null;
                logOutput("Disconnected!");
            };

            webSocket.onerror = function (errorEvent) {
                logOutput(`Error: ${JSON.stringify(errorEvent)}`);
            };

            webSocket.onmessage = function (messageEvent) {
                var serverMsg = messageEvent.data;
                logOutput(`<== ${serverMsg}`);
            };

        } catch (exception) {
            logOutput(`error: ${JSON.stringify(exception)}`);
        }
    }

    function onConnectClick() {
        openWSConnection();
    }

    function onDisconnectClick() {
        if (webSocket) {
            webSocket.close();
        } else {
            logOutput(`Not Connected!`);
        }
    }

    function onSendMessageClick() {
        if (webSocket) {
            var message = document.getElementById("message").value || "ping";
            logOutput("==> " + message);
            webSocket.send(message);
        } else {
            logOutput(`Not Connected!`);
        }
    }

    function onEditorKeys(e) {
        if (e.code == 'Enter') {
            e.preventDefault();
            onSendMessageClick();
        }
    }

    function logOutput(value) {
        var debugOutput = document.getElementById("output");
        if (debugOutput) {
            debugOutput.value += value + "\n\n";
            debugOutput.scrollTop = debugOutput.scrollHeight;
        }
        console.log(value);
    }

    init();
};
```

The code performs the following operations:
* **init**: attaches DOM objects to their handlers.
* **openWSConnection**: uses WebSocket API to initiate a websocket connection. It also registers callback handlers for websocket operations.
* **onConnectClick**: click handler to open a websocket connection.
* **onDisconnectClick**: click handler to close the websocket connection.
* **onSendMessageClick**: click handler to send message on websocket connection.
* **onEditorKeys**: Captures the enter key invoke send message.
* **logOutput**: Prints messages in the textarea.


#### Load script to html file

Next, we'll update `index.html` to load the `ws-script.js`:

{{< highlight html "hl_lines=8" >}}
<!DOCTYPE HTML>
<html>
    <head>
        <meta charset="utf-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <style>body {background-color:#f4f6f5;} </style>
        <script type = "text/javascript" src="scripts/ws-script.js"></script>
    </head>

    <body>
        <p>
            <button id="connect">Connect</button> 
            <button id="disconnect">Disconnect</button>
        </p>
        <p>
            <input id="message" type="text" size="20" maxlength="240" placeholder="ping">
            <button id="sendMessage">SendMessage</button> 
        </p>
        <textarea id="output" rows="20" cols="60" readonly></textarea>
    </body>
</html>
{{< /highlight >}}

The code attaches a script tag that loads `ws-script.js` file to the HTML headers.


#### Add scripts route to serve

The script is not yet visible to your web server. Let's add a route to the web server `src/server.ts` file:

{{< highlight ts "hl_lines=19" >}}
import http from "http";
import express from "express";
import path from "path";
import { WsProxyIn } from "./proxy-in";

const PORT = 9998;

const startServer = async () => {
    const app = express();
    const Server = http.createServer(app);
    const publicPath = path.join(__dirname, '..', 'public');
    const wsProxyIn = new WsProxyIn();

    wsProxyIn.init(Server);

    app.get('/', (req, res) => {
        res.sendFile(path.join(publicPath, 'index.html'));
    });
    app.use("/scripts", express.static(path.join(publicPath, 'scripts')));

    Server.listen(PORT, () => {
        console.log(
            `started websocket server at ws://localhost:${PORT}...`
        );
    });
};

startServer();
{{< /highlight >}}

The code defines a route that expose the `/scripts` directory to the web server.


### Test websocket communication

Open the web browser and refresh `http://localhost:9998/` to load the latest javascript code. You should see the same page as before, but the buttons should now be operational. Let's establish a a connection and send a message:

* click `Connect` should connect to the websocket server.
* click `Send Message` should send 'ping' to server.
    * server should respond with 'pong'
* type 'hello word' and click `Send Message` again.
    * server should respond with 'ok'.
* click `Disconnect` to release websocket connection.

<img src="/blog/images/websocket/websocket.svg"
     alt="WebSocket Frontend"
     style="justify: center; max-width: 540px" />

Congratulations, your websocket boilerplate is ready for use.


## Step 4: Add multi-session support

<br/><br/><br/><br/><br/>


#### Implement outgoing proxy

Outgoing proxy is responsible for the outgoing message, which is just a simple API.

```bash
touch src/proxy-in.ts
```

Paste the following content in the `src/proxy-out.ts` file:

```ts
import WS from "ws";

export class WsProxyOut {

    public sendMessage(ws: WS, message: string) {
        const ws = this.sessions.get(sid);
        if (ws) {
            ws.send(message);
        }
    }

}

```