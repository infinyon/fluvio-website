---
title: Websocket Glue for Data Streaming Apps
author: 
    name: "The Fluvio Team"
description: Step-by-step tutorial on how to create a session aware websocket glue for data streaming applications.
date: 2020-12-26
slug: websocket-glue-for-streaming-apps
url: /blog/2020/12/websocket-glue-for-streaming-apps
---

Modern applications need to interact with their users in real-time which require real time data streaming support. While we had Fluvio to handle data streaming, we needed a websocket connection proxy to bridge the data streaming with web applications.

This websocket connection layer should have the following properties:

* establish long-lived websocket connections
* work natively in javascript for easy integration with front end clients
* handle cookies for session identification
* handle session management for server initiated connections
* scale to a large number of concurrent sessions

Since we couldn't find such code we ended up building it ourselves. We used this solution in both of our data streaming apps: Chat App and Bot Assistant.

If you want to jump right into it, you can download it from <a href="https://github.com/infinyon/fluvio-demo-apps-node/tree/master/websocket-glue" target="_blank">github</a>. The rest of this article takes you through a step-by-step on how we've built this solution one component at a time.

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
* [Step 4: Add session support](#step-4-add-session-support)

This blog is part of a series of tutorials that shows the power of combining the [Websocket glue](#) with <a href="https://github.com/infinyon/fluvio">Fluvio</a> to build full featured data streaming apps.

##### Architecture

The goals is to create a session aware client/server websocket mechanism that we can leveraged by any real-time data streaming apps. 

<img src="/blog/images/websocket-glue/architecture.svg"
     alt="WebSocket Architecture"
     style="justify: center; max-width: 440px" />

Let's get started.


## Step 1: Add backend server

The backend websocket server uses `Node.js` and `Typescript`. First we setup the environment, then we implement the websocket connection handler.

### Add Node.js server

Let's create a directory `websocket-glue`:

```bash
mkdir websocket-glue && cd websocket-glue
```

Initialize node (this example uses Node v13.5.0):

```bash
npm init --yes
```

It generates the following `package.json` file:

```bash
{
  "name": "websocket-glue",
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
* **@types/node**: typescript library for `Node.js`.
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
  "name": "websocket-glue",
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

Open a web browser and navigate to `http://localhost:9998/`.

<img src="/blog/images/websocket-glue/frontend.svg"
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

Our websocket implementation is a proxy server that intermediates the communication between clients and the server business logic. As the solution allows the clients or the server to initiate requests, we'll create two separate files **proxy-in** and **proxy-out**. This separation gives us better division of responsibility between incoming and outgoing requests. We'll come back to this in [Step 4](#step-4-add-session-support).

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

<img src="/blog/images/websocket-glue/websocket.svg"
     alt="WebSocket Frontend"
     style="justify: center; max-width: 540px" />

Congratulations, you have a basic websocket glue is ready for use.


## Step 4: Add session support

A websocket servers must be able to support multiple conversations in parallel, where each conversation is uniquely identified by a session id. The preferred method to managed these conversations is through <a href="https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API" target="_blank">HTTP cookies</a>.

Once a websocket session is established both, the client and the server can leverage it to initiate requests. Client initiated requests are straight forward as the session information is passed through header cookies. Server initiated requests are bit more involved as they require a session management layer. 

In this section covers both aspects:
* [Add session cookies for client requests](#add-session-cookies-for-client-requests)
* [Add session management for server requests](#add-session-management-for-server-requests)

### Add session cookies for client requests

Web servers push cookies to clients through HTTP headers. After receipt, the clients attaches the cookies to each subsequent message it sends the server. Messages exchanged using the same cookie are also known as sessions.

Let's updates `src/proxy-in.ts` websocket server to generate and push session cookies: 

{{< highlight typescript "hl_lines=3-5 21-24 33-41 44-45 47 50 54 67-81" >}}
import WS from "ws";
import http from "http";
import crypto from 'crypto';

const COOKIE_NAME = "CookieName"

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
            const session = WsProxyIn.parseCookie(COOKIE_NAME, request.headers.cookie);
            if (session) {
                request.headers.session = session;
            }

            WsProxyIn.wss.handleUpgrade(request, socket, head, function (ws: WS) {
                WsProxyIn.wss.emit("connection", ws, request);
            });
        });
    }

    private onConnection() {
        WsProxyIn.wss.on("headers", (headers: Array<string>, req) => {
            const session = WsProxyIn.parseCookie(COOKIE_NAME, req.headers.cookie);
            if (!session) {
                let session = crypto.randomBytes(20).toString("hex");
                req.headers.session = session;

                headers.push("Set-Cookie: " + COOKIE_NAME + "=" + session);
            }
        });

        WsProxyIn.wss.on("connection", function (ws, req) {
            const session_hdr = req.headers.session;
            const sid = ((Array.isArray(session_hdr)) ? session_hdr[0] : session_hdr) || ""; 

            console.log(`session opened: ${sid}`);

            ws.on("close", function () {
                console.log(`session closed: ${sid}`);
            });

            ws.on("message", (clientMsg: string) => {
                console.log(`<== ${clientMsg} from ${sid}`);

                var response = "ok";
                if (clientMsg == "ping") {
                    response = "pong";
                }

                ws.send(response);
                console.log("==> ", response);
            });
        });
    }

    private static parseCookie(cookieName: string, cookie_hdr?: string) {
        if (cookie_hdr) {
            const cookiePair = cookie_hdr.split(/; */).map((c: string) => {
                const [key, v] = c.split('=', 2);
                return [key, decodeURIComponent(v)];
            }).find(res =>
                (res[0] == cookieName)
            );

            if (Array.isArray(cookiePair) && cookiePair.length > 1) {
                return cookiePair[1];
            }
        }
        return undefined;
    }    
}
{{< /highlight >}}

The code generates and assigns a session id to a cookie with arbitrary name. Then, it utilizes WebSocket headers to send it to the client.

The webSocket connection setup in two steps:
1. `headers` request
2. `connection` request. 

In the `headers` request, the code checks the HTTP headers for the session cookie. If not found, it generates a new session id and appends the result to the HTTP header. WebSocket will take care of the rest. In `connection` callback, the code reads the session id from `request.headers.session`.

Finally, `parseSessionFromCookie` reads the session id from the cookie header and returns the the caller.

### Test session cookies

Open the web browser and refresh `http://localhost:9998/`. Then, open browser cookies and look for an entry called `CookieName`. The cookie stores the session id.

<img src="/blog/images/websocket-glue/webcookies.svg"
     alt="WebSocket Frontend"
     style="justify: center; max-width: 580px" />

If you open a 2nd browser in `Incognito Mode` a new cookie is assigned.

At the terminal the web server logs new connection with the cookie information.

```bash
started websocket server at ws://localhost:9998...
session opened: 8771b27ee1c8af52e8473c2d8f8e3d932a654155
<== hello from 8771b27ee1c8af52e8473c2d8f8e3d932a654155
==>  ok
session opened: 3cd4c1fd800f8adb0b1ca5a5a8cce80ce0335788
<== ping from 3cd4c1fd800f8adb0b1ca5a5a8cce80ce0335788
==>  pong
session closed: 3cd4c1fd800f8adb0b1ca5a5a8cce80ce0335788
session closed: 8771b27ee1c8af52e8473c2d8f8e3d932a654155
```
Congratulations, you have implemented session cookies using websocket.

### Add session management for server requests

Session management is an intermediate file that caches the session id with the websocket connection object. These cached connections allows server modules to send messages to any of the clients based on their session identifier.

#### Add session management file

Let's add the session management to a file called `proxy-out`:

```bash
touch src/proxy-out.ts
```

Paste the following content in the `src/proxy-out.ts` file:

```ts
import WS from "ws";
type SID = string;

export class WsProxyOut {
    private sessions: Map<SID, WS>;

    constructor() {
        this.sessions = new Map();
    }

    public addSession(sid: SID, ws: WS) {
        this.sessions.set(sid, ws);
    }

    public getSession(sid: SID) {
        this.sessions.get(sid);
    }

    public closeSession(sid: SID) {
        const ws = this.sessions.get(sid);
        if (ws) {
            ws.close();
        }
        this.sessions.delete(sid);
    }

    public sendMessage(sid: SID, message: string) {
        const ws = this.sessions.get(sid);
        if (ws) {
            ws.send(message);
        }
    }

    public broadcastMessage(message: string) {
        for (let ws of this.sessions.values()) {
            ws.send(message);
        }
    }

    public show() {
        console.log("Sessions");
        console.table(this.sessions);
    }    
}
```

WsProxyOut class saves session ids (SID) together with the Websocket connection in a Map object. Any module with a reference to this object can send messages to one or all clients.

#### Integrated session management

Sessions are generated inside incoming proxy file, hence the integration is done there. However, the sessions must be shareable with any number of server modules and it needs to be a top level object.

Let's update `src/proxy-in.ts`:

{{< highlight typescript "hl_lines=4 10 12 14 50 54 66" >}}
import WS from "ws";
import http from "http";
import crypto from 'crypto';
import { WsProxyOut } from "./proxy-out";

const COOKIE_NAME = "CookieName"

export class WsProxyIn {
    private static wss: WS.Server;
    private static proxyOut: WsProxyOut

    constructor(proxyOut: WsProxyOut) {
        WsProxyIn.wss = new WS.Server({ clientTracking: false, noServer: true });
        WsProxyIn.proxyOut = proxyOut;
    }

    public init(server: http.Server) {
        this.onUpgrade(server);
        this.onConnection();
    }

    private onUpgrade(server: http.Server) {
        server.on("upgrade", function (request, socket, head) {
            const session = WsProxyIn.parseCookie(COOKIE_NAME, request.headers.cookie);
            if (session) {
                request.headers.session = session;
            }

            WsProxyIn.wss.handleUpgrade(request, socket, head, function (ws: WS) {
                WsProxyIn.wss.emit("connection", ws, request);
            });
        });
    }

    private onConnection() {
        WsProxyIn.wss.on("headers", (headers: Array<string>, req) => {
            const session = WsProxyIn.parseCookie(COOKIE_NAME, req.headers.cookie);
            if (!session) {
                let session = crypto.randomBytes(20).toString("hex");
                req.headers.session = session;

                headers.push("Set-Cookie: " + COOKIE_NAME + "=" + session);
            }
        });

        WsProxyIn.wss.on("connection", function (ws, req) {
            const session_hdr = req.headers.session;
            const sid = ((Array.isArray(session_hdr)) ? session_hdr[0] : session_hdr) || "";

            WsProxyIn.proxyOut.addSession(sid, ws);
            console.log(`session opened: ${sid}`);

            ws.on("close", function () {
                WsProxyIn.proxyOut.closeSession(sid);
                console.log(`session closed: ${sid}`);
            });

            ws.on("message", (clientMsg: string) => {
                console.log(`<== ${clientMsg} from ${sid}`);

                var response = "ok";
                if (clientMsg == "ping") {
                    response = "pong";
                }

                WsProxyIn.proxyOut.sendMessage(sid, response);
                console.log("==> ", response);
            });
        });
    }

    private static parseCookie(cookieName: string, cookie_hdr?: string) {
        if (cookie_hdr) {
            const cookiePair = cookie_hdr.split(/; */).map((c: string) => {
                const [key, v] = c.split('=', 2);
                return [key, decodeURIComponent(v)];
            }).find(res =>
                (res[0] == cookieName)
            );

            if (Array.isArray(cookiePair) && cookiePair.length > 1) {
                return cookiePair[1];
            }
        }
        return undefined;
    }
}
{{< /highlight >}}

The code adds a private static variable `proxyOut` which stores a pointer to the session management object. When the connection status changes, `proxyOut` is notified to update its internal cache. 

OnMessage API was also changed to use `proxyOut` for sending messages using the session id.

Next, we need to update `src\server.ts` to crate the session manager object an pass it to the incoming proxy:

{{< highlight typescript "hl_lines=5 13-14" >}}
import http from "http";
import express from "express";
import path from "path";
import { WsProxyIn } from "./proxy-in";
import { WsProxyOut } from "./proxy-out";

const PORT = 9998;

const startServer = async () => {
    const app = express();
    const Server = http.createServer(app);
    const publicPath = path.join(__dirname, '..', 'public');
    const proxyOut = new WsProxyOut();
    const wsProxyIn = new WsProxyIn(proxyOut);

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

The code create a new `WsProxyOut` object and passes it to `WsProxyIn`.

### Test server initiated requests

To test server initiated requests, we'll add a simple timeout that triggers every 3 seconds and broadcasts a notification to all clients.

We'll temporarily add a trigger to `src\server.ts`:

{{< highlight typescript "hl_lines=18-21" >}}
import http from "http";
import express from "express";
import path from "path";
import { WsProxyIn } from "./proxy-in";
import { WsProxyOut } from "./proxy-out";

const PORT = 9998;

const startServer = async () => {
    const app = express();
    const Server = http.createServer(app);
    const publicPath = path.join(__dirname, '..', 'public');
    const proxyOut = new WsProxyOut();
    const wsProxyIn = new WsProxyIn(proxyOut);

    wsProxyIn.init(Server);

    // test broadcast
    setInterval(() => {
        proxyOut.broadcastMessage("notify");
    }, 3000);

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

Navigate to your browser pointing at `http://localhost:9998/` and connect.
The `notify` message is printed to the output every 3 seconds:

<img src="/blog/images/websocket-glue/websocket-notify.svg"
     alt="WebSocket Frontend"
     style="justify: center; max-width: 540px" />

Congratulations, your session aware websocket glue is ready for use.


## Conclusion

Websocket is a good choice for streaming real-time data between web server clients and backend servers. The blog shows that 220 lines of code is all you need to build your own real-time applications on websocket.

This websocket glue is used in the following sample applications:
* [Build your own custom Bot Assistant](/blog/2020/12/bot-assistant/)
* [Build a persistent Chat App without a database](/blog/2020/12/persistent-chat-app)

You can reach us on <a href="https://discordapp.com/invite/bBG2dTz" target="_blank">discord</a>. We look forward to hearing from you.