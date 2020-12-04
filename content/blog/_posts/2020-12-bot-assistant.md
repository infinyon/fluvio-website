---
title: Build Your Own Custom Robot Assistant
author: 
    name: "The Fluvio Team"
description: Leverage real-time data streaming to build your own custom robot assistant.
date: 2020-12-01
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
* [Step 3: Add WebSocket communication](#step-3-add-websocket-communication)
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

Copy the following code in `css/assistant.css`

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
	max-height: calc(100vh - 120px);
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
            header = createElement("div", { "class": "header" }, [bot, title, close]),
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

The WsServer is then called inside `src/bot-server.ts`. Let's add WebSocket API inside `startServer` block:

{{< highlight typescript "hl_lines=3 12" >}}
import http from "http";
import express from "express";
import WebSocket from "./ws-server";

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
};

// Start Server
startServer();
{{< /highlight >}}

Note that `ts-watch` has updated the code. Next, we'll add websocket to the client.


#### Add WebSocket to`bot-client`

WebSocket library is available in most modern web browser, which allows us to hook it up with javascript. In the `bot-client` directory let's add some troubleshooting.


##### Add troubleshooting 

Open open `scripts/assistant.js` and add a function to publish client logs to a textarea called `debugOutput`:

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
};
{{< /highlight >}}

Next, open `index.html` and add `debugOutput` textarea:

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

Anytime the code calls `logOutput`, it will be display in the textarea (as well as as in the console).

##### Update assistant.js file

Now, are ready to update the code. Open `scripts/assistant.js` and make the following updates.
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
                logOutput(`<== ${wsMsg}\n`);
            };

        } catch (exception) {
            logOutput(`error: ${JSON.stringify(exception)}`);
        }
    }

    ...
}
```

Let's temporarily hook-up the connection to `onOpenDialog` to test the WS connection:

{{< highlight javascript "hl_lines=4" >}}
function onOpenDialog() {
    document.querySelector(".assistant button").style.display = "none";
    document.querySelector(".assistant .chat").style.display = "block";
    openWSConnection();
}
{{< /highlight >}}

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

{{< highlight javascript "hl_lines=5-6" >}}
window.onload = () => {
    ....
    loadAssistant();

    // TODO: Remove after testing
    enableChatEditor();
}
{{< /highlight >}}

<center> Consolidate <i>assistant.js</i> file published in <a href="https://gist.github.com/ajhunyady/9449ad8c29f1d6131a9a2d860f33bc7b" target="_blank">gist</a></center>


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

{{< highlight typescript "hl_lines=3 5 21-24 34-43  46-47 50 65-79" >}}
import WS from "ws";
import http from "http";
import crypto from 'crypto';

const COOKIE_NAME = "Fluvio-Bot-Assistant"

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
            const session = req.headers.session;
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

// Parse session from cookie
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

const wsSingleton = new WsServer();
Object.freeze(wsSingleton);

export default wsSingleton;
{{< /highlight >}}

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

Congratulations, you have successfully added session cookies to Bot Assistant.


## Step 5: Add state machine

While there are many ways to drive a robot assistant, this project uses a custom state machine. You may think of a state machine as a guided tour, where the bot sends the client a series of choices. Upon response the bot looks-up the reply and moves to the next state. Then the loop repeats until the end state is reached.

<img src="/blog/images/bot-assistant/state-machine.svg"
     alt="State Machine"
     style="justify: center; max-width: 600px;" />

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
To test the code, we'll attach the state machine parser to `bot-server.ts`:

{{< highlight typescript "hl_lines=4 22-25 28-35" >}}
import http from "http";
import express from "express";
import WebSocket from "./ws-server";
import { StateMachine, loadStateMachine } from "./state-machine";

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

Congratulations, your state machine parser is up and running.


## Step 6: Add workflow controller

Workflow controller is the mediator between the client and the state machine. The controller receives a client message, computes the next state, and generates a reply message.

<img src="/blog/images/bot-assistant/workflow-controller.svg"
     alt="Workflow Controller"
     style="justify: center; max-width: 800px;" />

The controller requires code changes to other `bot-server` components:
* `messages.ts` to include header information
* `bot-server.ts` to initialize workflow controller
* `ws-server.ts` to invoke workflow controller of new client messages.

We also need to update the client `assistant.js` to handle the Bot Assistant workflow.

### Update `messages.ts` file

Aside from data structures for state machine, the message exchanged with the client also needs header information. 

Let's append `messages.ts` file as follows: 

``` typescript
export type TimeStamp = string;

/* Message Header */
export interface Message {
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

...

/* Append header to a request message */
export function buildRequest(message: RequestMessage) {
    return <Message>{
        payload: <Request>{
            kind: "Request",
            message: message
        },
        timestamp: getDateTime(),
    };
};

/* Generate timestamp */
function getDateTime() {
    return new Date(Date.now() - new Date().getTimezoneOffset() * 60000)
        .toISOString()
        .slice(0, -1);
}
```

<center>Consolidated <i>messages.ts</i> file published in <a href="https://gist.github.com/ajhunyady/1f380b29d72381648b1e25bbc7867901" target="_blank">gist</a>.</center>

### Add `workflow-controller.ts` file

The workflow controller is the central coordinator between the client and the state machine. 

Let's add the file:

```bash
touch src/workflow-controller.ts
```

Copy the following content in the `src/workflow-controller.ts` file:

```ts
import WS from "ws";
import { Message, ResponseMessage, ChoiceResponse, UserText, buildRequest } from "./messages";
import { StateMachine, State } from "./state-machine";

class WorkflowController {
    private static _stateMachine: StateMachine;
    private static _initState: string;

    init(stateMachine: StateMachine) {
        WorkflowController._stateMachine = stateMachine;
        WorkflowController._initState = stateMachine.keys().next().value;
    }

    processNewConnection(ws: WS) {
        const nextStates = WMS.getInit();
        nextStates.forEach(state => {
            if (state.sendRequest) {
                const request = buildRequest(state.sendRequest);
                const message = JSON.stringify(request);
                ws.send(message);
            }
        })
    }

    processClientMessage(ws: WS, msgObj: string) {
        const message: Message = JSON.parse(msgObj);

        if (message.payload) {
            if (message.payload.kind == 'Response') {
                const nextStates = WMS.getNext(message.payload.message);
                nextStates.forEach(state => {
                    if (state.sendRequest) {
                        const request = buildRequest(state.sendRequest);
                        const message = JSON.stringify(request);
                        ws.send(message);
                    }
                })
            }
        }
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
}

// Workflow Manager Singleton
const WCS = new WorkflowController();
Object.freeze(WCS);

export default WCS;
```

In summary, Workflow controller has the following public functions:
* `init` - loads the state machine and saves initial state
* `processNewConnection` - called from new connections to run state machine from initial state.
* `processNewConnection` - called from new messages to run state machine based on the client response.

The other APIs help the controller identify the response type and traverse the state machine to generate subsequent requests.


### Update `bot-server.ts` file

We need to hook-up workflow controller in the `bot-server.ts` file. Let's update the file:

{{< highlight typescript "hl_lines=5 26" >}}
import http from "http";
import express from "express";
import WebSocket from "./ws-server";
import { StateMachine, loadStateMachine } from "./state-machine";
import WorkflowController from "./workflow-controller";

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
    let filePath = getFileName();
    const stateMachine: StateMachine = loadStateMachine(filePath);
    WorkflowController.init(stateMachine);
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

Just two changes: import `WorkflowController` and initialize.


### Update `ws-server.ts` file

Next, we hook-up workflow controller in the websocket server. Let's update the file:

{{< highlight typescript "hl_lines=4 50 58" >}}
import WS from "ws";
import http from "http";
import crypto from 'crypto';
import WorkflowController from './workflow-controller';

const COOKIE_NAME = "Fluvio-Bot-Assistant"

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
            const session = req.headers.session;
            console.log(`session opened - ${session}`);

            WorkflowController.processNewConnection(ws);

            ws.on("close", function () {
                console.log(`session closed - ${session}`);
            });

            ws.on("message", (msgObj: string) => {
                console.log(`< ${msgObj}`);
                WorkflowController.processClientMessage(ws, msgObj);
            });

        });
    }
}

// Parse session from cookie
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

const wsSingleton = new WsServer();
Object.freeze(wsSingleton);

export default wsSingleton;
{{< /highlight >}}

Just a couple of changes: import `WorkflowController` and attach to connection callback in two places: _on.connection_ and _on.message_.


#### Test workflow controller initialization

With the backend server updated, we are ready to test the initial step of the workflow. To test message, we'll need to update the frontend. 

The backend server code has been automatically updated by `ts-watch`, hence our code should be up to date.
Let's open the web browser to `http://localhost:9999/`, then click on "Bot Assistant` button. 

The client initiates a new connection and the workflow controller responds with the following messages:

<img src="/blog/images/bot-assistant/client-workflow-init.svg"
     alt="Client Workflow Initial Message"
     style="justify: center; max-width: 700px" />

Congratulations, `Workflow Controller` is up and running, let's update the client to navigate the entire state machine.

### Update client `assistant.js`

Our initial assistant implementation was focused on simple navigation capabilities. Next, we need to add support for the workflow protocol. 

Create a function called `onMessageFromServer` that parses and dispatches the messages.

```javascript
window.onload = () => {
    var webSocket = null;
    var sessionId = "";
    ....

    /* On messages received from Websocket */
    function onMessageFromServer(value) {
        const msg = JSON.parse(value);
        const payload = msg.payload;

        if (!payload || !payload.kind || !payload.message) {
            console.error(`Error: Invalid message ${JSON.stringify(payload)}`);
            return;
        }

        const message = payload.message;
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

    function showUserText(content) {
        if (content.length > 0) {
            var msg = createElement("div", { "class": "msg" }, content),
                msgLeft = createElement("div", { "class": "msg-right" }, msg);

            document.querySelector(".msg-body").appendChild(msgLeft);
            scrollToBottom(".inner-body");
        }
    }

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

    function pickChoice(groupId, itemId, content) {
        choicesToButton(groupId, content);

        sendWsMessage({
            kind: "ChoiceResponse",
            groupId: groupId,
            itemId: itemId,
            content: content,
        });
    }

    function choicesToButton(groupId, content) {
        document.getElementById(groupId).remove();

        var button = createElement("div", { "class": "button selected" }, content),
            btn = createElement("div", { "class": "btn" }, button),
            msgRight = createElement("div", { "class": "msg-right" }, btn);

        document.querySelector(".msg-body").appendChild(msgRight);
        scrollToBottom(".inner-body");
    }

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

    function disableChatEditor() {
        var chatBox = document.getElementById("user-msg");
        chatBox.addEventListener("keydown", {}, false);

        chatBox.setAttribute("contenteditable", false);
        chatBox.setAttribute("placeholder", "Choose an Option");
    }

    /* scroll chat view to last message */
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

    /* Send a message on WebSocket */
    function sendWsMessage(msg) {
        if (webSocket.readyState != WebSocket.OPEN) {
            logOutput("WebSocket is not connected: " + webSocket.readyState);
            return;
        }

        const message = {
            payload: {
                kind: "Response",
                message: msg
            },
            timestamp: getUnixTime()
        };

        const msgObj = JSON.stringify(message)
        logOutput(`==> ${msgObj}`);

        webSocket.send(msgObj);
    }

    function getUnixTime() {
        return (new Date(new Date().toString().split('GMT')[0] + ' UTC').toISOString().split('.')[0]) + "." + new Date().getMilliseconds()
    }

    ...
};
```

Other changes are as follows:
* `onEditorKeys` - creates a UserText response type
* `sendWsMessage` - wraps responses in message header
* `enableChatEditor` - takes additional parameters: chatPrompt and chatText
* Remove `enableChatEditor` that was temporarily added.


Next, let's hook-up the function to Ws connection:

{{< highlight javascript "hl_lines=12" >}}
window.onload = () => {
    ...

    // Open WebSocket connection
    function openWSConnection() {
        try {
            ...

            webSocket.onmessage = function (messageEvent) {
                var wsMsg = messageEvent.data;
                logOutput(`<== ${wsMsg}`);
                onMessageFromServer(wsMsg);
            };

        } catch (exception) {
            logOutput(`error: ${JSON.stringify(exception)}`);
        }
    }
    ...
}
{{< /highlight >}}

<center>Consolidated <i>assistant.js</i> file published in <a href="https://gist.github.com/ajhunyady/44b613f006eab31cc3e9fb9f964ac5b9" target="_blank">gist</a>.</center>


### Update `assistant.css`

Next we'll update `assistant.css` file for styling. In addition, the javascript file updated above, looks-up editor content based on css class names.

Open `css/assistant.css` file and append the following fields right before `footer`:

```css
...

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
	padding: 10px;
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
...
```

<center> Consolidate <i>assistant.css</i> file published in <a href="https://gist.github.com/ajhunyady/ecca29e6a6e8b8fa2a8ec36c2e277c70" target="_blank">gist</a></center>


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
     style="justify: center; max-width: 740px" />

The new architecture give us additional flexibility for:

* **scale** the proxy and workflow independently of each other. 

* **handoff** a conversation to a human operator. We can do that by adding an `operator service` independently that interacts directly with the client through the data stream.

* **add-on services** such as: analytics, machine learning, or connectors to other products.

-> **Prerequisites:** This section assumes you have access to a Fluvio cluster. Step-by-step instructions on setting-up Fluvio are available in [Quick Start](/docs/getting-started/fluvio-cloud/).

To integrate Fluvio data streaming we'll make the following code changes:
* [Restructure code in services](#restructure-code-in-services)
* [Add Fluvio data streaming](#add-fluvio-data-streaming)


### Restructure code in services

As shown in the diagram above, we'll divide the bot server code into separate services: `proxy-service` and `workflow-service`.

After division we'll end up with the following file layout in `bot-server/src` directory:

```bash
tree
.
├── bot-server.ts
├── messages.ts
├── proxy-service
│   └── ws-server.ts
└── workflow-service
    ├── state-machine.ts
    └── workflow-controller.ts
```

Shared files (_bot-server.ts_ and _messages.ts_) are left at the top level, others are divided along service boundaries. 

-> Make sure all **import** statements impacted by file movement are updated and the code continues to compile and run the same as before.

### Add Fluvio data streaming

Fluvio has a node native library <a href="https://www.npmjs.com/package/@fluvio/client">@fluvio/client</a> in **npm**. Let's install in `bot-server` directory:

```bash
npm install @fluvio/client
...
added 2 packages from 1 contributor and audited 93 packages in 4.846s
found 0 vulnerabilities
```

In current form, _proxy_ and _workflow_ services are running in the same binary. After the Fluvio integration, they will be completely decoupled and can be moved to different machines virtually anywhere in the world. To that effect, let's add a fluvio API file to each service.

#### Add fluvio.ts to proxy-service

Create `fluvio.ts` file inside proxy-service:

```bash
cd ./proxy-service
touch fluvio.ts
```

Copy the following code into the file:

```typescript
import Fluvio, { Offset, OffsetFrom } from '@fluvio/client';
import { dataStreamingEvents } from './data_streams';

/* Data Streaming Event Emitter */
class DataStreamingEvents extends EventEmitter {
    readonly FLUVIO_MESSAGE = 'Fluvio';
}
export const dataStreamingEvents = new DataStreamingEvents();

/* Find a topic*/
export async function findTopic(topicName: string) {
    const fluvio = new Fluvio();

    await fluvio.connect();
    const admin = await fluvio.admin();
    const topic = await admin.findTopic(topicName);

    return (topic != null);
}

/* Create a topic */
export async function createTopic(topicName: string) {
    const fluvio = new Fluvio();

    await fluvio.connect();
    const admin = await fluvio.admin();
    await admin.createTopic(topicName);
}

/* Create topic if does not exist */
export async function createTopicIfNotFound(topicName: string) {
    if (!await findTopic(topicName)) {
        await createTopic(topicName);
        await sleep(2000);
        console.log(`proxy: topic '${topicName}' created`);
    }
}

/* Produce - produce a message */
export async function produceMessage(topicName: string, msg: string) {
    const fluvio = new Fluvio();

    await fluvio.connect();
    const producer = await fluvio.topicProducer(topicName);
    producer.sendRecord(msg, 0);
}

/* Consumer Stream - Continuous fetch records from stream. */
export async function startConsumerStream(topicName: string) {
    const fluvio = new Fluvio();

    await fluvio.connect();

    const consumer = await fluvio.partitionConsumer(topicName, 0)
    const offset: Offset = new Offset({ from: OffsetFrom.End, index: 0 })

    console.log('proxy: listening for events ... ');

    consumer.stream(offset, (record: string) => {
        dataStreamingEvents.emit(
            dataStreamingEvents.FLUVIO_MESSAGE,
            record
        );
    })
}

/* Consumer Fetch - Fetch all messages from offset 0 */
export async function fetchMessages(topicName: string) {
    const fluvio = new Fluvio();

    await fluvio.connect();

    const consumer = await fluvio.partitionConsumer(topicName, 0)
    const offset: Offset = new Offset()

    const fetched = await consumer.fetch(offset);
    if (fetched) {
        fetched.records.batches.forEach(batch => {
            batch.records.forEach(record => {
                dataStreamingEvents.emit(
                    dataStreamingEvents.FLUVIO_MESSAGE,
                    record.value
                );
            });
        });
    }

    console.log(`proxy: fetched ${fetched.highWatermark} messages`);
}

export async function sleep(ms: number) {
    return new Promise((resolve) => {
        setTimeout(resolve, ms)
    })
}
```

The file provides convenience functions to search and create topics, as well as produce and consume messages. Messages can be consumed continuously or fetched from a specific offset. Messages received from the topic are emitted in FLUVIO_MESSAGE.

Next we'll add a data streaming file to proxy fluvio messages between the client and the workflow service.

#### Add data-streaming.ts to proxy-service

Data streaming file is the facilitator for all fluvio interactions. It creates the topic, and intermediates all message exchanges between the websocket and Fluvio.


Create `fluvio.ts` file inside proxy-service:

```bash
cd ./proxy-service
touch data-streaming.ts
```

Copy the following code into the file:

```typescript