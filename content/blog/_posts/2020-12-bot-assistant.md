---
title: Build Your Own Custom Robot Assistant
author: 
    name: "The Fluvio Team"
description: Leverage fluvio real-time data streaming to build your own custom robot assistant.
date: 2020-12-08
slug: bot-assistant
url: /blog/2020/12/bot-assistant
---

Robot assistants are nothing new, they have been around for a while with many full featured products available for use. While these products fit most use cases, there are time when you want to roll out your own.

A custom robot assistant gives you full control over and workflows and the services for your customer interactions. When used in conjunction with a data streaming product such as Fluvio, you gain persistence, resiliency, and scalability.

<img src="/blog/images/bot-assistant/bot-assistant.svg"
     alt="Bot Assistant Example"
     style="justify: center; max-width: 380px" />

The project is also available for download in <a href="https://github.com/infinyon/fluvio-demo-apps-node/tree/master/bot-assistant" target="_blank">github</a>.

#### Overview 

This blog will describe how to build a robot assistant, called `Bot Assistant` one step at a time. In summary:

* [Step 1: Add frontend client](#step-1-add-frontend-client)
* [Step 2: Add backend server](#step-2-add-backend-server)
* [Step 3: Add websocket communication](#step-3-add-websocket-communication)
* [Step 4: Add session cookies](#step-4-add-session-cookies)
* [Step 5: Add state machine](#step-5-add-state-machine)
* [Step 6: Add workflow controller](#step-6-add-workflow-controller)
* [Step 7: Add data streaming and persistency](#step-7-add-data-streaming-and-persistency)

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

Paste the following code in `index.html` file:

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

In the header we are referencing two files:
* `css/assistant.css` - styles file
* `scripts/assistant.js` - script file to change the DOM.

In the body, we have a `div` with class named `assistant`. The script file looks this `div` to provision the Bot assistant.

Let's add the stylesheet and the script files next:

```bash
mkdir css; mkdir scripts
```

#### Assistant stylesheet

First let's add the stylesheet:

```bash
touch css/assistant.css
```

Paste the following code in `css/assistant.css`

```css
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

/* Assistant - Chat Box */

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
	max-height: calc(100vh - 300px);
	overflow: auto;
	overflow-x: hidden;
}

.assistant .msg-body {
	font-size:12px;
	padding: 10px 10px 5px 5px;
}

/* Footer  */

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

In summary the stylesheet has three sections, *Assistant - Button* and *Assistant - Chat Box*.
The *Chat Box* has three subsections: a header with the bot icon, title and a close icon, the body area, and the footer. The footer has an editor for user input that is currently `read-only`.

#### Assistant script

Next, let's add the script that creates DOM elements for the user interaction.

```bash
touch scripts/assistant.js
```

Paste the following code in `scripts/assistant.js`

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
            msgBody = createElement("div", { "class": "msg-body" }),
            innerBody = createElement("div", { "class": "inner-body" }, msgBody),
            body = createElement("div", { "class": "body-wrapper" }, innerBody),
            userMsg = createElement("div", {
                "id": "user-msg",
                "class": "textareaElement",
                "placeholder": "Choose an option",
                "contenteditable": "false"
            }),
            footer = createElement("div", { "class": "footer" }, userMsg),
            aDialog = createElement("div", { "class": "chat" }, [header, body, footer]);

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

The script is invoked during <a href="https://developer.mozilla.org/en-US/docs/Web/API/Window/load_event" target="_blank">window.onload</a> event. The functions are as follows:
* **loadAssistant** - function to create button and editor, attach open/close listeners and append to DOM.
* **onOpenDialog** - callback invoked when assistant button is clicked.
* **onCloseDialog** - callback invoked when close dialog icon is clicked.
* **createElement** - a utility function that makes it easy to create DOM elements.

The script uses several images to enhance the visualization. Let's load the image from the github project:

```bash
mkdir -p img/assistant
curl -L http://fluvio.io/blog/images/bot-assistant/download/note.svg --output img/assistant/note.svg
curl -L http://fluvio.io/blog/images/bot-assistant/download/bot.svg --output img/assistant/bot.svg
curl -L http://fluvio.io/blog/images/bot-assistant/download/redo.svg --output img/assistant/redo.svg
curl -L http://fluvio.io/blog/images/bot-assistant/download/close.svg --output img/assistant/close.svg
```

You should end up with the following file hierarchy:

```bash
tree
.
├── css
│   └── assistant.css
├── img
│   └── assistant
│       ├── bot.svg
│       ├── close.svg
│       ├── note.svg
│       └── redo.svg
├── index.html
└── scripts
    └── assistant.js
```

The source code for **Step 1** is available in <a href="https://github.com/infinyon/fluvio-demo-apps-node/tree/master/bot-assistant/_blog/step1/bot-client" target="_blank">github</a>.

### Test Bot client

To test the code we've written so far, open `assistant/index.html` in your Web browser. You should see an empty page the an icon at the bottom on the screen. Click on the icon to open the dialog box, the `X` in the dialog header to close it.

<img src="/blog/images/bot-assistant/bot-open-close.svg"
     alt="Bot Assistant Example"
     style="justify: center; max-width: 420px" />

Congratulations, `Bot Client` is up and running, let's setup the server next.


## Step 2: Add backend server

With the frontend in place, we turn our attention to the backend. The backend server needs to handle multiple client connections simultaneously. Each client is an independent entity in an arbitrary state of the workflow. 

Our project uses `Typescript` and `Node.js` which requires that we setup the environment first.

### Bot server environment

Create a directory `bot-server` in parallel with `bot-client`:

```bash
cd ..
mkdir bot-server
cd bot-server
```

#### Add Typescript

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

Paste the following content in `src\bot-server.ts` file:

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
tree -I 'node_modules|dist'
.
├── package-lock.json
├── package.json
├── src
│   └── bot-server.ts
└── tsconfig.json
```

The source code for **Step 2** is available in <a href="https://github.com/infinyon/fluvio-demo-apps-node/tree/master/bot-assistant/_blog/step2" target="_blank">github</a>.

### Test Bot server

Let's compile to ensure everything the server is up and running:

```bash
npm run start:server

3:18:15 PM - Starting compilation in watch mode...
3:18:17 PM - Found 0 errors. Watching for file changes.
started bot assistant server at http://localhost:9998...
```

Congratulations, `Bot Server` is up and running.


## Step 3: Add websocket communication

WebSocket (WS) protocol is natively integrated in most common web browsers and is also readily available in Node.js/Typescript. Let's use WS to create a simple Ping/Pong the client/server communication exchange to test communication. In the next sections, we'll expand on this to implement the bot assistant protocol.

<img src="/blog/images/bot-assistant/ping-pong.svg"
     alt="WebSocket Ping/Pong"
     style="justify: center; max-width: 440px" />

### Add websocket to `bot-server`

Websocket package in node is called `ws` and it is available for download through **npm**. We'll use `ws` to create a simple echo server that responds to client ping requests.

#### Install websocket

Inside `bot-server` directory, install `ws` package and the typescript definition file:

```bash
npm install ws && \
npm install -D @types/ws
```

#### Implement websocket echo server

Our WebSocket negotiation is a proxy that intermediates the communication from clients to the server business logic. We'll create a new file called `ws-proxy` in the `bot-server` directory:

```bash
touch src/ws-proxy.ts
```

Paste the following content in the `src/ws-proxy.ts` file:

```ts
import WS from "ws";
import http from "http";

export class WsProxy {
    private static _wss: WS.Server;

    constructor() {
        WsProxy._wss = new WS.Server({ clientTracking: false, noServer: true });
    }

    public init(server: http.Server) {
        this.onUpgrade(server);
        this.onConnection();
    }

    private onUpgrade(server: http.Server) {
        server.on("upgrade", function (request, socket, head) {
            WsProxy._wss.handleUpgrade(request, socket, head, function (ws: WS) {
                WsProxy._wss.emit("connection", ws, request);
            });
        });
    }

    private onConnection() {
        WsProxy._wss.on("connection", function (ws, req) {
            console.log("session opened");

            ws.on("close", function () {
                console.log("session closed");
            });

            ws.on("message", (clientMsg: string) => {
                console.log(`<== ${clientMsg}`);

                if (clientMsg == "ping") {
                    ws.send("pong");
                    console.log("==> pong");
                }
            });
        });
    }
}
```

We created a class `WsProxy` to handle websocket communication. Let's review the code:

* **constructor**: provisions a `WS` and saves it in a static variable.
* **init**: adds two callbacks `onUpgrade` and `onConnection` to handle WebSocket protocol.
    * `onUpgrade` is invoked when a new client is connected. By convention this function emits an **onConnection** event.
    * `onConnection` is called when connection related events, such as: **connection**, **message**, and **close**.

The behavior of the server is quite simple: it accepts new client connections, checks against **ping** and responds with **pong**.

Next, we need to instantiate the WsProxy inside `src/bot-server.ts`:

{{< highlight typescript "hl_lines=3 12-14" >}}
import http from "http";
import express from "express";
import { WsProxy } from "./ws-proxy";

const PORT = 9998;

// Provision Bot Assistant server
const startServer = async () => {
    const app = express();
    const Server = http.createServer(app);

    // Attach websocket to server
    const wsProxy = new WsProxy();
    wsProxy.init(Server);

    // Start server
    Server.listen(PORT, () => {
        console.log(
            `started bot assistant server at http://localhost:${PORT}...`
        );
    });
};

// Start Server
startServer();
{{< /highlight >}}

Note that `ts-watch` refreshed the code for us. Next, we'll add websocket to the client.


#### Add websocket to `bot-client`

WebSocket library is available in most modern web browser, which allows us to hook it up with javascript with easy. Before we do that, we'll add a troubleshooting window to help us track message exchanges between the client and the server. 

##### Add troubleshooting 

Open `bot-client/index.html` file and add a `debugOutput` textarea:

{{< highlight html "hl_lines=15-17" >}}
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

        <!-- debugging area - begin -->
        <textarea id="debugOutput" rows="20" cols="60" readonly></textarea>
        <!-- debugging area - end -->
   </body>
</html>
{{< /highlight >}}

The code adds a textarea with id `debugOutput`. Let's update javascript assistant `bot-client/scripts/assistant.js` to publish to this text area:

{{< highlight javascript "hl_lines=47-55" >}}
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
            msgBody = createElement("div", { "class": "msg-body" }),
            innerBody = createElement("div", { "class": "inner-body" }, msgBody),
            body = createElement("div", { "class": "body-wrapper" }, innerBody),
            userMsg = createElement("div", {
                "id": "user-msg",
                "class": "textareaElement",
                "placeholder": "Choose an option",
                "contenteditable": "false"
            }),
            footer = createElement("div", { "class": "footer" }, userMsg),
            aDialog = createElement("div", { "class": "chat" }, [header, body, footer]);

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

    // Log output in the "debugOutput" textarea (if available) and the console
    function logOutput(value) {
        var debugOutput = document.getElementById("debugOutput");
        if (debugOutput) {
            debugOutput.value += value + "\n\n";
            debugOutput.scrollTop = debugOutput.scrollHeight;
        }
        console.log(value);
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
{{< /highlight >}}

Anytime the code calls `logOutput`, the messages is displayed in the textarea as well as the web browser console. If the textarea component is removed, the program will continue writing messages to the web browser console.

##### Update assistant.js file

We are ready to add websocket functionality to the `bot-client/scripts/assistant.js` file:

{{< highlight javascript "hl_lines=2 40 49-79 81-87 89-102 141-142" >}}
window.onload = () => {
    var webSocket = null;

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
            msgBody = createElement("div", { "class": "msg-body" }),
            innerBody = createElement("div", { "class": "inner-body" }, msgBody),
            body = createElement("div", { "class": "body-wrapper" }, innerBody),
            userMsg = createElement("div", {
                "id": "user-msg",
                "class": "textareaElement",
                "placeholder": "Choose an option",
                "contenteditable": "false"
            }),
            footer = createElement("div", { "class": "footer" }, userMsg),
            aDialog = createElement("div", { "class": "chat" }, [header, body, footer]);

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
        openWSConnection();
    }

    // On close assistant dialog callback
    function onCloseDialog() {
        document.querySelector(".assistant .chat").style.display = "none";
        document.querySelector(".assistant button").style.display = "block";
    }

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
                var serverMsg = messageEvent.data;
                logOutput(`<== ${serverMsg}\n`);
            };

        } catch (exception) {
            logOutput(`error: ${JSON.stringify(exception)}`);
        }
    }

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

            logOutput(`==> ${content}`);
            chatBox.innerHTML = '';
        }
    }

    // Log output in the "debugOutput" textarea (if available) and the console
    function logOutput(value) {
        var debugOutput = document.getElementById("debugOutput");
        if (debugOutput) {
            debugOutput.value += value + "\n";
            debugOutput.scrollTop = debugOutput.scrollHeight;
        }
        console.log(value);
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

    // TODO: Remove after testing
    enableChatEditor();
};
{{< /highlight >}}

The changes are across different areas of the file. Let's review:
* **webSocket** variable to cache the websocket instance.
* **onWSOpenConnection** API to create connection and handle callbacks.
    * hooked-up in **onOpenDialog** to open the connection when the dialog is opened.
* **enableChatEditor** to allow user input.
    * temporarily hooked-up at the end of the file.
* **onEditorKeys** to capture user input and send message on websocket on _enter_ key.

The source code for **Step 3** is available in <a href="https://github.com/infinyon/fluvio-demo-apps-node/tree/master/bot-assistant/_blog/step3" target="_blank">github</a>.

### Test websocket Communication

We are leveraging Bot Assistant editor window to test our websocket communication. Ensure the [bot-server is up and running](#test-bot-server) and refresh `index.html` in the web browser.

Type **ping** in the editor window, and the server should respond with **pong**.

<img src="/blog/images/bot-assistant/ping-pong-test.svg"
     alt="Ping/Pong WebSocket Connection Example"
     style="justify: center; max-width: 800px" />

Congratulations, websocket communication is up and running.

## Step 4: Add session cookies

Bot assistant needs to support multiple conversations in parallel. To accomplish this, `bot-server` will generate a unique session id for each new conversation. The session id is saved in a `cookie` in the client Web Browser.

### Add session cookie to `bot-server`

The server needs to ability to generate an push a session cookie to the client. Let's update `ws-proxy.ts` file. 

{{< highlight typescript "hl_lines=3 5 21-24 34-43 46-47 50 64-78" >}}
import WS from "ws";
import http from "http";
import crypto from 'crypto';

const COOKIE_NAME = "Fluvio-Bot-Assistant"

export class WsProxy {
    private static _wss: WS.Server;

    constructor() {
        WsProxy._wss = new WS.Server({ clientTracking: false, noServer: true });
    }

    public init(server: http.Server) {
        this.onUpgrade(server);
        this.onConnection();
    }

    private onUpgrade(server: http.Server) {
        server.on("upgrade", function (request, socket, head) {
            const session = WsProxy.parseSessionFromCookie(request.headers.cookie);
            if (session) {
                request.headers.session = session;
            }

            WsProxy._wss.handleUpgrade(request, socket, head, function (ws: WS) {
                WsProxy._wss.emit("connection", ws, request);
            });
        });
    }

    private onConnection() {

        WsProxy._wss.on("headers", function (headers: Array<string>, req) {
            const session = WsProxy.parseSessionFromCookie(req.headers.cookie);

            if (!session) {
                let session = crypto.randomBytes(20).toString("hex");
                req.headers.session = session;

                headers.push("Set-Cookie: " + COOKIE_NAME + "=" + session);
            }
        });

        WsProxy._wss.on("connection", function (ws, req) {
            const session = req.headers.session;
            console.log(`session opened - ${session}`);

            ws.on("close", function () {
                console.log(`session closed - ${session}`);
            });

            ws.on("message", (clientMsg: string) => {
                console.log(`<== ${clientMsg}`);

                if (clientMsg == "ping") {
                    ws.send("pong");
                    console.log("==> pong");
                }
            });
        });
    }

    // Parse session from cookie
    private static parseSessionFromCookie(cookie?: string) {
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
}{{< /highlight >}}

We generate a cookie called **Fluvio-Bot-Assistant** and utilize WebSocket to send it to the client.

WebSocket connection setup hast two steps:
1. `headers` request
2. `connection` request. 

In `headers` callback, we check the HTTP headers for the session cookie. If not found, we'll generate a new session id and append the HTTP header. WebSocket will take care of the rest. In `connection` callback, we read the session id from `request.headers.session`.

Finally, we built `parseSessionFromCookie` API to grab the session id from the cookie header.

### Add web server support to `bot-client`

Web clients handle cookies through the HTTP protocol implemented by Web servers. Our `bot-client` is just a file, hence it lacks cookie support. We need to start a web server to server `assistant.js`.

#### Add node.js to `bot-client`

For consistency, we'll use *node.js* and *express* for the web server functionality in the client. Let's generate a new node project inside `bot-client` directory:

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

We'll add `client.js` in the root `bot-client` directory. The file will provision and instantiate a web server.

```bash
touch client.js
```

Paste the following content in `client.js` file:

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

The file provisions a web server on port 9999 and configures the routes for _scripts_, _styles_, and _images_ used by `assistant.js`. Then, it starts the web server which reads the default route and invokes `index.html`.

The source code for **Step 4** is available in <a href="https://github.com/infinyon/fluvio-demo-apps-node/tree/master/bot-assistant/_blog/step4" target="_blank">github</a>.
### Test Session Cookies

To test the code, let's start the web client.

```bash
npm run start:client
...
[nodemon] starting `node ./client.js`
listening http://localhost:9999
```

Assuming `bot-server` web server is also running, open a web browser and load `localhost:9999`. Click on Bot Assistant icon to generate a new connection. Then refresh the page to update cookie.

<img src="/blog/images/bot-assistant/client-cookies.svg"
     alt="Client Cookies"
     style="justify: center; max-width: 680px" />

Open the cookies in browser settings and notice a `Fluvio-Bot-Assistant` cookie. If you delete the cookie and refresh the page, a new session id is generated.

Congratulations, you have successfully added session cookies to Bot Assistant.


## Step 5: Add state machine

While there are many ways to drive a robot assistant, this project uses custom state machines. You may think of a state machine as a guided tour, where the bot sends the client a series of choices. Upon response, the bot looks-up the reply and in the state machine and iterates to the next state. Then the loop repeats until the end state is reached.

<img src="/blog/images/bot-assistant/state-machine.svg"
     alt="State Machine"
     style="justify: center; max-width: 600px;" />

The state machine definition is expressed in a JSON file that is described in detail in the next section.

### State machine definition

The state machine is composed of states, where the states can be:
* sendRequest
* matchResponse

The bot uses `sendRequest` to generate and send a message to the client, then it waits for the response. When the response arrives, the bot looks-up the `matchResponse` state to identify where it should resume. Each request/response pair has a unique identifier. The identifier is used to keep context during the client/server message exchange. Whereas the state machine has the overall end-to-end context.

Bot assistant generates one of the following `sendRequest` messages:

* **BotText** - sends information to the user (text or HTML format)
* **ChoiceRequest** - sends a list of options to the user. GroupId is the unique identifier used to pair this request with the *ChoiceResponse*
* **StartChatSession** - asks the client to enable chat editor. SessionId is the unique identifier and it used to pair it with *UserText*
* **EndChatSession** - ask the client to disable chat session. Uses the SessionId defined in the *StartChatSession*

The Client replies with one of the following `mathResponse` messages:

* **ChoiceResponse** - one or more responses with unique itemIds for the groupId matching a *ChoiceRequest*
* **UserText** - a text response with sessionId matching a *StartChatSession*

#### State transitions

The state transition have two types of flows:
* internal flows - driven by one or more _internal states_.
* external flows - driven by an _external state_. An external state tells the engine to generate a requests and wait for the response before resuming.

Internal states are states that have a `next` field. Whereas external states have a `sessionId` or `groupId` instead of the `next` field. Internal states can be chained, whereas external states cannot as they trigger a message exchange.

State transitions are triggered by a new connection or a client response. If it begins at an _internal state_, the engine collects the state information and moves to the next state until it encounters an _external state_. At that time, it generates a client request and waits for the response to resume.

<img src="/blog/images/bot-assistant/state-transitions.svg"
     alt="State Transitions"
     style="justify: center; max-width: 800px;" />

The client displays the request choices and asks the user to make a selection. Upon selection, the client generates a response and replies to the bot assistant.

Now that we have the state machine and the state transition definitions, let's create a custom state machine.

### Create state machine JSON file

Bot assistant defines custom state machines in JSON. We'll change the bot-server to accept the state machine in a command line parameter which allows the state machine file to reside anywhere in you local machine. 

For the purpose of this exercise, we'll add the custom state machine in the `bot-server` directory.


```bash
cd ../bot-server
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
            "question": "What programming language do you use in your hobby projects?",
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
        "next": "anyOtherChoices"
    },
    "langChoiceGo": {
        "matchResponse": {
            "kind": "ChoiceResponse",
            "groupId": "lang",
            "itemId": "go"
        },
        "next": "anyOtherChoices"
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
        "next": "anyOtherChoices"
    },
    "anyOtherChoices": {
        "sendRequest": {
            "kind": "ChoiceRequest",
            "groupId": "others",
            "question": "Any other?",
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
    "anyOtherYes": {
        "matchResponse": {
            "kind": "ChoiceResponse",
            "groupId": "others",
            "itemId": "yes"
        },
        "next": "langChoices"
    },
    "anyOtherNo": {
        "matchResponse": {
            "kind": "ChoiceResponse",
            "groupId": "others",
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

The state machine asks the user for his or her favorite programming language. It shows 3 options:
* Rust
* Go
* Other

If the user chooses `Rust` or `Go`, it sends the choice request `Try Again`. For `Other`, it runs a sequence of states:
* **sendRequest** to open an interactive session,
* **matchResponse** to captures the user response,
* **sendRequest** to close interactive the session
* **sendRequest** choice to try again.

This basic state machine show two different interaction models: a choice request/response or a user interaction. When the client receives a choice request, it presents the user with a series of choices. The user clicks on one of the choices and the client generates a response. For an interactive session, the client is asked to open an interactive session for the user to type his answer. After the server receives the response, it sends the client another request to close the interactive session. It is the responsibility of the server to manage access to the user editor.

### Load state machine

The state machine is managing the flow of information between the server and the clients. Implicitly, the information layer is describing the protocol format.

Our approach is to define two files to address both concerns:
1. `messages.ts` - defines the message types
2. `state-machine.ts` - defines the state machine workflow using messages as payload.

#### Create `messages.ts` file

Let's add `messages.ts` file inside `bot-server/src` directory:

```bash
touch src/messages.ts
```

Paste the following message definitions:

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

RequestMessage and ResponseMessage are defined as unions of different message types. The actual definition is described in the previous section.

Next we'll crate the state machine.
#### Create `state-machine.ts` file

The state machine defines the state workflow. Create `state-machine.ts` file inside `bot-server/src` directory:

```bash
touch src/state-machine.ts
```

Paste the following code in the state-machine file:

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

#### Initialize state machine

The state machine is initialize in the `bot-server.ts`. Bot assistant can run with any state machine file, so it makes sense to load json files from the command line.

Let's change the code to read a json file and load its content into the state machine:

{{< highlight typescript "hl_lines=4 24-27 30-37" >}}
import http from "http";
import express from "express";
import { WsProxy } from "./ws-proxy";
import { StateMachine, loadStateMachine } from "./state-machine";

const PORT = 9998;

// Provision Bot Assistant server
const startServer = async () => {
    const app = express();
    const Server = http.createServer(app);

    // Attach websocket to server
    const wsProxy = new WsProxy();
    wsProxy.init(Server);

    // Start server
    Server.listen(PORT, () => {
        console.log(
            `started bot assistant server at http://localhost:${PORT}...`
        );
    });

    // Initialize state machine
    let filePath = getFileName();
    const stateMachine: StateMachine = loadStateMachine(filePath);
    console.log(stateMachine);
};

// read state machine file from command line
function getFileName() {
    if (process.argv.length != 3) {
        console.log("Usage: node bot-server.js <state-machine.json>");
        process.exit(1);
    }
    return process.argv[2];
}

// Start Server
startServer();
{{< /highlight >}}

The program retrieves the file name, from the command line, creates a state machine variable and displays the result in the console.

The source code for **Step 5** is available in <a href="https://github.com/infinyon/fluvio-demo-apps-node/tree/master/bot-assistant/_blog/step5" target="_blank">github</a>.

### Test state machine initialization

To test the state machine, we need to restart the server with a command line parameter. Mpm uses environment variables, which we'll define at the command line using PARAMS definition:

```bash
PARAMS=state-machine.json npm run start:server
```

The code should read the state machine and display it at the command line:

```bash
12:11:29 PM - Found 0 errors. Watching for file changes.
Map(9) {
  'greetings' => {
    sendRequest: { kind: 'BotText', content: "Hi, I'm Bot! Nice to meet you." },
    next: 'langChoices'
  },
  ....
}
```

Congratulations, your state machine parser is up and running.


## Step 6: Add workflow controller

Workflow controller is the mediator between the websocket proxy and the state machine. The controller receives messages from the client, computes the next state, generates a reply, and sends a response.

<img src="/blog/images/bot-assistant/workflow-controller.svg"
     alt="Workflow Controller"
     style="justify: center; max-width: 800px;" />

To implement the controller functionality we need to make the following code changes to `bot-server`:
* **messages.ts**: to include message header for client/server communication.
* **ws-proxy.ts**: to produce events on client messages.
* **workflow-controller.ts**: to add new file to coordinate message flow.
* **bot-server.ts**: to initialize workflow controller.

We also need to update `assistant.js` file in `bot-client` to display server requests and forward user responses.

### Update `messages.ts` file

When the state machine messages are sent between the client and the server, they need header information.

Let's append `messages.ts` file with the following changes:

{{< highlight typescript "hl_lines=1-2 4-9 11-13 15-18 20-23 79-85 87-94 96-103 105-108 110-115" >}}
export type TimeStamp = string;
export type SID = string;

/* Message Header */
export interface Message {
    sid: SID;
    payload?: Payload;
    timestamp: TimeStamp;
}

export type Payload =
    | Request
    | Response;

export interface Request {
    kind: "Request",
    message: RequestMessage,
}

export interface Response {
    kind: "Response",
    message: ResponseMessage,
}

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

/* Build an initialization message */
export function buildInitMessage(sid: SID) {
    return <Message>{
        sid: sid,
        timestamp: getDateTime(),
    };
};

/* Append header to a request message */
export function buildRequest(sid: SID, message: RequestMessage) {
    return <Message>{
        sid: sid,
        payload: <Request>{ kind: "Request", message: message },
        timestamp: getDateTime(),
    };
};

/* Append header to a response message */
export function buildResponse(sid: SID, message: ResponseMessage) {
    return <Message>{
        sid: sid,
        payload: <Response>{ kind: "Response", message: message },
        timestamp: getDateTime(),
    };
};

/* Returns true if Request message, false otherwise */
export function isRequest(payload?: Payload) {
    return (payload) ? (payload.kind == "Request") : false;
}

/* Generate timestamp */
function getDateTime() {
    return new Date(Date.now() - new Date().getTimezoneOffset() * 60000)
        .toISOString()
        .slice(0, -1);
}
{{< /highlight >}}

At the top of the file, we added a message header for the session id, payload type and timestamp. At the bottom, we implemented a series of helper APIs: 
* **buildInitMessage**: creates a message without a payload that indicates a new connection.
* **buildRequest**: creates a message of `Request` kind.
* **buildResponse**: creates a message of `Response` kind.
* **isRequest**: checks if the message is of `Request` kind.
* **getDateTime**: generates a timestamp.

### Update `ws-proxy.ts` file

The websocket will communicate with the state machine through a workflow controller that we'll create in the next section. In the meantime, we need to create a message exchange mechanism between websocket and other components. We'll use node's events facility to publish events for connection requests and client messages. 

Let's update `src/ws-proxy.ts` file:

{{< highlight typescript "hl_lines=1 5 11 15 50-56 61 67 72-81 100-110" >}}
import { EventEmitter } from "events";
import WS from "ws";
import http from "http";
import crypto from 'crypto';
import { SID } from './messages';

const COOKIE_NAME = "Fluvio-Bot-Assistant"

export class WsProxy {
    private static _wss: WS.Server;
    private static _sessions: Map<SID, WS>;

    constructor() {
        WsProxy._wss = new WS.Server({ clientTracking: false, noServer: true });
        WsProxy._sessions = new Map();
    }

    public init(server: http.Server) {
        this.onUpgrade(server);
        this.onConnection();
    }

    private onUpgrade(server: http.Server) {
        server.on("upgrade", function (request, socket, head) {
            const session = WsProxy.parseSessionFromCookie(request.headers.cookie);
            if (session) {
                request.headers.session = session;
            }

            WsProxy._wss.handleUpgrade(request, socket, head, function (ws: WS) {
                WsProxy._wss.emit("connection", ws, request);
            });
        });
    }

    private onConnection() {

        WsProxy._wss.on("headers", function (headers: Array<string>, req) {
            const session = WsProxy.parseSessionFromCookie(req.headers.cookie);

            if (!session) {
                let session = crypto.randomBytes(20).toString("hex");
                req.headers.session = session;

                headers.push("Set-Cookie: " + COOKIE_NAME + "=" + session);
            }
        });

        WsProxy._wss.on("connection", function (ws, req) {
            const session_hdr = req.headers.session;
            const session = ((Array.isArray(session_hdr)) ? session_hdr[0] : session_hdr) || "";
            console.log(`session opened - ${session}`);

            WsProxy._sessions.set(session, ws);

            wsProxyEvents.emit(wsProxyEvents.CONNECTION, session);

            ws.on("close", function () {
                console.log(`session closed - ${session}`);

                WsProxy._sessions.delete(session);
            });

            ws.on("message", (clientMsg: string) => {
                console.log(`<== ${clientMsg}`);

                wsProxyEvents.emit(wsProxyEvents.MESSAGE, session, clientMsg);
            });
        });
    }

    // Send message to client
    public sendMessage(session: string, clientMsg: string) {
        const ws = WsProxy._sessions.get(session);
        if (!ws) {
            return;
        }

        console.log(`==> ${clientMsg}`);
        ws.send(clientMsg);
    }

    // Parse session from cookie
    private static parseSessionFromCookie(cookie?: string) {
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
}

/* WebSocket Proxy Event Emitter */
class WsProxyEvents extends EventEmitter {
    readonly CONNECTION = 'WebSocket-Connection';
    readonly MESSAGE = 'WebSocket-Message';

    private static _instance = new WsProxyEvents();
    static get instance() {
        return this._instance;
    }
}
export const wsProxyEvents = WsProxyEvents.instance;
{{< /highlight >}}

The code changes are as follows:
* **sessions** variable to cache the session id with the websocket connection. The session id becomes the identifier of the client for return messages.
    * **session.set** to append a session on new connections.
    * **session.delete** to remove the session on closed connections.
* **emit** to dispatch a message on new connections and client responses.
* **sendMessage** to look-up the session and send a message to the client.

We also extended `EventEmitter` to create a customer **WsProxyEvent** to emit the following types:
* **CONNECTION** - for new connections.
* **MESSAGE** - for client messages.

### Add `workflow-controller.ts` file

The workflow controller is the coordinator between the client and the state machine. It listens for websocket messages, invokes the state machine to produce the next message, and calls the websocket to inform the client.

Let's add the file:

```bash
touch src/workflow-controller.ts
```

Paste the following content in the `src/workflow-controller.ts` file:

```ts
import {
    SID,
    ResponseMessage,
    ChoiceResponse,
    UserText,
    buildRequest,
} from "./messages";
import { StateMachine, State } from "./state-machine";
import { WsProxy, wsProxyEvents } from "./ws-proxy";

export class WorkflowController {
    private static _stateMachine: StateMachine;
    private static _initState: string;
    private _wsProxy: WsProxy;

    init(stateMachine: StateMachine, wsProxy: WsProxy) {
        this._wsProxy = wsProxy;

        this.listenForEvents();

        WorkflowController._stateMachine = stateMachine;
        WorkflowController._initState = stateMachine.keys().next().value;
    }

    processNewConnection(sid: SID) {
        const nextStates = this.getInit();

        nextStates.forEach(state => {
            if (state.sendRequest) {
                const request = buildRequest(sid, state.sendRequest);
                const message = JSON.stringify(request.payload?.message);
                this._wsProxy.sendMessage(sid, message);
            }
        })
    }

    processClientMessage(sid: SID, clientMsg: string) {
        const message: ResponseMessage = JSON.parse(clientMsg);

        const nextStates = this.getNext(message);
        nextStates.forEach(state => {
            if (state.sendRequest) {
                const request = buildRequest(sid, state.sendRequest);
                const message = JSON.stringify(request.payload?.message);
                this._wsProxy.sendMessage(sid, message);
            }
        });
    }

    private getInit() {
        return this.processNext(WorkflowController._initState);
    }

    private getNext(response: ResponseMessage) {
        var state: string = this.getState(response);

        return this.processNext(state);
    }

    private getState(response: ResponseMessage) {
        switch (response.kind) {
            case "ChoiceResponse": {
                return this.getChoiceResponseState(response);
            }
            case "UserText": {
                return this.getUserTextState(response);
            }
        }
    }

    private processNext(startState: string) {
        var nextStates: State[] = [];

        var state = WorkflowController._stateMachine.get(startState);
        while (state) {
            nextStates.push(state);

            const next = state.next || "";
            state = WorkflowController._stateMachine.get(next);
            if (next.length > 0 && !state) {
                console.error(`Error: Cannot find next state: ${next}`);
            }
        }

        return nextStates;
    }

    private getChoiceResponseState(choiceResponse: ChoiceResponse) {
        for (let [key, state] of WorkflowController._stateMachine.entries()) {
            if (state.matchResponse &&
                state.matchResponse.kind == choiceResponse.kind &&
                state.matchResponse.groupId == choiceResponse.groupId &&
                state.matchResponse.itemId == choiceResponse.itemId) {
                return key;
            }
        }

        console.error(`Error: cannot find choice ${JSON.stringify(choiceResponse)}`);
        return WorkflowController._initState;
    }

    private getUserTextState(userText: UserText) {
        for (let [key, state] of WorkflowController._stateMachine.entries()) {
            if (state.matchResponse &&
                state.matchResponse.kind == "UserText" &&
                state.matchResponse.sessionId == userText.sessionId) {
                return key;
            }
        }

        console.error(`Error: cannot find user session ${JSON.stringify(userText)}`);
        return WorkflowController._initState;
    }

    private listenForEvents() {
        wsProxyEvents.on(
            wsProxyEvents.CONNECTION,
            (sid: SID) => {
                this.processNewConnection(sid);
            }
        );

        wsProxyEvents.on(
            wsProxyEvents.MESSAGE,
            async (sid: SID, clientMsg: string) => {
                this.processClientMessage(sid, clientMsg);
            }
        );
    }
}
```

The workflow controller has the following public functions:
* **init** caches an instance of the `WsProxy`, initializes the state machine, and registers a listener for websocket messages.
* **processNewConnection** reads the state machine from the first state and produces a request.
* **processClientMessage** parses the client response, looks-up the resume state and produces the next request.

The private APIs help the controller identify the response type and traverse the state machine to generate subsequent requests.

### Update `bot-server.ts` file

We need to hook-up workflow controller in the `bot-server.ts` file. Let's update the file:

{{< highlight typescript "hl_lines=5 28-29" >}}
import http from "http";
import express from "express";
import { WsProxy } from "./ws-proxy";
import { StateMachine, loadStateMachine } from "./state-machine";
import { WorkflowController } from "./workflow-controller";

const PORT = 9998;

// Provision Bot Assistant server
const startServer = async () => {
    const app = express();
    const Server = http.createServer(app);

    // Attach websocket to server
    const wsProxy = new WsProxy();
    wsProxy.init(Server);

    // Start server
    Server.listen(PORT, () => {
        console.log(
            `started bot assistant server at http://localhost:${PORT}...`
        );
    });

    // Initialize state machine
    let filePath = getFileName();
    const stateMachine: StateMachine = loadStateMachine(filePath);
    const workflowController = new WorkflowController();
    workflowController.init(stateMachine, wsProxy);
};

// read state machine file from command line
function getFileName() {
    if (process.argv.length != 3) {
        console.log("Usage: node bot-server.js <state-machine.json>");
        process.exit(1);
    }
    return process.argv[2];
}

// Start Server
startServer();
{{< /highlight >}}

The code creates a `workflowController` variable and calls its `init` function.


#### Test workflow controller initialization

We are ready to test the initial step of the workflow. The code has been automatically updated by `ts-watch` and should be ready to run.

Let's open the web browser to `http://localhost:9999/`, then click on "Bot Assistant` button. 

The client initiates a new connection and the workflow controller responds with the following messages:

<img src="/blog/images/bot-assistant/client-workflow-init.svg"
     alt="Client Workflow Initial Message"
     style="justify: center; max-width: 700px" />

Congratulations, `Workflow Controller` is up and running. Next we'll update the client to participate in the state machine negotiation.

### Update client `assistant.js`

Our initial assistant implementation was focused on simple navigation capabilities. Next, we need to add support for the workflow protocol. 

Let's update the code in `bot_client/scripts/assistant.js`:

{{< highlight javascript "hl_lines=74 82-107 109-120 122-135 137-146 148-167 169-179 181-191 193-204 207-210 214 219-226 228-232 242-247" >}}
window.onload = () => {
    var webSocket = null;

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
            msgBody = createElement("div", { "class": "msg-body" }),
            innerBody = createElement("div", { "class": "inner-body" }, msgBody),
            body = createElement("div", { "class": "body-wrapper" }, innerBody),
            userMsg = createElement("div", {
                "id": "user-msg",
                "class": "textareaElement",
                "placeholder": "Choose an option",
                "contenteditable": "false"
            }),
            footer = createElement("div", { "class": "footer" }, userMsg),
            aDialog = createElement("div", { "class": "chat" }, [header, body, footer]);

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
        openWSConnection();
    }

    // On close assistant dialog callback
    function onCloseDialog() {
        document.querySelector(".assistant .chat").style.display = "none";
        document.querySelector(".assistant button").style.display = "block";
    }

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
                var serverMsg = messageEvent.data;
                logOutput(`<== ${serverMsg}`);
                onMessageFromServer(serverMsg);
            };

        } catch (exception) {
            logOutput(`error: ${JSON.stringify(exception)}`);
        }
    }

    // On messages received from Websocket
    function onMessageFromServer(value) {
        const message = JSON.parse(value);
        switch (message.kind) {
            case "BotText":
                showBotText(message.content);
                break;
            case "UserText":
                showUserText(message.content);
                break;
            case "ChoiceRequest":
                showBotText(message.question);
                showChoiceButtons(message.groupId, message.choices);
                break;
            case "ChoiceResponse":
                choicesToButton(message.groupId, message.content);
                break;
            case "StartChatSession":
                sessionId = message.sessionId;
                enableChatEditor(message.chatPrompt, message.chatText);
                break;
            case "EndChatSession":
                disableChatEditor();
                break;
        };
    }

    // Send a message on WebSocket
    function sendWsMessage(message) {
        if (webSocket.readyState != WebSocket.OPEN) {
            logOutput("WebSocket is not connected: " + webSocket.readyState);
            return;
        }

        const msgObj = JSON.stringify(message)
        logOutput(`==> ${msgObj}`);

        webSocket.send(msgObj);
    }

    // Show text from bot assistant
    function showBotText(content) {
        if (content.length > 0) {
            removeDuplicateAvatar("bot");

            var img = createElement("img", { "src": `img/assistant/bot.svg` }),
                avatar = createElement("div", { "class": "avatar", "id": "bot" }, img),
                msg = createElement("div", { "class": "msg" }, content),
                msgLeft = createElement("div", { "class": "msg-left" }, [msg, avatar]);

            document.querySelector(".msg-body").appendChild(msgLeft);
            scrollToBottom(".inner-body");
        }
    }

    // Show text from user interactive session
    function showUserText(content) {
        if (content.length > 0) {
            var msg = createElement("div", { "class": "msg" }, content),
                msgLeft = createElement("div", { "class": "msg-right" }, msg);

            document.querySelector(".msg-body").appendChild(msgLeft);
            scrollToBottom(".inner-body");
        }
    }

    // Show choices
    function showChoiceButtons(groupId, choices) {
        if (choices.length > 0) {
            var buttons = [];

            choices.forEach(choice => {
                var button = createElement("div", { "class": "button" }, choice.content);
                button.addEventListener('click', function () {
                    pickChoice(groupId, choice.itemId, choice.content);
                }, false);

                buttons.push(createElement("div", { "class": "btn" }, button));
            });

            var msgLeft = createElement("div", { "class": "msg-left", "id": groupId }, buttons);

            document.querySelector(".msg-body").appendChild(msgLeft);
            scrollToBottom(".inner-body");
        }
    }

    // Callback invoked on user selection
    function pickChoice(groupId, itemId, content) {
        choicesToButton(groupId, content);

        sendWsMessage({
            kind: "ChoiceResponse",
            groupId: groupId,
            itemId: itemId,
            content: content,
        });
    }

    // Swap choices with a button representing the selection
    function choicesToButton(groupId, content) {
        document.getElementById(groupId).remove();

        var button = createElement("div", { "class": "button selected" }, content),
            btn = createElement("div", { "class": "btn" }, button),
            msgRight = createElement("div", { "class": "msg-right" }, btn);

        document.querySelector(".msg-body").appendChild(msgRight);
        scrollToBottom(".inner-body");
    }

    // On multiple bot messages, ensure avatar is only displayed on last entry
    function removeDuplicateAvatar(id) {
        var messages = document.querySelector('.msg-body').children;
        if (messages.length > 0) {
            var lastMessage = messages[messages.length - 1];
            if (lastMessage.getAttribute("class") === 'msg-left') {
                if (lastMessage.lastChild.id == id) {
                    lastMessage.removeChild(lastMessage.lastChild);
                }
            }
        }
    }

    // Make editor section editable
    function enableChatEditor(chatPrompt, chatText) {
        if (chatText) {
            showBotText(chatText);
        }

        var chatBox = document.getElementById("user-msg");
        chatBox.setAttribute("contenteditable", true);
        chatBox.setAttribute("placeholder", chatPrompt || "Type question here ...");

        chatBox.addEventListener("keydown", onEditorKeys, false);
    }

    // Disable interactive chat
    function disableChatEditor() {
        var chatBox = document.getElementById("user-msg");
        chatBox.addEventListener("keydown", {}, false);

        chatBox.setAttribute("contenteditable", false);
        chatBox.setAttribute("placeholder", "Choose an option");
    }

    // Scroll to last messages
    function scrollToBottom(tag) {
        var div = document.querySelector(tag);
        div.scrollTop = div.scrollHeight - div.clientHeight;
    }

    // Callback on chat editor user input (key press)
    function onEditorKeys(e) {
        var chatBox = document.getElementById("user-msg");

        if (e.code == 'Enter' && chatBox.textContent.length > 0) {
            e.preventDefault();

            const content = chatBox.textContent;
            sendWsMessage({
                kind: "UserText",
                sessionId: sessionId,
                content: content,
            });
            showUserText(content);

            chatBox.innerHTML = '';
        }
    }

    // Log output in the "debugOutput" textarea (if available) and the console
    function logOutput(value) {
        var debugOutput = document.getElementById("debugOutput");
        if (debugOutput) {
            debugOutput.value += value + "\n\n";
            debugOutput.scrollTop = debugOutput.scrollHeight;
        }
        console.log(value);
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
{{< /highlight >}}

The code implements Bot assistant protocol and the user interactions choice buttons and custom messages. Let's review the changes:

* `onMessageFromServer` - parses messages from the server and processes them based on their kind:
    * `BotText` - shows bot text in the chat box.
    * `UserText` - shows user text in the chat box.
    * `ChoiceRequest` - displays the choices in  the chat box.
    * `ChoiceResponse` - swaps choices with a button based on user selection.
    * `StarChatSession` - enables chat editor
    * `EndChatSession` - disables chat editor
* `wsSendMessage` - send a message to the server
* `removeDuplicateAvatar` - ensures Bot avatar is only displayed in the last entry
* `scrollToBottom` - makes the last entry in the chat box visible.

All other APIs are supporting functions to implement the workflow.

Next, let's hook-up the function to Ws connection:


### Update `assistant.css`

Assistant file manipulates DOM objects based on class names. Let's update `assistant.css` to add classes for the bot and user messages.

Open `bot-client/css/assistant.css` file and and update as follows:

{{< highlight css "hl_lines=89-92 94-97 99-110 112-118 120-129 131-133 135-140 142-149 151-153 155 157-161 163-171 173-176 178-182" >}}
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

/* Assistant - Chat Box */

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
	max-height: calc(100vh - 300px);
	overflow: auto;
	overflow-x: hidden;
}

.assistant .msg-body {
	font-size:12px;
	padding: 10px 10px 5px 5px;
}

.assistant .msg-left{
	margin-bottom:7px;
	word-break: break-all;
}

.assistant .msg-left .avatar {
	width: 50px;
	margin-top: -40px;
}

.assistant .msg-left .operator {
	margin-top: -40px;
	padding: 1px;
	font-size:1.6em;
	width:35px;
	height:35px;
	line-height:1.8em;
	text-align:center;
	border-radius:50%;
	background:plum;
	color:white;
}

.assistant .msg-left img {
	width:35px;
	height:35px;
	border-radius:50%;
	background:#bbb;
	border: 1px solid #eee;
}

.assistant .msg-left .msg {
	background:#f2f2f2;
	padding:10px;
	min-height:15px;
	margin: 3px;
	border: 1px solid #ddd;
	border-radius:7px;
	margin-left: 44px;
	margin-right: 30px;
}

.assistant .msg-left .button {
	margin: -2px 30px 7px 50px;
}

.assistant .msg-right {
	position: relative;
	right: 0px;
	margin: 3px;
	margin-bottom:10px;
}

.assistant .msg-right .msg {
	background:#d4e7fa;
	padding:10px;
	min-height:15px;
	margin-left: 80px;
	border-radius:7px;
	word-break: break-all;
}

.assistant .msg-right .button {
	float: right;
}

/* button  */

.assistant .btn {
	display: inline-block;
	margin: 2px;
	width: 100%;
}

.assistant .button {
	width: max-content;
	border-radius:15px;
	padding: 10px 15px;
	transition-duration: 0.2s;
	background-color: white;
	color: #006687;
	border: 1px solid #008CBA;
}

.assistant .button.selected {
	background-color: #008CBA;
	color: white;
}
  
.assistant .button:hover {
	cursor: pointer;
	background-color: #008CBA;
	color: white;
}

/* footer  */

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
{{< /highlight >}}

The source code for **Step 6** is available in <a href="https://github.com/infinyon/fluvio-demo-apps-node/tree/master/bot-assistant/_blog/step6" target="_blank">github</a>.

#### Test end-to-end workflow

Let's refresh the web browser to `http://localhost:9999/`, then click on "Bot Assistant` button. 
Click on the choices and see the bot assistant traverse through our state machine:

<img src="/blog/images/bot-assistant/workflow-end-to-end.svg"
     alt="Workflow end-to-end"
     style="justify: center; max-width: 700px" />

Congratulations, `Workflow Controller` is fully functional.


## Step 7: Add data streaming and persistency

Bot Assistant works well but it is still limited its usefulness. When the website is refreshed all previous messages are lost.

We use [Fluvio](https://fluvio.io) to remediate this issue. Fluvio is a high throughput, low latency data streaming platform that scales horizontally to handle persistency for a large number of concurrent messages. 
 
We deploy Fluvio between connection proxy and workflow controller which also enables us to divide our monolith into microservices. We divide the code into two independent services:
* Proxy Service - responsible for client connections
* Workflow Service - responsible for workflow management

<img src="/blog/images/bot-assistant/architecture.svg"
     alt="Bot Assistant Architecture"
     style="justify: center; max-width: 780px" />

The new architecture give us additional flexibility for:

* **scale** the proxy and workflow independently of each other. 

* **handoff** a conversation to a human operator. We can do that by adding an `operator service` independently that interacts directly with the client through the data stream.

* **add-on services** such as: analytics, machine learning, or connectors to other products.

-> **Prerequisites:** This section assumes you have access to a Fluvio cluster. Step-by-step instructions on setting-up Fluvio are available in [Quick Start](/docs/getting-started/fluvio-cloud/).

To integrate Fluvio data streaming we'll make the following code changes:
1. [Restructure code along service lines](#restructure-code-along-service-lines)
2. [Install fluvio-lib package](#install-fluvio-lib-package)
3. [Add fluvio-lib to proxy-service](#add-fluvio-libts-to-proxy-service)
4. [Add streaming-controller to proxy-service](#add-streaming-controllerts-to-proxy-service)
5. [Add fluvio-lib to workflow-service](#add-fluvio-libts-to-workflow-service)
6. [Integrate fluvio with workflow-controller](#integrate-fluvio-with-workflow-controller)
7. [Update bot-server initialization](#update-bot-serverts-initialization)
8. [Add session handling to bot-client](#add-session-handling-to-bot-client)

### Restructure code along service lines

As shown in the diagram above, we'll divide the bot server code into separate services: `proxy-service` and `workflow-service`.

```bash
cd ../bot-server/src
mkdir {proxy-service,workflow-service}
mv ws-proxy.ts proxy-service/
mv {workflow-controller.ts,state-machine.ts} workflow-service/
```

After movement we'll end up with the following file layout in `bot-server/src` directory:

```bash
tree
.
├── bot-server.ts
├── messages.ts
├── proxy-service
│   └── ws-proxy.ts
└── workflow-service
    ├── state-machine.ts
    └── workflow-controller.ts
```

Shared files, _bot-server.ts_ and _messages.ts_, are left at the top level, others are divided along service boundaries. 

-> Make sure all **import** statements impacted by file movement are updated and the code continues to compile and run the same as before.

### Install `fluvio-lib` package

Fluvio has a node native library <a href="https://www.npmjs.com/package/@fluvio/client">@fluvio/client</a> in **npm**. Let's install in `bot-server` directory:

```bash
npm install @fluvio/client
...
added 2 packages from 1 contributor and audited 93 packages in 4.846s
found 0 vulnerabilities
```

In the current implementation, `workflow-controller` accesses APIs in proxy service directory, which breaks modularity. After the Fluvio integration, we can decouple such interdependencies, which enables us to run each services in its own binary. Fluvio allows the two services to run independently on servers anywhere in the world.

Next we'll add a fluvio library file to each of the two services.

### Add `fluvio-lib.ts` to proxy-service

We implement a Fluvio class to handles data streaming requirements for the proxy service. The class intermediates incoming topic messages through an emitter called FLUVIO_MESSAGE.

Let's create `fluvio-lib.ts` file inside proxy-service:

```bash
touch ./src/proxy-service/fluvio-lib.ts
```

Paste the following code in `./src/proxy-service/fluvio-lib.ts` file:

```typescript
import { EventEmitter } from "events";
import Fluvio, { Offset, OffsetFrom } from '@fluvio/client';

// Fluvio Library
export class FluvioLib {
    private _fluvio: Fluvio;
    private _topicName: string;

    public async init(topicName: string) {
        this._topicName = topicName;
        this._fluvio = new Fluvio();

        await this._fluvio.connect();
        await this.createTopicIfNotFound(topicName);
    }

    public async produceMessage(msg: string) {
        const producer = await this._fluvio.topicProducer(this._topicName);
        producer.sendRecord(msg, 0);
    }

    public async fetchMessages() {
        this._fluvio = new Fluvio();
        await this._fluvio.connect();

        const consumer = await this._fluvio.partitionConsumer(this._topicName, 0)
        const offset: Offset = new Offset()

        const fetched = await consumer.fetch(offset);
        if (fetched) {
            fetched.records.batches.forEach(batch => {
                batch.records.forEach(record => {
                    fluvioEvents.emit(
                        fluvioEvents.FLUVIO_MESSAGE,
                        record.value
                    );
                });
            });
        }

        console.log(`proxy: fetched ${fetched.highWatermark} messages`);
    }

    public async startConsumerStream() {
        const consumer = await this._fluvio.partitionConsumer(this._topicName, 0);
        const offset: Offset = new Offset({ from: OffsetFrom.End, index: 0 })

        console.log('proxy: listening for events ... ');

        consumer.stream(offset, (record: string) => {
            fluvioEvents.emit(
                fluvioEvents.FLUVIO_MESSAGE,
                record
            );
        })
    }

    private async createTopicIfNotFound(topicName: string) {
        const admin = await this._fluvio.admin();
        const topic = await admin.findTopic(topicName);

        if (!topic) {
            await admin.createTopic(topicName);
            console.log(`topic: '${topicName}' created`);
            await this.sleep(2000);
        }
    }

    private async sleep(ms: number) {
        return new Promise((resolve) => {
            setTimeout(resolve, ms)
        })
    }
}

/* Fluvio Event Emitter */
class FluvioEvents extends EventEmitter {
    readonly FLUVIO_MESSAGE = 'Fluvio-Message';

    private static _instance = new FluvioEvents();
    static get instance() {
        return this._instance;
    }
}
export const fluvioEvents = FluvioEvents.instance;
```

Let's review the class routines:
* **init** establishes a new connection to fluvio and creates the topic.
* **produceMessage** send a message to fluvio.
* **fetchMessages** reads all messages in the topic and emits them in FLUVIO_MESSAGE
* **startConsumerStream** opens a stream and emits all messages on FLUVIO_MESSAGE
* **createTopicIfNoFound** searches for the topic and it creates one if it does not exist.
* **sleep** pauses the flow for a number of milliseconds.

We also extended EventEmitter to emit fluvio messages.

Next we'll add a streaming controller to intermediate messages between Fluvio and the websocket.

#### Add `streaming-controller.ts` to proxy-service

Streaming controller is the coordinator of the fluvio data streaming messages. It creates the topic, caches messages, and intermediates all exchanges between the websocket and Fluvio.

Create `fluvio.ts` file inside proxy-service:

```bash
touch ./src/proxy-service/streaming-controller.ts
```

Paste the following code into the file:

```typescript
import { WsProxy, wsProxyEvents } from "./ws-proxy";
import { FluvioLib, fluvioEvents } from "./fluvio-lib";
import { Message, SID, buildInitMessage, buildResponse, isRequest } from "../messages";

type Messages = Array<Message>;

export class StreamingController {
    private _dataStreams: Map<SID, Messages>;
    private _fluvio: FluvioLib;
    private _wsProxy: WsProxy;

    constructor() {
        this._dataStreams = new Map();
        this._fluvio = new FluvioLib();
    }

    public async init(topicName: string, wsProxy: WsProxy) {
        this._wsProxy = wsProxy;

        this.listenForEvents();

        await this._fluvio.init(topicName);
        console.log("after topic add");
        await this._fluvio.fetchMessages();
        console.log("after fetch");
        await this._fluvio.startConsumerStream();

        this.show();
    }

    public async messageFromClient(sid: SID, clientMsg: string) {
        const response = JSON.parse(clientMsg);
        const message = buildResponse(sid, response);

        await this._fluvio.produceMessage(JSON.stringify(message));
    }

    public sendToClient(message: Message) {
        if (message.payload) {
            const clientMessage = message.payload.message;
            this._wsProxy.sendMessage(message.sid, JSON.stringify(clientMessage));
        }
    }

    private append(message: Message) {
        const sid = message.sid;
        var messages = this._dataStreams.get(sid);
        if (!messages) {
            messages = new Array();
        }
        messages.push(message);
        this._dataStreams.set(sid, messages);
    }

    private show() {
        let table = new Map();
        for (let [key, value] of this._dataStreams) {
            table.set(key, value.length);
        }
        console.table(table, ["SID", "Messages"]);
    }

    private listenForEvents() {

        fluvioEvents.on(
            fluvioEvents.FLUVIO_MESSAGE,
            (msgObj: string) => {
                const message: Message = JSON.parse(msgObj);
                this.append(message);
                if (isRequest(message.payload)) {
                    this.sendToClient(message);
                }
            }
        );

        wsProxyEvents.on(
            wsProxyEvents.CONNECTION,
            async (sid: SID) => {
                const messages = this._dataStreams.get(sid);
                if (messages) {
                    messages.forEach(message => {
                        this.sendToClient(message);
                    });
                } else {
                    const message = buildInitMessage(sid);
                    await this._fluvio.produceMessage(JSON.stringify(message));
                }
            }
        );

        wsProxyEvents.on(
            wsProxyEvents.MESSAGE,
            async (sid: SID, clientMsg: string) => {
                const response = JSON.parse(clientMsg);
                const message = buildResponse(sid, response);

                await this._fluvio.produceMessage(JSON.stringify(message));
            }
        );
    }
}
```

Aside from coordination, streaming controller also builds a memory map of the message exchanges anchored by a session id. The memory map accelerates the playback of messages in the event of a connection reset.

Let's review the streaming controller routines:
* **init** connects with fluvio, fetches all messages to build a memory map, starts the consumer, and initializes event listeners.
* **messageFromClient** appends messages with headers and writes them to the fluvio topic.
* **sendToClient** strips message headers and sends the payload to websocket proxy.
* **append** adds a messages to the memory map
* **show** dumps the memory map to the screen
* **listenForEvents** proxies the messages on behalf of both event emitters:
    * fluvioEvents.FLUVIO_MESSAGE
    * wsProxyEvents.CONNECTION
    * wsProxyEvents.MESSAGE

Congratulations! Proxy service code changes are done. Let's update workflow service next.

### Add `fluvio-lib.ts` to workflow-service

The `fluvio-lib.ts` file in the workflow service is similar to `fluvio-lib.ts` the proxy service. The reason for a separate files is to keep the services decoupled.

Let's create `fluvio-lib.ts` file inside proxy-service:

```bash
touch ./src/workflow-service/fluvio-lib.ts
```

Paste the following code in `./src/workflow-service/fluvio-lib.ts` file:

```typescript
import { EventEmitter } from "events";
import Fluvio, { Offset, OffsetFrom } from '@fluvio/client';

// Fluvio Library
export class FluvioLib {
    private _fluvio: Fluvio;
    private _topicName: string;

    public async init(topicName: string) {
        this._topicName = topicName;
        this._fluvio = new Fluvio();

        await this._fluvio.connect();
    }

    public async produceMessage(msg: string) {
        const producer = await this._fluvio.topicProducer(this._topicName);
        producer.sendRecord(msg, 0);
    }

    public async startConsumerStream() {
        const consumer = await this._fluvio.partitionConsumer(this._topicName, 0);
        const offset: Offset = new Offset({ from: OffsetFrom.End, index: 0 })

        console.log('workflow: listening for events ... ');

        consumer.stream(offset, (record: string) => {
            fluvioEvents.emit(
                fluvioEvents.FLUVIO_MESSAGE,
                record
            );
        })
    }
}

/* Fluvio Event Emitter */
class FluvioEvents extends EventEmitter {
    readonly FLUVIO_MESSAGE = 'Fluvio-Message';

    private static _instance = new FluvioEvents();
    static get instance() {
        return this._instance;
    }
}
export const fluvioEvents = FluvioEvents.instance;
```

Workflow controller has fewer requirements on Fluvio and the file implements fewer routines: **produceMessage** and **startConsumerStream**. The APIs are reviewed in the [previous section](#add-fluvio-libts-to-proxy-service).

Next, we'll update workflow controller to exchange messages with fluvio instead of directly with the websocket proxy.

### Integrate fluvio with workflow-controller

When workflow controller communicates with fluvio instead of the websocket proxy, it becomes an independent service. A handful of code changes will turn our workflow implementation into a powerful microservice that can be moved, scaled and upgraded independently. 

Paste the following code changes in the `./src/workflow-service/workflow-controller.ts` file:

{{< highlight typescript "hl_lines=7 10 17-19 21 24-25 31-34 36-39 106-114 117-131" >}}
import {
    SID,
    ResponseMessage,
    ChoiceResponse,
    UserText,
    buildRequest,
    isRequest,
} from "../messages";
import { StateMachine, State } from "./state-machine";
import { FluvioLib, fluvioEvents } from "./fluvio-lib";

export class WorkflowController {
    private static _stateMachine: StateMachine;
    private static _initState: string;
    private _fluvio: FluvioLib;

    constructor() {
        this._fluvio = new FluvioLib();
    }

    init(stateMachine: StateMachine, wsProxy: WsProxy) {
        this.listenForEvents();

        await this._fluvio.init(topicName);
        await this._fluvio.startConsumerStream();

        WorkflowController._stateMachine = stateMachine;
        WorkflowController._initState = stateMachine.keys().next().value;
    }

    async processNewConnection(sid: SID) {
        const nextStates = this.getInit();
        await this.sendMessages(sid, nextStates);
    }

    async processClientMessage(sid: SID, response: ResponseMessage) {
        const nextStates = this.getNext(response);
        await this.sendMessages(sid, nextStates);
    }

    private getInit() {
        return this.processNext(WorkflowController._initState);
    }

    private getNext(response: ResponseMessage) {
        var state: string = this.getState(response);

        return this.processNext(state);
    }

    private getState(response: ResponseMessage) {
        switch (response.kind) {
            case "ChoiceResponse": {
                return this.getChoiceResponseState(response);
            }
            case "UserText": {
                return this.getUserTextState(response);
            }
        }
    }

    private processNext(startState: string) {
        var nextStates: State[] = [];

        var state = WorkflowController._stateMachine.get(startState);
        while (state) {
            nextStates.push(state);

            const next = state.next || "";
            state = WorkflowController._stateMachine.get(next);
            if (next.length > 0 && !state) {
                console.error(`Error: Cannot find next state: ${next}`);
            }
        }

        return nextStates;
    }

    private getChoiceResponseState(choiceResponse: ChoiceResponse) {
        for (let [key, state] of WorkflowController._stateMachine.entries()) {
            if (state.matchResponse &&
                state.matchResponse.kind == choiceResponse.kind &&
                state.matchResponse.groupId == choiceResponse.groupId &&
                state.matchResponse.itemId == choiceResponse.itemId) {
                return key;
            }
        }

        console.error(`Error: cannot find choice ${JSON.stringify(choiceResponse)}`);
        return WorkflowController._initState;
    }

    private getUserTextState(userText: UserText) {
        for (let [key, state] of WorkflowController._stateMachine.entries()) {
            if (state.matchResponse &&
                state.matchResponse.kind == "UserText" &&
                state.matchResponse.sessionId == userText.sessionId) {
                return key;
            }
        }

        console.error(`Error: cannot find user session ${JSON.stringify(userText)}`);
        return WorkflowController._initState;
    }

    private async sendMessages(sid: SID, nextStates: State[]) {
        for (let idx = 0; idx < nextStates.length; idx++) {
            const state = nextStates[idx];
            if (state.sendRequest) {
                const message = buildRequest(sid, state.sendRequest);
                await this._fluvio.produceMessage(JSON.stringify(message));
            }
        }
    }

    private listenForEvents() {
        fluvioEvents.on(
            fluvioEvents.FLUVIO_MESSAGE,
            async (msgObj: string) => {
                const message = JSON.parse(msgObj);
                if (!isRequest(message.payload)) {
                    const sid = message.sid;

                    if (message.payload) {
                        await this.processClientMessage(sid, message.payload.message);
                    } else {
                        await this.processNewConnection(sid);
                    }
                }
            }
        );
    }
}
{{< /highlight >}}

Let's review the code changes:
* **import**: replaces WsProxy it FluvioLib.
* **constructor**: declares a Fluvio object.
* **init**: initializes Fluvio object to connect to a topic.
* **sendMessage**: appends request headers and sends the message to Fluvio.
* **listenForEvents** replaces the WsProxyEvents listener with the fluvioEvents listener. 

Finally, **processNewConnection** and **processClientMessage** are updated to use _sendMessage_ API.

Changes in the workflow controller initialization impacts bot-server initialization. Let's update bot-server next.

### Update `bot-server.ts` initialization

Changes to the bot-server are two fold: 
* add **streamingController** - initialize with topic name and the wsProxy instance.
* update **workflowController** - replace wsProxy with topic name.

Update the highlighted lines in the `./src/bot-server.ts` file:

{{< highlight typescript "hl_lines=6 9 27-29 35" >}}
import http from "http";
import express from "express";
import { WsProxy } from "./ws-proxy";
import { StateMachine, loadStateMachine } from "./state-machine";
import { WorkflowController } from "./workflow-controller";
import { StreamingController } from "./proxy-service/streaming-controller";

const PORT = 9998;
const DATA_STREAM_TOPIC = "bot-assist-messages";

// Provision Bot Assistant server
const startServer = async () => {
    const app = express();
    const Server = http.createServer(app);

    // Attach websocket to server
    const wsProxy = new WsProxy();
    wsProxy.init(Server);

    // Start server
    Server.listen(PORT, () => {
        console.log(
            `started bot assistant server at http://localhost:${PORT}...`
        );
    });

    // Initialize streaming controller
    const streamingController = new StreamingController();
    await streamingController.init(DATA_STREAM_TOPIC, wsProxy);

    // Initialize state machine
    let filePath = getFileName();
    const stateMachine: StateMachine = loadStateMachine(filePath);
    const workflowController = new WorkflowController();
    await workflowController.init(DATA_STREAM_TOPIC, stateMachine);
};

// read state machine file from command line
function getFileName() {
    if (process.argv.length != 3) {
        console.log("Usage: node bot-server.js <state-machine.json>");
        process.exit(1);
    }
    return process.argv[2];
}

// Start Server
startServer();
{{< /highlight >}}

Congratulations! The bot server changes are now completed. Let's put the finishing touches on the bot client.

### Add session handling to bot-client

The bot client implementation works well, but if a network connection occurs or the web proxy resets, the client stops working. In addition, the client lack the ability to restart the workflow.

<a href="https://github.com/joewalnes" target="_blank">Joe Walnes</a> has written a great utility called <a href="https://github.com/joewalnes/reconnecting-websocket" target="_blank">reconnecting-socket.js</a> that we'll be leveraging in our project.

Let's copy the file in the `bot-client/scripts` directory:

```bash
cd ../bot-client
curl -L https://raw.githubusercontent.com/infinyon/fluvio-demo-apps-node/master/bot-assistant/bot-client/scripts/reconnecting-socket.js --output scripts/reconnecting-socket.js
```

#### Update `assistant.js`

We'll update assistant file to use _reconnecting-websocket.js_. In addition, we'll add a new button to remove the browser cookie and restart the workflow.

Paste the following code changes in the `./scripts/assistant.js` file:

{{< highlight javascript "hl_lines=3 5-6 15-16 20-21 37 57-64 74 77-78 83 102-108 262-273 294-301" >}}
window.onload = () => {
    var webSocket = null;
    var sessionId = "";

    // Load reconnecting socket to DOM
    loadScript("scripts/reconnecting-socket.js");

    // Create and attach Bot Assistant HTML elements
    function loadAssistant() {
        // Add assistant button
        var note = createElement("img", { "src": `img/assistant/note.svg` }),
            aButton = createElement("button", {}, note);

        // Append assistant dialog
        var status = createElement("div", { "id": "bot-status", "class": "status off" }),
            overlay = createElement("div", { "class": "overlay" }, status),        
            bot = createElement("img", { "src": `img/assistant/bot.svg`, "class": "bot" }),
            title = createElement("span", {}, "Bot Assistant"),
            aDialogClose = createElement("img", { "src": `img/assistant/close.svg`, "class": "close" }),
            aDialogReset = createElement("img", { "src": `img/assistant/redo.svg` }),
            header = createElement("div", { "class": "header" }, [bot, overlay, title, aDialogClose, aDialogReset]),
            msgBody = createElement("div", { "class": "msg-body" }),
            innerBody = createElement("div", { "class": "inner-body" }, msgBody),
            body = createElement("div", { "class": "body-wrapper" }, innerBody),
            userMsg = createElement("div", {
                "id": "user-msg",
                "class": "textareaElement",
                "placeholder": "Choose an option",
                "contenteditable": "false"
            }),
            footer = createElement("div", { "class": "footer" }, userMsg),
            aDialog = createElement("div", { "class": "chat" }, [header, body, footer]);

        // Attach event listeners
        aButton.addEventListener('click', onOpenDialog, false);
        aDialogClose.addEventListener('click', onCloseDialog, false);
        aDialogReset.addEventListener('click', onResetSession, false);

        // Add to document
        document.querySelector(".assistant").appendChild(aButton);
        document.querySelector(".assistant").appendChild(aDialog);
    }

    // On open assistant dialog callback
    function onOpenDialog() {
        document.querySelector(".assistant button").style.display = "none";
        document.querySelector(".assistant .chat").style.display = "block";
        openWSConnection();
    }

    // On close assistant dialog callback
    function onCloseDialog() {
        document.querySelector(".assistant .chat").style.display = "none";
        document.querySelector(".assistant button").style.display = "block";
    }

    // Clear the cookie and restart connection to create a new session.
    function onResetSession() {
        document.cookie = "Fluvio-Bot-Assistant=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/";

        closeWsConnection();
        clearMessages();
        openWSConnection();
    }

    // Open WebSocket connection
    function openWSConnection() {
        try {
            if (webSocket != null) {
                return; // already connected
            }

            logOutput("Connecting to: ws://localhost:9998/");
            webSocket = new ReconnectingWebSocket("ws://localhost:9998/");

            webSocket.onopen = function (openEvent) {
                clearMessages();
                document.getElementById("bot-status").setAttribute("class", "status on");                
                logOutput("Connected!");
            };

            webSocket.onclose = function (closeEvent) {
                document.getElementById("bot-status").setAttribute("class", "status off");
                logOutput("Disconnected!");
            };

            webSocket.onerror = function (errorEvent) {
                logOutput(`Error: ${JSON.stringify(errorEvent)}`);
            };

            webSocket.onmessage = function (messageEvent) {
                var serverMsg = messageEvent.data;
                logOutput(`<== ${serverMsg}`);
                onMessageFromServer(serverMsg);
            };

        } catch (exception) {
            logOutput(`error: ${JSON.stringify(exception)}`);
        }
    }

    // Close WS Connection
    function closeWsConnection() {
        if (webSocket.open) {
            webSocket.close();
            webSocket = null;
        }
    }

    // On messages received from Websocket
    function onMessageFromServer(value) {
        const message = JSON.parse(value);
        switch (message.kind) {
            case "BotText":
                showBotText(message.content);
                break;
            case "UserText":
                showUserText(message.content);
                break;
            case "ChoiceRequest":
                showBotText(message.question);
                showChoiceButtons(message.groupId, message.choices);
                break;
            case "ChoiceResponse":
                choicesToButton(message.groupId, message.content);
                break;
            case "StartChatSession":
                sessionId = message.sessionId;
                enableChatEditor(message.chatPrompt, message.chatText);
                break;
            case "EndChatSession":
                disableChatEditor();
                break;
        };
    }

    // Send a message on WebSocket
    function sendWsMessage(message) {
        if (webSocket.readyState != WebSocket.OPEN) {
            logOutput("WebSocket is not connected: " + webSocket.readyState);
            return;
        }

        const msgObj = JSON.stringify(message)
        logOutput(`==> ${msgObj}`);

        webSocket.send(msgObj);
    }

    // Show text from bot assistant
    function showBotText(content) {
        if (content.length > 0) {
            removeDuplicateAvatar("bot");

            var img = createElement("img", { "src": `img/assistant/bot.svg` }),
                avatar = createElement("div", { "class": "avatar", "id": "bot" }, img),
                msg = createElement("div", { "class": "msg" }, content),
                msgLeft = createElement("div", { "class": "msg-left" }, [msg, avatar]);

            document.querySelector(".msg-body").appendChild(msgLeft);
            scrollToBottom(".inner-body");
        }
    }

    // Show text from user interactive session
    function showUserText(content) {
        if (content.length > 0) {
            var msg = createElement("div", { "class": "msg" }, content),
                msgLeft = createElement("div", { "class": "msg-right" }, msg);

            document.querySelector(".msg-body").appendChild(msgLeft);
            scrollToBottom(".inner-body");
        }
    }

    // Show choices
    function showChoiceButtons(groupId, choices) {
        if (choices.length > 0) {
            var buttons = [];

            choices.forEach(choice => {
                var button = createElement("div", { "class": "button" }, choice.content);
                button.addEventListener('click', function () {
                    pickChoice(groupId, choice.itemId, choice.content);
                }, false);

                buttons.push(createElement("div", { "class": "btn" }, button));
            });

            var msgLeft = createElement("div", { "class": "msg-left", "id": groupId }, buttons);

            document.querySelector(".msg-body").appendChild(msgLeft);
            scrollToBottom(".inner-body");
        }
    }

    // Callback invoked on user selection
    function pickChoice(groupId, itemId, content) {
        choicesToButton(groupId, content);

        sendWsMessage({
            kind: "ChoiceResponse",
            groupId: groupId,
            itemId: itemId,
            content: content,
        });
    }

    // Swap choices with a button representing the selection
    function choicesToButton(groupId, content) {
        document.getElementById(groupId).remove();

        var button = createElement("div", { "class": "button selected" }, content),
            btn = createElement("div", { "class": "btn" }, button),
            msgRight = createElement("div", { "class": "msg-right" }, btn);

        document.querySelector(".msg-body").appendChild(msgRight);
        scrollToBottom(".inner-body");
    }

    // On multiple bot messages, ensure avatar is only displayed on last entry
    function removeDuplicateAvatar(id) {
        var messages = document.querySelector('.msg-body').children;
        if (messages.length > 0) {
            var lastMessage = messages[messages.length - 1];
            if (lastMessage.getAttribute("class") === 'msg-left') {
                if (lastMessage.lastChild.id == id) {
                    lastMessage.removeChild(lastMessage.lastChild);
                }
            }
        }
    }

    // Make editor section editable
    function enableChatEditor(chatPrompt, chatText) {
        if (chatText) {
            showBotText(chatText);
        }

        var chatBox = document.getElementById("user-msg");
        chatBox.setAttribute("contenteditable", true);
        chatBox.setAttribute("placeholder", chatPrompt || "Type question here ...");

        chatBox.addEventListener("keydown", onEditorKeys, false);
    }

    // Disable interactive chat
    function disableChatEditor() {
        var chatBox = document.getElementById("user-msg");
        chatBox.addEventListener("keydown", {}, false);

        chatBox.setAttribute("contenteditable", false);
        chatBox.setAttribute("placeholder", "Choose an option");
    }

    // Scroll to last messages
    function scrollToBottom(tag) {
        var div = document.querySelector(tag);
        div.scrollTop = div.scrollHeight - div.clientHeight;
    }

    // Clear messages in both editors
    function clearMessages() {
        var parent = document.querySelector('.msg-body');
        while (parent.firstChild) {
            parent.removeChild(parent.firstChild);
        }

        var debugOutput = document.getElementById("debugOutput");
        if (debugOutput) {
            debugOutput.value = "";
        }
    }

    // Callback on chat editor user input (key press)
    function onEditorKeys(e) {
        var chatBox = document.getElementById("user-msg");

        if (e.code == 'Enter' && chatBox.textContent.length > 0) {
            e.preventDefault();

            const content = chatBox.textContent;
            sendWsMessage({
                kind: "UserText",
                sessionId: sessionId,
                content: content,
            });
            showUserText(content);

            chatBox.innerHTML = '';
        }
    }

    //  Load external javascript file to DOM
    function loadScript(fileName) {
        var js_script = document.createElement('script');
        js_script.type = "text/javascript";
        js_script.src = fileName;
        js_script.async = false;
        document.getElementsByTagName('head')[0].appendChild(js_script);
    }

    // Log output in the "debugOutput" textarea (if available) and the console
    function logOutput(value) {
        var debugOutput = document.getElementById("debugOutput");
        if (debugOutput) {
            debugOutput.value += value + "\n\n";
            debugOutput.scrollTop = debugOutput.scrollHeight;
        }
        console.log(value);
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
{{< /highlight >}}

In summary, the code changes are as follows:
* add a routine to load `reconnecting-websocket.js` into the DOM.
* add connection status indicator - (on - green, off - red).
* add a refresh button to reset workflow.

We'll also need to update the stylesheet to add status indicator.

#### Update `assistant.css`

Update the stylesheet to add status overlay and color indicator.

Paste the following code changes in the `./css/assistant.css` file:

{{< highlight css "hl_lines=70-78 80-84 86-88 90-92" >}}
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

/* Assistant - Chat Box */

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

.assistant .header .overlay {
	background-color: #f6f6f6;
	padding: 1px;
	position: absolute;
	top: 32px;
	left: 33px;
	border-radius: 12px;
	z-index: 1;
}

.assistant .header .status{
	width: 12px;
	height: 12px;
	border-radius: 12px;
}

.assistant .header .status.off{
	background-color: #FF3B28;
}

.assistant .header .status.on{
	background-color: greenyellow;
}

.assistant .header .close{
	float:right;
	cursor:pointer;
	width: 28px;
	margin-right: 0;
}

.assistant .inner-body{
	min-height: 250px;
	max-height: calc(100vh - 300px);
	overflow: auto;
	overflow-x: hidden;
}

.assistant .msg-body {
	font-size:12px;
	padding: 10px 10px 5px 5px;
}

.assistant .msg-left{
	margin-bottom:7px;
	word-break: break-all;
}

.assistant .msg-left .avatar {
	width: 50px;
	margin-top: -40px;
}

.assistant .msg-left .operator {
	margin-top: -40px;
	padding: 1px;
	font-size:1.6em;
	width:35px;
	height:35px;
	line-height:1.8em;
	text-align:center;
	border-radius:50%;
	background:plum;
	color:white;
}

.assistant .msg-left img {
	width:35px;
	height:35px;
	border-radius:50%;
	background:#bbb;
	border: 1px solid #eee;
}

.assistant .msg-left .msg {
	background:#f2f2f2;
	padding:10px;
	min-height:15px;
	margin: 3px;
	border: 1px solid #ddd;
	border-radius:7px;
	margin-left: 44px;
	margin-right: 30px;
}

.assistant .msg-left .button {
	margin: -2px 30px 7px 50px;
}

.assistant .msg-right {
	position: relative;
	right: 0px;
	margin: 3px;
	margin-bottom:10px;
}

.assistant .msg-right .msg {
	background:#d4e7fa;
	padding:10px;
	min-height:15px;
	margin-left: 80px;
	border-radius:7px;
	word-break: break-all;
}

.assistant .msg-right .button {
	float: right;
}

/* button  */

.assistant .btn {
	display: inline-block;
	margin: 2px;
	width: 100%;
}

.assistant .button {
	width: max-content;
	border-radius:15px;
	padding: 10px 15px;
	transition-duration: 0.2s;
	background-color: white;
	color: #006687;
	border: 1px solid #008CBA;
}

.assistant .button.selected {
	background-color: #008CBA;
	color: white;
}
  
.assistant .button:hover {
	cursor: pointer;
	background-color: #008CBA;
	color: white;
}

/* footer  */

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
{{< /highlight >}}

Congratulations! You have powered through this lengthy tutorial on how to create your own custom robot assistant.

The project is available for download in <a href="https://github.com/infinyon/fluvio-demo-apps-node/tree/master/bot-assistant" target="_blank">github</a>.

## Conclusion

While building a robot assistant may have seemed like an overwhelming undertaking, this blog shows that it is possible to build an end-to-end solution in just a few hours. 

The blog also shows the value of a persistent data streaming platform such as Fluvio. The platform allows the bot assistant to scale to a large number of client instance with just a few lines of code.

This project is just scratching the surface on what a bot assistant can do. The improvement range from live operator assistance to machine learning responses, backend service integrations, and much more.

You can reach us on <a href="https://discordapp.com/invite/bBG2dTz" target="_blank">discord</a>. We look forward to hearing from you.