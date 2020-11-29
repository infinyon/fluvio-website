---
title: Build a Custom Bot Assistant
author: 
    name: "The Fluvio Team"
description: Leverage real-time data streaming to build your custom robot assistant 
date: 2020-11-23
slug: bot-assistant
url: /blog/2020/11/bot-assistant
---

Robot assistants are nothing new, they have been around for a while with many full featured products available for use. While these products fit most use cases, there are time when you want to roll out your own.

A custom bot assistant gives you full control over and workflows and the services for your customer interactions. When used in conjunction with a data streaming product such as Fluvio, you gain persistence, resiliency, and scalability.

<img src="/blog/images/bot-assistant/bot-assistant.png"
     alt="Bot Assistant Example"
     style="justify: center; max-width: 320px" />

The project is also available for download in <a href="https://github.com/infinyon/fluvio-demo-apps-node/tree/master/bot-assistant" target="_blank">github</a>.

#### Overview 

This blog will describe how to build a bot assistant one step at a time. In summary:

* [Step 1: Add frontend client](#step-1-add-frontend-client)
* [Step 2: Add backend server](#step-2-add-backend-server)
* [Step 3: Add WebSocket communication](#step-3-add-websocket-communication)
* [Step 4: Add session cookies](#step-4-add-session-cookies)
* [Step 5: Add state machine](#step-5-add-state-machine)
* [Step 6: Add workflow manager](#step-6-add-workflow-manager)
* Data streaming and Persistency

The implementation uses the following software components:

* <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript" target="_blank">Javascript</a> - to develop front end development
* <a href="https://www.typescriptlang.org/docs/" target="_blank">TypeScript</a> - to build the backend
* <a href="https://nodejs.org/">Node.js</a> - to run the web server
* <a href="https://developer.mozilla.org/en-US/docs/Web/API/WebSocket" target="_blank">WebSocket</a> - for real-time communication
* <a href="https://fluvio.io" target="_blank">Fluvio</a> - for data streaming and persistency


## Step 1: Add frontend client

The frontend components and navigation are straight forward. We need two HTML elements:
* `Bot` button
* `Bot Assistant` dialog box

The `Bot` button is displayed on the lower right-hand side of the screen that opens the `Bot Assistant` dialog box. The dialog box has a close `X` icon that hides the dialog and shows the `Bot` button again. 

Bot Assistant should be easy to integrate, hence we'll use javascript to dynamically add and control these HTML elements.

### Bot client environment

Let's create a project directory called `assistant` and add a `bot-client` folder. 

```bash
 mkdir -p assistant/bot-client
 cd assistant/bot-client
```

Bot assistant code is loaded dynamically through javascript. Let's create the html file:

```bash
touch index.html
```

Copy the following code in `index.html` file:

```html
<!DOCTYPE HTML>
<html>
   <head> 
      <meta charset="utf-8">
      <meta http-equiv="X-UA-Compatible" content="IE=edge">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">

      <link rel="stylesheet" type="text/css" href="css/assistant.css"/>
      <script type = "text/javascript" src="scripts/assistant.js"></script>
   </head>

   <body>
      <div class="assistant"></div>
   </body>
</html>
```

The index file references a stylesheet `css/assistant.css` and a script `scripts/assistant.js`. The stylesheet and the scripts assume a `div` with class `assistant`.

Let's add the files next:

```bash
mkdir css; mkdir scripts
```

#### Assistant stylesheet

First let's add the stylesheet:

```bash
touch css/assistant.css
```

Copy the following code in `css/assistant.css`

```css
body {
	background: #f6f6f6;
}

.assistant {
	font-family: 'Lucida Sans', Geneva, Verdana, sans-serif;
	position:fixed;
	bottom:20px;
	right:25px;
}

/* Assistant - Button */
 .assistant button {
	width: 45px;
	height: 45px;
	background:#008CBA;
	border-radius:5px;
	cursor:pointer;
	border: none;
    outline: none;
}

.assistant button img {
	padding-top:5px;
	width: 25px;
	height: 25px;
}

.assistant button:focus {
    border: none;
    outline: none;
}

/* Assistant - Chat Dialog */
.assistant .chat{
	display: none;
	width:360px;
	background:white;
	border-radius:5px 5px 0px 0px;
	border: 1px solid gray;
}

.assistant .header{	
	background: #008CBA;
	color:white;
	padding:8px;
	font-weight:bold;
	border-radius:5px 5px 0px 0px;
	line-height: 32px;
}

.assistant .header span{	
	padding-left:0;
	font-size: 11pt;
}

.assistant .header img {
	width:18px;
	height:35px;
	margin-right: 10px;
	float:right;
}

.assistant .header img.bot {
	width:35px;
	height:35px;
	border-radius:50%;
	background:#bbb;
	float:left;	
}

.assistant .header .close{
	float:right;
	cursor:pointer;
	width: 28px;
	margin-right: 0;
}

.assistant .inner-body{
	min-height: 250px;
	max-height: calc(100vh - 120px);
	overflow: auto;
	overflow-x: hidden;
}

.assistant .msg-body {
	font-size:12px;
	padding: 10px 10px 5px 5px;
}

.assistant .footer {
	background:white;
	bottom: 0;
	padding-bottom: 10px;
	width: 100%;
}

.assistant .footer .textareaElement {
	padding: 15px 10px 0 10px;
	border-top: 1px solid #ccc;
	min-height: 20px;
	overflow-x: hidden;
	overflow-y: auto;
	font-size: 11pt;
	font-family: Arial, Helvetica, sans-serif;
	color: #333;
}

.assistant .footer .textareaElement:focus {
	outline: none;
}

.assistant .footer [placeholder]:empty::before {
    content: attr(placeholder);
    color: #aaa; 
}

.assistant .footer [placeholder]:empty:focus::before {
    content: "";
}
```

In summary the stylesheet has two core sections, *Assistant - Button* and *Assistant - Chat Box*.
The *Chat Box* has three subsections: a header with the bot icon, title and a close icon, the body area, and the footer. The footer has an editor for user input that is currently `read-only`.

#### Assistant script

Next, let's add the script that creates HTML components and manages the interaction with the dialog box:

```bash
touch scripts/assistant.js
```

Copy the following code in `scripts/assistant.js`

```javascript
window.onload = () => {

    // Create and attach Bot Assistant HTML elements
    function loadAssistant() {
        // Add assistant button
        var note = createElement("img", { "src": `img/assistant/note.svg` }),
            aButton = createElement("button", {}, note);

        // Append assistant dialog
        var bot = createElement("img", { "src": `img/assistant/bot.svg`, "class": "bot" }),
            title = createElement("span", {}, "Bot Assistant"),
            aDialogClose = createElement("img", { "src": `img/assistant/close.svg`, "class": "close" }),
            header = createElement("div", { "class": "header" }, [bot, title, aDialogClose]),
            msg_body = createElement("div", { "class": "msg-body" }),
            inner_body = createElement("div", { "class": "inner-body" }, msg_body),
            body = createElement("div", { "class": "body-wrapper" }, inner_body),
            user_msg = createElement("div", {
                "id": "user-msg",
                "class": "textareaElement",
                "placeholder": "Type here",
                "contenteditable": "false"
            }),
            footer = createElement("div", { "class": "footer" }, user_msg),
            aDialog = createElement("div", { "class": "chat" },
                [header, body, footer]);


        // Attach event listeners
        aButton.addEventListener('click', onOpenDialog, false);
        aDialogClose.addEventListener('click', onCloseDialog, false);

        // Add to document
        document.querySelector(".assistant").appendChild(aButton);
        document.querySelector(".assistant").appendChild(aDialog);
    }

    // On open assistant dialog callback
    function onOpenDialog() {
        document.querySelector(".assistant button").style.display = "none";
        document.querySelector(".assistant .chat").style.display = "block";
    }

    // On close assistant dialog callback
    function onCloseDialog() {
        document.querySelector(".assistant .chat").style.display = "none";
        document.querySelector(".assistant button").style.display = "block";
    }

    // Create element utility function
    function createElement(element, attribute, inner) {
        if (typeof (element) === "undefined") { return false; }
        if (typeof (inner) === "undefined") { inner = ""; }

        var el = document.createElement(element);
        if (typeof (attribute) === 'object') {
            for (var key in attribute) {
                el.setAttribute(key, attribute[key]);
            }
        }
        if (!Array.isArray(inner)) {
            inner = [inner];
        }
        for (var k = 0; k < inner.length; k++) {
            if (inner[k].tagName) {
                el.appendChild(inner[k]);
            } else {
                el.innerHTML = inner[k];
            }
        }
        return el;
    }

    // Call main function
    loadAssistant();
};
```

The script is invoked by <a href="https://developer.mozilla.org/en-US/docs/Web/API/Window/load_event" target="_blank">window.onload</a> event. Let's review the functions in reverse chronological order:

* **createElement** - a utility function that makes it easy to create DOM elements.
* **onCloseDialog** - operations to be performed when close dialog icon is clicked.
* **onOpenDialog** - operations to be performed when assistant button is clicked.
* **loadAssistant** - function to create button and editor, attach open/close listeners and append to DOM.

The script uses several images to enhance the visualization. Let's load the image from the github project:

```bash
mkdir -p img/assistant
curl http://fluvio.io/blog/images/bot-assistant/download/note.svg > img/assistant/note.svg
curl http://fluvio.io/blog/images/bot-assistant/download/bot.svg > img/assistant/bot.svg
curl http://fluvio.io/blog/images/bot-assistant/download/close.svg > img/assistant/close.svg
```

If everything worked as expect you should have the following hierarchy:

```bash
tree
.
├── css
│   └── assistant.css
├── img
│   └── assistant
│       ├── bot.svg
│       ├── close.svg
│       └── note.svg
├── index.html
└── scripts
    └── assistant.js
```

### Test Bot Client

To test the code we've written so far, open `assistant/index.html` in your Web browser. You should see an empty page the an icon at the bottom on the screen. Click on the icon to open the dialog box, the `X` in the dialog header to close it.

<img src="/blog/images/bot-assistant/bot-open-close.svg"
     alt="Bot Assistant Example"
     style="justify: center; max-width: 420px" />

Congratulations, `Bot Client` is up and running, let's setup the server next.


## Step 2: Add backend server

We need a backend server that can process multiple clients in different stages of the workflow in parallel. 

Our environment uses `Typescript` and `Node.js`, so let's setup the environment first.


#### Typescript

Next, create a typescript configuration file:

```bash
touch tsconfig.json
```

Copy the following content in `tsconfig.json` file:

```json
{
  "compilerOptions": {
    "target": "es6",
    "module": "commonjs",
    "lib": [
      "dom",
      "es2017",
      "ES2015"
    ],
    "sourceMap": true,
    "declaration": true,
    "outDir": "dist",
    "strict": true,
    "noImplicitAny": true,
    "strictPropertyInitialization": false,
    "moduleResolution": "node",
    "esModuleInterop": true,
    "experimentalDecorators": true,
    "resolveJsonModule": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": [
    "src/*",
  ],
}
```

### Bot server environment

Create a directory `bot-server` in parallel with `bot-client`:

```bash
mkdir bot-server
cd bot-server
```

#### Add Node.js server

Initialize a new node package (this example uses Node v13.5.0), and install a few services:

```bash
npm init -y
```

Install `express`, `typescript` and a few other development services:

```bash
npm install typescript express && \
npm install -D ts-node tsc-watch @types/node @types/express
```

We installed `tsc-watch` to keep track of file changes for `bot-server`. Update `package.json` script _main_ and _script_ values with the following:

{{< highlight json "hl_lines=5 7" >}}
{
  "name": "bot-server",
  "version": "1.0.0",
  "description": "",
  "main": "bot-server.js",
  "scripts": {
    "start:server": "tsc-watch --onSuccess \"node ./dist/bot-server.js $PARAMS\""
  },
  "keywords": [],
  "author": "fluvio <admin@fluvio.io> (fluvio.io)",
  "license": "Apache 2.0",
  "dependencies": {
    "express": "^4.17.1",
    "typescript": "^4.1.2"
  },
  "devDependencies": {
    "@types/express": "^4.17.9",
    "@types/node": "^14.14.9",
    "ts-node": "^9.0.0",
    "tsc-watch": "^4.2.9"
  }
}
{{< /highlight >}}

#### Add `bot-server.ts` file

It's time to add the `src` directory and provision the `bot-server.ts` file:

```bash
mkdir src
touch src/bot-server.ts
```

We'll start by provisioning an `express` server to ensure the environment is in working order.
Copy the following content in `src\bot-server.ts` file:

```ts
import http from "http";
import express from "express";

const PORT = 9998;

// Provision Bot Assistant Server
const startServer = async () => {
    const app = express();
    const Server = http.createServer(app);

    // Start server
    Server.listen(PORT, () => {
        console.log(
            `started bot assistant server at http://localhost:${PORT}...`
        );
    });

};

// Start Server
startServer();
```

The code provisions a web server on port 9998. 

Let's sanity check `bot-server` the file hierarchy:

```bash
tree
.
├── package.json
├── src
│   └── bot-server.ts
└── tsconfig.json
```


### Test Bot Server

Let's compile to ensure everything the server is up and running:

```bash
npm run start:server

3:18:15 PM - Starting compilation in watch mode...
3:18:17 PM - Found 0 errors. Watching for file changes.
started bot assistant server at http://localhost:9998...
```

Congratulations, `Bot Server` is up and running.


## Step 3: Add WebSocket communication

WebSocket (WS) protocol is natively integrated in most common web browsers and is also readily available in Node.js/Typescript. Hence, we'll use WS to create a simple Ping/Pong the client/server communication exchange.

<img src="/blog/images/bot-assistant/ping-pong.svg"
     alt="WebSocket Ping/Pong"
     style="justify: center; max-width: 440px" />

### Add WebSocket to `bot-server`

First, we'll install websocket package, then we'll create a simple echo server to respond to ping requests.

#### Install WebSocket

Install `ws` package as follows:

```bash
npm install ws && \
npm install -D @types/ws
```

#### Implement WebSocket echo server

WebSocket negotiation should be handled separately from the rest of the system. In the `bot-server` directory let's create a new file for that purpose:

```bash
touch src/ws-server.ts
```

Copy the following content in the `src/ws-server.ts` file:

```ts
import WS from "ws";
import http from "http";

class WsServer {
    private _wss: WS.Server;

    constructor() {
        this._wss = new WS.Server({ clientTracking: false, noServer: true });
    }

    public init(server: http.Server) {
        this.onUpgrade(server);
        this.onConnection();
    }

    private onUpgrade(server: http.Server) {
        server.on("upgrade", function (request, socket, head) {

            wsSingleton._wss.handleUpgrade(request, socket, head, function (ws: WS) {
                wsSingleton._wss.emit("connection", ws, request);
            });
        });
    }

    private onConnection() {
        this._wss.on("connection", function (ws, req) {
            console.log("session opened");

            ws.on("close", function () {
                console.log("session closed");
            });

            ws.on("message", (msgObj: string) => {
                console.log(`< ${msgObj}`);
                if (msgObj == "ping") {
                    ws.send("pong");
                    console.log("> pong");
                }
            });

        });
    }

}

const wsSingleton = new WsServer();
Object.freeze(wsSingleton);

export default wsSingleton;
```

We a singleton class to handle all websocket communication. 

Let's review the code in the `WsServer` class:
* A websocket server is saved in a static variable when the class is instantiated.
* Two callbacks `onUpgrade` and `onConnection` are registered during initialization.
* WebSocket protocol invokes `onUpgrade` when a new client is connected. 
    * It emits an `onConnection` event.
* `onConnection` is where all connection related changes are handled
    * Anytime a **ping** message is received, the server responds with a **pong** message.

The WsServer is then called inside `src/bot-server.ts`. The following code goes in the file header:

```ts
import express from "express";
import WebSocket from "./ws_server";
...

```

Next we need to call WebSocket API inside `startServer` block:

```ts
const startServer = async () => {
    ...
    const Server = http.createServer(app);

    WebSocket.init(Server);
    ...
}
```

Note that `ts-watch` has updated the code. Next, we'll add websocket to the client.


#### Add WebSocket to`bot-client`

WebSocket library is available in most modern web browser, which allows us to hook it up with javascript. In the `bot-client` directory let's add some troubleshooting.


##### Add troubleshooting 

Open open `scripts/assistant.js` and add a function to publish client logs to a textarea called `debugOutput`:

```javascript
window.onload = () => {
    ...

    function logOutput(value) {
        var debugOutput = document.getElementById("debugOutput");
        if (debugOutput) {
            debugOutput.value += value + "\n";
            debugOutput.scrollTop = debugOutput.scrollHeight;
        }
        console.log(value);
    }
}
```

Next, open `index.html` and add `debugOutput*` textarea:

```html
<body>
    ...
    <!-- debugging area - begin -->
    <textarea id="debugOutput" rows="20" cols="60" readonly></textarea>
    <!-- debugging area - end -->
</body>
```

Anytime the code calls `logOutput`, it will be display in the textarea (as well as as in the console).

##### Update assistant.js file

Now, are read to update the code. Open `scripts/assistant.js` and make the following updates.
Add a variable `webSocket` to cache the websocket instance.

```javascript
window.onload = () => {
    var webSocket = null;
    ...
}
```

Next, let add `onWsConnection` to connect to the websocket connection management events:

```javascript
window.onload = () => {
    ...

   // Open WebSocket connection
    function openWSConnection() {
        try {
            if (webSocket != null) {
                return; // already connected
            }

            logOutput("Connecting to: ws://localhost:9998/");
            webSocket = new WebSocket("ws://localhost:9998/");

            webSocket.onopen = function (openEvent) {
                logOutput("Connected!");
            };

            webSocket.onclose = function (closeEvent) {
                logOutput("Disconnected!");
            };

            webSocket.onerror = function (errorEvent) {
                logOutput(`Error: ${JSON.stringify(errorEvent)}`);
            };

            webSocket.onmessage = function (messageEvent) {
                var wsMsg = messageEvent.data;
                logOutput(`< ${wsMsg}`);
            };

        } catch (exception) {
            logOutput(`error: ${JSON.stringify(exception)}`);
        }
    }

    ...
}
```

Let's temporarily hook-up the connection to `onOpenDialog` to test the WS connection:

```javascript
function onOpenDialog() {
    ...
    openWSConnection();
}
```

Next, we  need to enable chat editor and link `onEditorKeys` to capture user input.

```javascript
window.onload = () => {
    ...

    // Make editor section editable
    function enableChatEditor() {
        var chatBox = document.getElementById("user-msg");
        chatBox.setAttribute("contenteditable", true);

        chatBox.addEventListener("keydown", onEditorKeys, false);
    }

    // Callback on chat editor user input (key press)
    function onEditorKeys(e) {
        var chatBox = document.getElementById("user-msg");

        if (e.code == 'Enter' && chatBox.textContent.length > 0) {
            e.preventDefault();

            const content = chatBox.textContent;
            webSocket.send(content);

            logOutput(`> ${content}`);
            chatBox.innerHTML = '';
        }
    }

    ...
}
```

Finally, we need to hook-up enabling routine, after `loadAssistant`:

```javascript
window.onload = () => {
    ....
    loadAssistant();

    // TODO: Remove after testing
    enableChatEditor();
}
```

Checkout the consolidate `assistant.js` file in <a href="https://gist.github.com/ajhunyady/9449ad8c29f1d6131a9a2d860f33bc7b" target="_blank">gist</a>.


### Test WebSocket Communication

We are leveraging Bot Assistant editor window to test our websocket communication. Ensure the [bot-server is up and running](#test-bot-server) and refresh `index.html` in the web browser.

Type `ping` in the editor window, and the server responds with `pong`.

<img src="/blog/images/bot-assistant/ping-pong-test.svg"
     alt="Ping/Pong WebSocket Connection Example"
     style="justify: center; max-width: 800px" />


## Step 4: Add session cookies

Bot assistant must support multiple conversations in parallel. The `bot-server` will generate a unique session id for each new conversation. The session id is saved as a `cookie` in the client Web Browser.

### Add session cookie to `bot-server`

First, we'll add session cookie support `ws-server.ts` file. 

{{< highlight typescript "hl_lines=3-5 12-15 25-34 37-38 41 56-69" >}}
import WS from "ws";
import http from "http";
import crypto from 'crypto';

const COOKIE_NAME = "Fluvio-Bot-Assistant"

class WsServer {
    ...

    private onUpgrade(server: http.Server) {
        server.on("upgrade", function (request, socket, head) {
            const session = parseSessionFromCookie(request.headers.cookie);
            if (session) {
                request.headers.session = session;
            }

            wsSingleton._wss.handleUpgrade(request, socket, head, function (ws: WS) {
                wsSingleton._wss.emit("connection", ws, request);
            });
        });
    }

    private onConnection() {

        this._wss.on("headers", function (headers: Array<string>, req) {
            const session = parseSessionFromCookie(req.headers.cookie);

            if (!session) {
                let session = crypto.randomBytes(20).toString("hex");
                req.headers.session = session;

                headers.push("Set-Cookie: " + COOKIE_NAME + "=" + session);
            }
        });

        this._wss.on("connection", function (ws, req) {
            const session = parseSessionFromCookie(req.headers.cookie);
            console.log(`session opened - ${session}`);

            ws.on("close", function () {
                console.log(`session closed - ${session}`);
            });

            ws.on("message", (msgObj: string) => {
                console.log(`< ${msgObj}`);
                if (msgObj == "ping") {
                    ws.send("pong");
                    console.log("> pong");
                }
            });

        });
    }
}

function parseSessionFromCookie(cookie?: string) {
    if (cookie) {
        const cookiePair = cookie.split(/; */).map((c: string) => {
            const [key, v] = c.split('=', 2);
            return [key, decodeURIComponent(v)];
        }).find(res =>
            (res[0] == COOKIE_NAME)
        );

        if (Array.isArray(cookiePair) && cookiePair.length > 1) {
            return cookiePair[1];
        }
    }
}
{{< /highlight >}}

<center><b>ws-server.ts</b> published in <a href="https://gist.github.com/ajhunyady/2e1fe181bec8dbc47d5e36dd96fc044f" target="_blank">gist</a></center>

We generate a cookie called **Fluvio-Bot-Assistant** and utilize WebSocket to send it to the client.

WebSocket connection setup hast two steps:
1. `headers` request
2. `connection` request. 

In `headers` callback, we check the HTTP headers for the session cookie. If not found, we'll generate a new session id and append the HTTP header. WebSocket will take care of the rest. In `connection` callback, we read the session id from `request.headers.session`.

### Add web server support to `bot-client`

In the web client cookies are handled by the HTTP protocol implemented by Web servers. Our `bot-client` is just a file, hence we to service it from a Node server.

#### Add node.js to `bot-client`

Let's generate a new node project inside `bot-client` directory:

```bash
cd ../bot-client
npm init -y
```

Install `express` and `nodemon` services:

```bash
npm install express && \
npm install -D nodemon
```

Update `package.json` script _main_ and _script_ values with the following:

{{< highlight json "hl_lines=5 7" >}}
{
  "name": "bot-client",
  "version": "1.0.0",
  "description": "",
  "main": "client.js",
  "scripts": {
    "start:client": "nodemon --watch client ./client.js"
  },
  "keywords": [],
  "author": "fluvio <admin@fluvio.io> (fluvio.io)",
  "license": "Apache 2.0",
  "dependencies": {
    "express": "^4.17.1"
  },
  "devDependencies": {
    "nodemon": "^2.0.6"
  }
}
{{< /highlight >}}

Next, we need to to add a file that manages the server configurations.


#### Add `client.js` file

Let's add `client.js` at the root of the `bot-client` directory:

```bash
touch client.js
```

Copy the following content in `client.js` file:

```ts
const express = require('express');
const path = require('path');
const app = express();

const PORT = 9999;

app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, 'index.html'));
});

app.use("/scripts", express.static(__dirname + "/scripts"));
app.use("/css", express.static(__dirname + "/css"));
app.use("/img", express.static(__dirname + "/img"));

app.listen(PORT, () => {
    console.log(`listening http://localhost:${PORT}`);
});
```

We added routes for our existing directories and provisioned the web client to run on port 9999. 

### Test Session Cookies

To test the code, let's start the web client.

```bash
npm run start:client
...
[nodemon] starting `node ./client.js`
listening http://localhost:9999
```

Assuming `bot-server` web server is also running, open a web browser and load `localhost:9999`.
Click on Bot Assistant icon to generate a new connection.

<img src="/blog/images/bot-assistant/client-cookies.svg"
     alt="Client Cookies"
     style="justify: center; max-width: 680px" />

Open the cookies in browser settings and notice a `Fluvio-Bot-Assistant` cookie. If you delete the cookie and refresh the page, a new session id is generated.

Congratulations, you have successfully added session cookies to Bot Assistant. Next, we'll build the state machine that drives the user interaction. 


## Step 5: Add state machine

While there are many ways to drive a robot assistant, this project uses a custom state machine. You may think of a state machine as a guided tour, where the bot sends the client a series of choices. Upon response the bot looks-up the reply and moves to the next state. Then the loop repeats until the end state is reached.

<img src="/blog/images/bot-assistant/state-machine.svg"
     alt="State Machine"
     style="justify: center; max-width: 800px;" />

The state machine definition is expressed in a JSON file described in the next section.

### State machine definition

The state machine is composed of states, where the states can be:
* sendRequest
* matchResponse

The bot uses `sendRequest` to generate messages for the client and `matchResponse` to compare the client response with the state where it should resume.

Bot assistant generates one of the following `sendRequest` messages:

* **BotText** - sends information to the user (text or HTML format)
* **ChoiceRequest** - sends a list of options to the user. GroupId is the unique identifier used to pair this request with the *ChoiceResponse*
* **StartChatSession** - asks the client to enable chat editor. SessionId is the unique identifier and it used to pair it with *UserText*
* **EndChatSession** - ask the client to disable chat session. Uses the SessionId defined in the *StartChatSession*

The Client replies with one of the following `mathResponse` messages:

* **ChoiceResponse** - one or more responses with unique itemIds for the groupId matching a *ChoiceRequest*
* **UserText** - a text response with sessionId matching a *StartChatSession*

#### State transitions

Each State has a field called `next` that informs bot of a state transition. Workflow manager receives the starting state from the client `Response` and traverses all states linked by the next field. When an empty `next` field is encountered, a `Request` is generated and sent to client.

<img src="/blog/images/bot-assistant/state-transitions.svg"
     alt="State Transitions"
     style="justify: center; max-width: 800px;" />

The client displays the choices and asks the user to make a selection. Upon selection, the client generates a `Response` and replies to the bot assistant.

Next, we'll create a custom state machine in JSON format.


### Create state machine JSON file

Bot assistant defines custom state machines in JSON. Let's add a custom state machine in the `bot-server` directory.


```bash
touch state-machine.json
```

Copy following state machine in the JSON file:

```json
{
    "greetings": {
        "sendRequest": {
            "kind": "BotText",
            "content": "Hi, I'm Bot! Nice to meet you."
        },
        "next": "langChoices"
    },
    "langChoices": {
        "sendRequest": {
            "kind": "ChoiceRequest",
            "groupId": "lang",
            "question": "What's your favorite programming language?",
            "choices": [
                {
                    "itemId": "rust",
                    "content": "Rust"
                },
                {
                    "itemId": "go",
                    "content": "Go"
                },
                {
                    "itemId": "other",
                    "content": "Other"
                }                
            ]
        }
    },
    "langChoiceRust": {
        "matchResponse": {
            "kind": "ChoiceResponse",
            "groupId": "lang",
            "itemId": "rust"
        },
        "next": "tryAgainChoices"
    },
    "langChoiceGo": {
        "matchResponse": {
            "kind": "ChoiceResponse",
            "groupId": "lang",
            "itemId": "go"
        },
        "next": "tryAgainChoices"
    },
    "langChoiceOther": {
        "matchResponse": {
            "kind": "ChoiceResponse",
            "groupId": "lang",
            "itemId": "other"
        },
        "next": "startLangPrefSession"
    },
    "startLangPrefSession": {
        "sendRequest": {
            "kind": "StartChatSession",
            "sessionId": "langPreference",
            "chatPrompt": "Type your preferred language here...",
            "chatText": "I enabled chat editor"
        }
    },
    "getLangPrefResponse": {
        "matchResponse": {
            "kind": "UserText",
            "sessionId": "langPreference"
        },
        "next": "endLangPrefSession"
    },
    "endLangPrefSession": {
        "sendRequest": {
            "kind": "EndChatSession",
            "sessionId": "langPreference"
        },
        "next": "tryAgainChoices"
    },
    "tryAgainChoices": {
        "sendRequest": {
            "kind": "ChoiceRequest",
            "groupId": "again",
            "question": "Try again?",
            "choices": [
                {
                    "itemId": "yes",
                    "content": "Yes"
                },
                {
                    "itemId": "no",
                    "content": "No"
                }                
            ]
        }
    },
    "tryAgainYes": {
        "matchResponse": {
            "kind": "ChoiceResponse",
            "groupId": "again",
            "itemId": "yes"
        },
        "next": "langChoices"
    },
    "tryAgainNo": {
        "matchResponse": {
            "kind": "ChoiceResponse",
            "groupId": "again",
            "itemId": "no"
        },
        "next": "done"
    },     
    "done": {
        "sendRequest": {
            "kind": "BotText",
            "content": "Great, thanks!"
        }
    }  
}
```

<center><b>state-machine.json</b> published in <a href="https://gist.github.com/ajhunyady/ac6612a4f2653e5ab58b5c6e96782039" target="_blank">gist</a></center>

The state machine asks the user for his or her favorite programming language. It shows 3 options:
* Rust
* Go
* Other

If the user chooses `Rust` or `Go`, it sends a request choice `Try Again`. For `Other`, it runs a sequence of states:
* **sendRequest** to open an interactive session,
* **matchResponse** to captures the user response,
* **sendRequest** to close interactive the session
* **sendRequest** choice to try again.

This basic state machine show two different interaction models, a choice and a user input.

### Load state machine

We create two files to load the state machine into Bot Assistant:
1. `messages.ts` - defines the message types
2. `state-machine.ts` - augments messages with state interface and loads the JSON file.

#### Create `messages.ts` file

Create `messages.ts` file inside `bot-server/src` directory:

```bash
touch messages.ts
```

Copy the following message definitions:

```typescript
/* Request Messages */
export type RequestMessage =
    | BotText
    | ChoiceRequest
    | StartChatSession
    | EndChatSession;

/* Response Messages */
export type ResponseMessage =
    | ChoiceResponse
    | UserText

export interface BotText {
    kind: "BotText",
    content: string
}

export interface ChoiceRequest {
    kind: "ChoiceRequest",
    question: string,
    groupId: string,
    choices: Array<Choice>,
}

export interface Choice {
    itemId: string,
    content: string
}

export interface UserText {
    kind: "UserText",
    sessionId: string,
    content?: string,
}

export interface ChoiceResponse {
    kind: "ChoiceResponse",
    groupId: string,
    itemId: string,
    content?: string,
}

export interface StartChatSession {
    kind: "StartChatSession",
    sessionId: string,
    chatPrompt?: string,
    chatText?: string,
}

export interface EndChatSession {
    kind: "EndChatSession",
    sessionId: string,
}
```

RequestMessage and ResponseMessage are unions of different message types.


#### Create `state-machine.ts` file

Create `state-machine.ts` file inside `bot-server/src` directory:

```bash
touch state-machine.ts
```

Copy the following code in the state-machine file:

```typescript
import Fs from "fs";
import { RequestMessage, ResponseMessage } from "./messages";

type name = string;

/* State Machine definition */
export type StateMachine = Map<name, State>;

export interface State {
    sendRequest?: RequestMessage,
    matchResponse?: ResponseMessage;
    next?: string,
}

/* Load state machine from JSON file */
export function loadStateMachine(filePath: string) {
    const jsonFile = Fs.readFileSync(filePath);
    const jsonObject = JSON.parse(jsonFile.toString());

    var state_machine: StateMachine = new Map<string, State>();
    for (var value in jsonObject) {
        state_machine.set(value, jsonObject[value])
    }

    return state_machine;
}
```

The code defines the state machine structures, reads the JSON file, and provisions an internal state machine object.
To test the code, we'll temporarily attach the state machine parser to `bot-server.ts`:

{{< highlight typescript "hl_lines=4-5 23-26" >}}
import http from "http";
import express from "express";
import WebSocket from "./ws_server";
import Path from "path"
import { StateMachine, loadStateMachine } from "./state_machine";

const PORT = 9998;

// Provision Bot Assistant Server
const startServer = async () => {
    const app = express();
    const Server = http.createServer(app);

    WebSocket.init(Server);

    // Start server
    Server.listen(PORT, () => {
        console.log(
            `started bot assistant server at http://localhost:${PORT}...`
        );
    });

    // Initialize State Machine
    let filePath = Path.join(__dirname, "..", "..", "bot-server", "state-machine.json")
    const stateMachine: StateMachine = loadStateMachine(filePath);
    console.log(stateMachine);
};

// Start Server
startServer();
{{< /highlight >}}

Finally, let's test the code:

```bash
npm run start:server

12:11:29 PM - Found 0 errors. Watching for file changes.
Map(9) {
  'greetings' => {
    sendRequest: { kind: 'BotText', content: "Hi, I'm Bot! Nice to meet you." },
    next: 'langChoices'
  },
  ....
}
```

Congratulations, your state machine parser is up and running. Next is the workflow manager.


## Step 6: Add workflow manager