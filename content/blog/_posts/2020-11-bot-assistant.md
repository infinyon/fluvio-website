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
* Workflow Engine
* Data streaming and Persistency

The implementation uses the following software components:

* <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript" target="_blank">Javascript</a> - to develop front end development
* <a href="https://www.typescriptlang.org/docs/" target="_blank">TypeScript</a> - to build the backend
* <a href="https://nodejs.org/">Node.js</a> - to run the web server
* <a href="https://developer.mozilla.org/en-US/docs/Web/API/WebSocket" target="_blank">WebSocket</a> - for real-time communication
* <a href="https://fluvio.io" target="_blank">Fluvio</a> - for data streaming and persistency


## Step 1: Add frontend client

The frontend navigation is pretty straight forward. There is a `Bot` button on the lower right-hand side of the screen and a `Bot Assistant` dialog box. The `Bot` button opens the `Bot Assistant` dialog box and the close `X` icon hides the dialog and shows the `Bot` button again. We use javascript to dynamically add and control these components.

<img src="/blog/images/bot-assistant/bot-nav.svg"
     alt="Bot Assistant Navigation"
     style="justify: center; max-width: 420px" />

### Bot client environment

Let's create an `assistant` project directory and add a `bot-client` folder. 

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
	font-family: 'Lucida Sans', 'Lucida Sans Regular', 'Lucida Grande', 'Lucida Sans Unicode', Geneva, Verdana, sans-serif;
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
     style="justify: center; max-width: 580px" />

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

#### Node.js

Initialize a new node package (this example uses Node v13.5.0), and install a few services:

```bash
npm init -y
```

Rename resulting script file name:

```json
{
  "main": "bot-server.js",
}
```

Install `express`, `typescript` and a few other development services:

```bash
npm install typescript express && \
npm install -D ts-node tsc-watch @types/node @types/express
```

We installed `tsc-watch` to keep track of file changes for `bot-server`; a file that we'll generate in next section. Let's create `start:server` script to watch for changes:

```json
{
    "scripts": {
        "start:server": "tsc-watch --onSuccess \"node ./dist/bot-server.js $PARAMS\""
    },
}
```

Resulting `package.json` is as follows:

```json
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
```

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

The program initializes a web server running on port 9998.

### Test Bot Server

Let's compile to ensure everything the server is up and running:

```bash
npm run start:server

3:18:15 PM - Starting compilation in watch mode...
3:18:17 PM - Found 0 errors. Watching for file changes.
started bot assistant server at http://localhost:9998...
```

Congratulations, `Bot Server` is up and running.


## Step 3: Add websocket communication

Websocket (WS) protocol is natively integrated in most common web browsers and is also readily available in Node.js/Typescript. Hence, we'll use WS to create a simple Ping/Pong the client/server communication exchange.

<img src="/blog/images/bot-assistant/ping-pong.svg"
     alt="WebSocket Ping/Pong"
     style="justify: center; max-width: 440px" />

### Add websocket to bot server

First, we'll install websocket package, then we'll create a simple echo server to respond to ping requests.

#### Install websocket

Install `ws` package as follows:

```bash
npm install ws && \
npm install -D @types/ws
```

#### Add websocket echo server

Websocket negotiation should be handled separately from the rest of the system. In the `bot-server` directory let's create a new file for that purpose:

```bash
touch src/ws_server.ts
```

Copy the following content in the `src/ws_server.ts` file:

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
* Websocket protocol invokes `onUpgrade` when a new client is connected. 
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


#### Add WebSocket to bot client

Websocket library is available in most modern web browser, which allows us to hook it up with javascript. In the `bot-client` directory let's add some troubleshooting.


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


### Test websocket Communication

We are leveraging Bot Assistant editor window to test our websocket communication. Ensure the [bot-server is up and running](#test-bot-server) and refresh `index.html` in the web browser.

Type `ping` in the editor window.

<img src="/blog/images/bot-assistant/ping-pong-test.svg"
     alt="Bot Assistant Example"
     style="justify: center; max-width: 800px" />