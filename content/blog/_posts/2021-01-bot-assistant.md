---
title: Build Your Own Custom Robot Assistant
author:
    name: "A.J. Hunyady"
    github: "ajhunyady"
description: Use data streaming to build a custom robot assistant that can connect to any backend service in the organization.
metadata: TECH
date: 2021-01-30
slug: bot-assistant
url: /blog/2021/01/bot-assistant
img: blog/images/bot-assistant/social/bot.jpg
twitter-card: summary_large_image
---

Many successful modern applications need to interact with their users in real-time, and this capability is quickly becoming the expected standard. However, building a real-time application from scratch is a daunting task, pulling focus away from the business problems the team is actually trying to solve. Fluvio is a real-time data streaming platform designed to make real-time application development easy. 

In this blog post, we're going to build a **Robot Assistant**, an add-on button on the website, that interacts with users in real-time. 

<img src="/blog/images/bot-assistant/bot-assistant.svg"
     alt="Bot Assistant Example"
     style="justify: center; max-width: 380px" />

We'll build the frontend and backend, then use Fluvio as our data streaming layer.
Fluvio data streaming gives us the ability to react in real-time, deploy to a massive audience, and preserve all data exchanges. 

The project is also available for download in <a href="https://github.com/infinyon/fluvio-demo-apps-node/tree/master/bot-assistant" target="_blank">github</a>.

## Prerequisites

This project is using `websocket-glue` for the client/server communication. For additional information on websocket checkout our blog:
*  [Websocket Glue for Data Streaming Apps](/blog/2021/01/websocket-glue-for-streaming-apps/)

Familiarity with the following software packages is useful but not required:  <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript" target="_blank">Javascript</a>, <a href="https://www.typescriptlang.org/docs/" target="_blank">TypeScript</a>, <a href="https://nodejs.org/">Node.js</a>, and <a href="https://developer.mozilla.org/en-US/docs/Web/API/WebSocket" target="_blank">WebSocket</a>.

## Overview 

This blog takes a step-by-step approach to building a robot assistant, called `Bot Assistant`, from the ground up. The following outline shows the steps involved:

* [Step 1: Create the project](#step-1-create-the-project)
    * [Add project directory](#add-project-directory)
    * [Add node.js server](#add-nodejs-server)
    * [Add typescript configuration](#add-typescript-configuration)
    * [Add `bot-assistant.ts` server file](#add-bot-assistantts-server-file)
* [Step 2: Implement backend server](#step-2-implement-backend-server)
    * [Add messages type definition file](#add-messages-type-definition-file)
    * [Add state machine](#add-state-machine)
    * [Add workflow controller](#add-workflow-controller)
    * [Add proxy service](#add-proxy-service)
    * [Add `bot-server.ts` file](#add-bot-serverts-file)
* [Step 3: Implement frontend client](#step-3-implement-frontend-client)
    * [Add `index.html` file](#add-indexhtml-file)
    * [Add stylesheet file](#add-stylesheet-file)
    * [Add assistant images](#add-assistant-images)
    * [Add `assistant.js` script](#add-assistantjs-script)
    * [Load `reconnecting-socket.js` file](#load-reconnecting-socketjs-file)
    * [Test Bot Assistant (v1)](#test-bot-assistant-v1)
* [Step 4: Add data streaming and persistency](#step-4-add-data-streaming-and-persistency)
    * [Add fluvio to `session-controller`](#add-fluvio-to-session-controller)
    * [Add fluvio to `workflow-controller`](#add-fluvio-to-workflow-controller)
    * [Add fluvio to `bot-server`](#add-fluvio-to-bot-server)
    * [Add fluvio setup script](#add-fluvio-setup-script)
    * [Test Bot Assistant](#test-bot-assistant)

## Step 1: Create the project

`Bot assistant` has a client and a server. The client runs in the web browser and controls the frontend user interaction, while the backend runs on a server and manages the websocket proxy and the state machine. The client and the server communicate with ech other through websocket.

Let's get started:
* [Add project directory](#add-project-directory)
* [Add node.js server](#add-nodejs-server)
* [Add typescript configuration](#add-typescript-configuration)
* [Add `bot-assistant.ts` server file](#add-bot-assistantts-server-file)

### Add project directory

Create a project directory called `bot-assistant` with two folders `public` and `src`: 

```bash
mkdir -p bot-assistant/{public,src}
cd bot-assistant
```

The `public` directory stores the client code, and the `src` directory contains the server code (the "app server"). Both the client and the app server are served from the same web server that we'll set up next.

### Add node.js server

Create a Node.js project and implement the server. This project is using Node.js v13:

```bash
npm init -y
```

which yields the following `package.json` file:

```json
{
  "name": "bot-assistant",
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

Install `express`, `typescript` and a few other development services:

```bash
npm install typescript express ws @fluvio/client
npm install -D tsc-watch @types/ws @types/node @types/express
```

We installed the following packages:
* **express**: to serve the client and server files.
* **ws**: for client/server communication.
* **@fluvio/client**: node API library to communicate with fluvio.
* **tsc-watch**: to keep track of typescript file changes.

Update package.json file as follows:

{{< highlight json "hl_lines=5 7" >}}
{
  "name": "bot-assistant",
  "version": "1.0.0",
  "description": "",
  "main": "bot-assistant.js",
  "scripts": {
    "start:server": "tsc-watch --onSuccess \"node ./dist/bot-assistant.js $PARAMS\""
  },
  "keywords": [],
  "author": "fluvio <admin@fluvio.io> (fluvio.io)",
  "license": "Apache 2.0",
  "dependencies": {
    "@fluvio/client": "^0.6.0-beta.3",
    "express": "^4.17.1",
    "typescript": "^4.1.3",
    "ws": "^7.4.2"
  },
  "devDependencies": {
    "@types/express": "^4.17.9",
    "@types/node": "^14.14.19",
    "@types/ws": "^7.4.0",
    "tsc-watch": "^4.2.9"
  }
}
{{< /highlight >}}

Change `main` to reference `bot-assistant.js` and `start:dev` script to start the typescript watcher.

#### Add typescript configuration

The project is implemented in typescript which requires a typescript configuration file. 

Add the `tsconfig.json` typescript configuration file:

```bash
touch tsconfig.json
```

Paste the following content in the `tsconfig.json` file:

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
    "strict": true,
    "moduleResolution": "node",
    "esModuleInterop": true,
  },
  "include": [
    "src/*",
  ],
}
```

For additional information on the typescript configuration parameters, check out the <a href="https://www.typescriptlang.org/tsconfig" target="_blank">documentation</a>.

### Add `bot-assistant.ts` server file

The `package.json` file instructs by Node.js to run `bot-assistant.js` when it initializes. In typescript, this file is compiled from `bot-assistant.ts`. This is the place where we provision the web server, add routes for the frontend, and initialize backend services.

Create the `bot-assistant.ts` file in the `src` directory:

```bash
touch src/bot-assistant.ts
```

Paste the following content in the `src/bot-assistant.ts` file:

```ts
import http from "http";
import express from "express";
import path from 'path';

const PORT = 9998;

const startServer = async () => {
    const app = express();
    const publicPath = path.join(__dirname, '..', 'public')

    app.get('/', (req, res) => {
        res.sendFile(path.join(publicPath, 'index.html'));
    });
    app.use("/scripts", express.static(path.join(publicPath, 'scripts')));
    app.use("/css", express.static(path.join(publicPath, 'css')));
    app.use("/img", express.static(path.join(publicPath, 'img')));

    const Server = http.createServer(app);

    Server.listen(PORT, () => {
        console.log(
            `started bot assistant server at http://localhost:${PORT}...`
        );
    });
};

process.on("uncaughtException", (e) => { console.log(e); process.exit(1); });
process.on("unhandledRejection", (e) => { console.log(e); process.exit(1); });

startServer();
```

This code adds routes for the frontend client and starts a server on port 9998. The routes are as follows:

* **/ (root)** => `public/index.html
* **/scripts** => `public/scripts`
* **/css** => `public/css`
* **/img** => `public/img`

Next, we'll implement the [backend server](#step-2-implement-backend-server) followed by the [frontend client](#step-3-implement-frontend-server).


## Step 2: Implement backend server

The backend server has two core services, `Proxy Service` and `Workflow Service`. 

The `Proxy service` handles the connection between the client and the workflows. It accepts websocket connections, forwards client messages to the workflow service, and returns the replies to the originator.

The `Workflow Service` manages state transitions. It initializes the state machine from a json file, accepts client state messages, computes the next state and returns a reply.

<img src="/blog/images/bot-assistant/bot-base-architecture.svg"
     alt="Bot Assistant Base Architecture"
     style="justify: center; max-width: 800px" />


The backend server implementation has several steps:
* [Add messages type definition file](#add-messages-type-definition-file)
* [Add state machine](#add-state-machine)
    * [Define state machine types](#define-state-machine-types)
    * [Define state transitions](#define-state-transitions)
    * [Create a state machine JSON file](#create-a-state-machine-json-file)
    * [Add `state-machine.ts` file](#add-state-machinets-file)
* [Add workflow controller](#add-workflow-controller)
    * [Add `workflow-controller.ts` file](#add-workflow-controllerts-file)
* [Add proxy service](#add-proxy-service)
    * [Add outgoing proxy](#add-outgoing-proxy)
    * [Add incoming proxy](#add-incoming-proxy)
    * [Add session controller](#add-session-controller)
* [Add `bot-server.ts` file](#add-bot-serverts-file)
    * [Update `bot-assistant.ts` file](#update-bot-assistantts-file)

If you prefer to skip ahead, you can download the source code from <a href="https://github.com/infinyon/fluvio-demo-apps-node/tree/master/bot-assistant/_blog/backend-server" target="_blank">github</a> and resume at [start backend server](#start-backend-server).

### Add messages type definition file

The messages file defines the types of the messages for the client/server communication. The messages are shared by both, proxy and workflow services, and it will be a top level file.

Let's add `messages.ts` file inside `src` directory:

```bash
touch src/messages.ts
```

Paste the following message type definitions:

```ts
export type TimeStamp = string;
export type SID = string;

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

export type RequestMessage =
    | BotText
    | ChoiceRequest
    | StartChatSession
    | EndChatSession;

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

export interface ChoiceResponse {
    kind: "ChoiceResponse",
    groupId: string,
    itemId: string,
    content?: string,
}

export interface UserText {
    kind: "UserText",
    sessionId: string,
    content?: string,
}

export function buildInitMessage(sid: SID) {
    return <Message>{
        sid: sid,
        timestamp: getDateTime(),
    };
};

export function buildRequest(sid: SID, message: RequestMessage) {
    return <Message>{
        sid: sid,
        payload: <Request>{ kind: "Request", message: message },
        timestamp: getDateTime(),
    };
};

export function buildResponse(sid: SID, message: ResponseMessage) {
    return <Message>{
        sid: sid,
        payload: <Response>{ kind: "Response", message: message },
        timestamp: getDateTime(),
    };
};

export function isRequest(payload?: Payload) {
    return (payload) ? (payload.kind == "Request") : false;
}

function getDateTime() {
    return new Date(Date.now() - new Date().getTimezoneOffset() * 60000)
        .toISOString()
        .slice(0, -1);
}
```

The message definitions are as follows:
* **Message**: is the top level type definition.
* **Payload**: defines payload types: request or response.
* **Request**: defines request messages (BotText, ChoiceRequest, StartChatSession, EndChatSession).
* **Response**: defines response messages (ChoiceResponse, UserText).
* **BotText**: is a text message sent by the Bot (text parsed as HTML).
* **ChoiceRequest**: is an array of choices sent by the Bot.
* **StartChatSession**: is a request by the Bot to enable chat editor.
* **EndChatSession**: is a request sent by the Bot to disable chat editor.
* **ChoiceResponse**: is the response to a _ChoiceRequest_.
* **UserText**: is text sent by the User.

The definitions are followed by a series of helper APIs: 
* **buildInitMessage**: creates a message without a payload that indicates a new connection.
* **buildRequest**: creates a `Request` message.
* **buildResponse**: creates a `Response` message.
* **isRequest**: checks if the message is of `Request` kind.
* **getDateTime**: generates a timestamp.

The type definitions are used extensively by the `state machine` defined in the following section.

### Add state machine

This project uses a state machine to implement the behavior of the robot assistant. We may think of a state machine as a guided tour where all traffic follows a well-defined path. The state machine defines the choices and the order in which they are to be sent to the client. Upon receipt, the client generates a response and returns an answer. The state machine uses the answer to identify the location to resume and generates the next choice. This request/response exchange continues until the end state is reached.

#### Define state machine types

The state machine is a chain of states expressed in a JSON format. Each state can have one of two types: `sendRequest`, or `matchResponse`. The `sendRequest` state instructs the workflow controller to generate a message and wait for the response. When the response arrives, the controller looks up the `matchResponse` state to identify where it should resume. Each request/response pair has a unique identifier. The identifier is a unique id that defines the context of a client/server message exchange. The final state is defined by a state without a `next` field.

The workflow controller generates one of the following `sendRequest` messages:

* **BotText** - sends the client an information field in text or HTML format.
* **ChoiceRequest** - sends a list of choices to the user. GroupId is the unique identifier paired with a *ChoiceResponse*.
* **StartChatSession** - asks the client to enable chat editor. SessionId is the unique identifier paired with a *UserText*
* **EndChatSession** - ask the client to disable chat session. Uses the SessionId paired with a *StartChatSession*

The Client replies with one of the following `mathResponse` messages:

* **ChoiceResponse** - send one of the choices in the *ChoiceRequest*.
* **UserText** - sends text generated by the user.

#### Define state transitions

The state transition have two flows:
* internal flows - driven by one or more _internal states_.
* external flows - driven by an _external state_. An external state tells the engine to generate a request and wait for the response before resuming.

Internal states have a `next` field whereas external states have a `sessionId` or `groupId` but no `next` field. Internal states are chained internally, whereas external are chained externally through a client response.

State transitions are triggered by a new connection or a client response. If it begins at an _internal state_, the engine collects the state information and moves to the next state until it encounters an _external state_. At that time, it generates a client request and waits for the response before it can resume.

<img src="/blog/images/bot-assistant/state-transitions.svg"
     alt="State Transitions"
     style="justify: center; max-width: 800px;" />

The client displays the request choices and asks the user to make a selection. Upon selection, the client generates a response and the cycle repeats.

Now that we have defined the state machine and the state transition, let's start the implementation.

#### Create a state machine JSON file

We'll create a state machine asks the user for their favorite programming language and collect their response.

Let's create `state-machine` directory and add `bot-assistant.json` file:

```bash
mkdir state-machines && touch state-machines/bot-assistant.json
```

Copy following state machine definition in the JSON file:

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

The state machine asks users for their favorite programming language and it presents them 3 options: `Rust`, `Go`, and `Other`.

If a user chooses `Rust` or `Go`, the state machine return:
* **anyOtherChoices** - another choice with `yes` or `no` answers.

For `Other`, it runs through the following states:
* **startLangPrefSession** opens an interactive session,
* **getLangPrefResponse** captures the user response,
* **endLangPrefSession** ends the interaction session.

This basic state machine show two different interaction models: a choice request/response or a user interaction. When the client receives a choice request, it presents the user with a series of choices. The user clicks on one of the choices and the client generates a response. For an interactive session, the client is asked to open an interactive session for the user to type his answer. After the server receives the response, it sends the client another request to close the interactive session. It is the responsibility of the server to manage access to the user editor.

Next, we need to load the JSON file into memory.

#### Add `state-machine.ts` file

The state machine is part of the workflow service that we'll define in the next section.

Create a `workflow-service` directory and add the `state-machine.ts` file:

```bash
mkdir src/workflow-service && touch src/workflow-service/state-machine.ts
```

Paste the following code in the `state-machine.ts` file:

```ts
import Fs from "fs";
import { RequestMessage, ResponseMessage } from "../messages";

type Name = string;

/* State Machine definition */
export type StateMachine = Map<Name, State>;

export interface State {
    sendRequest?: RequestMessage,
    matchResponse?: ResponseMessage;
    next?: string,
}

/* Load state machine from JSON file */
export function loadStateMachine(filePath: string) {
    const jsonFile = Fs.readFileSync(filePath);
    const jsonObject = JSON.parse(jsonFile.toString());

    const state_machine: StateMachine = new Map();
    for (var value in jsonObject) {
        state_machine.set(value, jsonObject[value])
    }

    return state_machine;
}
```

The code reads the JSON file, and provisions an internal state machine variable.

### Add workflow controller

The workflow controller is the mediator between the websocket proxy and the state machine. The controller receives messages from the client, computes the next state, generates a reply, and sends a response.

#### Add `workflow-controller.ts` file

Add the `workflow-controller.ts` file to the `workflow-service` directory:

```bash
touch src/workflow-service/workflow-controller.ts
```

Paste the following code:

```ts
import {
    SID,
    Message,
    ResponseMessage,
    ChoiceResponse,
    UserText,
    buildRequest,
    isRequest
} from "../messages";
import { StateMachine, State } from "./state-machine";
import { SessionController } from "../proxy-service/session-controller";

export class WorkflowController {
    private stateMachine: StateMachine;
    private initState: string;
    private sessionController: SessionController;

    constructor(
        stateMachine: StateMachine,
    ) {
        this.stateMachine = stateMachine;
        this.initState = stateMachine.keys().next().value;

        this.sessionController = Object();
    }

    public init(sessionController: SessionController) {
        this.sessionController = sessionController;
    }

    private processNewConnection(sid: SID) {
        const nextStates = this.processNext(this.initState);
        this.sendMessages(sid, nextStates);
    }

    private processNextState(sid: SID, response: ResponseMessage) {
        const state: string = this.getState(response);
        const nextStates = this.processNext(state);
        this.sendMessages(sid, nextStates);
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

        var state = this.stateMachine.get(startState);
        while (state) {
            nextStates.push(state);

            const next = state.next || "";
            state = this.stateMachine.get(next);
            if (next.length > 0 && !state) {
                console.error(`Error: Cannot find next state: ${next}`);
            }
        }

        return nextStates;
    }

    private getChoiceResponseState(choiceResponse: ChoiceResponse) {
        for (let [key, state] of this.stateMachine.entries()) {
            if (state.matchResponse &&
                state.matchResponse.kind == choiceResponse.kind &&
                state.matchResponse.groupId == choiceResponse.groupId &&
                state.matchResponse.itemId == choiceResponse.itemId) {
                return key;
            }
        }

        console.error(`Error: cannot find choice ${JSON.stringify(choiceResponse)}`);
        return this.initState;
    }

    private getUserTextState(userText: UserText) {
        for (let [key, state] of this.stateMachine.entries()) {
            if (state.matchResponse &&
                state.matchResponse.kind == "UserText" &&
                state.matchResponse.sessionId == userText.sessionId) {
                return key;
            }
        }

        console.error(`Error: cannot find user session ${JSON.stringify(userText)}`);
        return this.initState;
    }

    private sendMessages(sid: SID, nextStates: State[]) {
        for (let idx = 0; idx < nextStates.length; idx++) {
            const state = nextStates[idx];
            if (state.sendRequest) {
                const message = buildRequest(sid, state.sendRequest);
                this.sessionController.processBotMessage(JSON.stringify(message));
            }
        }
    }

    public processProxyMessage(clientMessage: string) {
        const message: Message = JSON.parse(clientMessage);
        if (!isRequest(message.payload)) {
            const sid = message.sid;
            if (message.payload) {
                this.processNextState(
                    sid,
                    <ResponseMessage>message.payload.message
                );
            } else {
                this.processNewConnection(sid);
            }
        }
    }
}
```

The workflow controller performs the following functions:
* **constructor**: caches a reference to the `stateMachine` and computes the initial state.
* **init**: caches a reference to the `sessionController`. This is done out of the constructor due to the circular reference. We'll come back to this when in the `Fluvio data streaming` section.
    * _Note_: The code does not compile until we add the session controller in the following section.
* **processProxyMessage**: is invoked by session controller to process a new client message. If the message has payload, it asks for next request, otherwise is needs the initial request:
    * **processNewConnection** reads the state machine from the first state and produces a request.
    * **processNextState** parses the client response, looks-up the resume state, and produces the next request.

The other APIs help the controller match a response and traverse the state machine to generate subsequent requests.

### Add proxy service

The proxy service has three components, incoming proxy `ProxyIn`, outgoing proxy `ProxyOut` and the session controller. The incoming proxy handles the websocket protocol, outgoing proxy sends messages based on a session id, and the session controller the interaction between the proxy and other services. 

For additional details, checkout [Websocket Glue for Data Streaming Apps](/blog/2021/01/websocket-glue-for-streaming-apps/).

#### Add outgoing proxy

Create a directory for the `proxy-service` and add `proxy-out.ts` file:

```bash
mkdir src/proxy-service && touch src/proxy-service/proxy-out.ts
```

Paste the following code:

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
}
```

As descried in the [websocket-glue blog], ProxyOut keeps a mapping between session session id and the websocket session.

[websocket-glue blog]: /blog/2021/01/websocket-glue-for-streaming-apps/

#### Add incoming proxy

Add `proxy-in.ts` file to manage the websocket protocol:

```bash
touch src/proxy-service/proxy-in.ts
```

Paste the following code:

```ts
import crypto from 'crypto';
import WS from "ws";
import http from "http";
import { SessionController } from "./session-controller";

const COOKIE_NAME = "Fluvio-Bot-Assistant"

export class WsProxyIn {
    private static wss: WS.Server;
    private static sessionController: SessionController;

    constructor(sessionController: SessionController) {
        WsProxyIn.wss = new WS.Server({ clientTracking: false, noServer: true });
        WsProxyIn.sessionController = sessionController;
    }

    public init(server: http.Server) {
        this.onUpgrade(server);
        this.onConnection();
    }

    private onUpgrade(server: http.Server) {
        server.on("upgrade", (request, socket, head) => {
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

        WsProxyIn.wss.on("connection", async (ws, req) => {
            const session_hdr = req.headers.session;
            const sid = ((Array.isArray(session_hdr)) ? session_hdr[0] : session_hdr) || "";
            await WsProxyIn.sessionController.sessionOpened(sid, ws);

            ws.on("close", async () => {
                await WsProxyIn.sessionController.sessionClosed(sid);
            });

            ws.on("message", async (clientMsg: string) => {
                await WsProxyIn.sessionController.messageFromClient(sid, clientMsg);
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
```

As descried in the [websocket-glue blog], the code accepts websocket connections, provisions cookies (`Fluvio-Bot-Assistant`), and passes the messages to the session controller.

#### Add session controller

Add `session-controller.ts` file:

```bash
touch src/proxy-service/session-controller.ts
```

Paste the following code:

```ts
import WS from "ws";
import { WsProxyOut } from "./proxy-out";
import { Message, SID, buildInitMessage, buildResponse, isRequest } from "../messages";
import { WorkflowController } from "../workflow-service/workflow-controller";

type Messages = Array<Message>;

export class SessionController {
    private sessionMessages: Map<SID, Messages>;
    private proxyOut: WsProxyOut;
    private workflowController: WorkflowController;

    constructor(
        proxyOut: WsProxyOut,
    ) {
        this.sessionMessages = new Map();
        
        this.proxyOut = proxyOut;
        this.workflowController = Object();
    }

    public init(workflowController: WorkflowController) {
        this.workflowController = workflowController;

        this.show();
    }

    public sessionOpened(sid: SID, ws: WS) {
        console.log(`start session - ${sid}`);

        this.proxyOut.addSession(sid, ws);

        const messages = this.sessionMessages.get(sid);
        if (messages) {
            this.sendMessagesToClient(messages);
        } else {
            const message = buildInitMessage(sid);
            this.workflowController.processProxyMessage(JSON.stringify(message));
        }
    }

    public sessionClosed(sid: SID) {
        console.log(`end session - ${sid}`);

        this.proxyOut.closeSession(sid);
    }


    public messageFromClient(sid: SID, clientMsg: string) {
        console.log(`${sid} <== ${clientMsg}`);

        const clientResponse = buildResponse(sid, JSON.parse(clientMsg));
        this.addMessageToSession(clientResponse);
        this.workflowController.processProxyMessage(JSON.stringify(clientResponse));
    }

    public sendMessagesToClient(messages: Messages) {
        messages.forEach(message => {
            this.sendMessageToClient(message);
        });
    }

    public sendMessageToClient(message: Message) {
        if (message.payload) {
            const clientMessage = message.payload.message;
            this.proxyOut.sendMessage(message.sid, JSON.stringify(clientMessage));
        }
    }

    private addMessageToSession(message: Message) {
        const sid = message.sid;
        var messages = this.sessionMessages.get(sid);
        if (!messages) {
            messages = new Array();
        }
        messages.push(message);
        this.sessionMessages.set(sid, messages);
    }

    public processBotMessage(botMessage: string) {
        const message: Message = JSON.parse(botMessage);
        this.addMessageToSession(message);

        if (isRequest(message.payload)) {
            this.sendMessageToClient(message);
        }
    }

    private show() {
        let table = new Map();
        for (let [sid, value] of this.sessionMessages) {
            table.set(sid, value.length);
        }
        console.table(table, ["SID", "Messages"]);
    }    
}
```

The session controller keeps a local copy of the messages exchanges anchored by session id. When a known session re-initiates a connection, the controller plays back the messages from memory. All other requests are passed along to the workflow controller.

We are now ready to add the `bot-server` file and initialize all server components.

### Add `bot-server.ts` file

Add `bot-server.ts` file in the `src` directory:

```bash
touch src/bot-server.ts
```

Paste the following code:

```ts
import { Server } from "http";
import { WsProxyIn } from "./proxy-service/proxy-in";
import { WsProxyOut } from "./proxy-service/proxy-out";
import { StateMachine, loadStateMachine } from "./workflow-service/state-machine";
import { WorkflowController } from "./workflow-service/workflow-controller";
import { SessionController } from "./proxy-service/session-controller";

export const initBotAssistant = (server: Server) => {

    const wsProxyOut = new WsProxyOut();
    const sessionController = new SessionController(wsProxyOut);
    const wsProxyIn = new WsProxyIn(sessionController);

    let filePath = getFileName();
    const stateMachine: StateMachine = loadStateMachine(filePath);
    const workflowController = new WorkflowController(stateMachine);

    sessionController.init(workflowController);
    workflowController.init(sessionController);

    wsProxyIn.init(server);
};

const getFileName = () => {
    if (process.argv.length != 3) {
        console.log("Usage: node bot-assistant.js <state-machine.json>");
        process.exit(1);
    }
    return process.argv[2];
}
```

The bot server file initializes all server components: incoming proxy, outgoing proxy, session controller, state machine and workflow controller.

To address circular reference challenges, it initializes `sessionController` and `workflowController` separately from the constructors. We'll come back to this in the [Fluvio data streaming](#step-4-add-data-streaming-and-persistency) section.

#### Update `bot-assistant.ts` file

The last step of the implementation integrates `initBotAssistant` into the `bot-assistant.ts` file.

Update the `bot-assistant.ts` file as follows:

{{< highlight ts "hl_lines=4 20" >}}
import http from "http";
import express from "express";
import path from 'path';
import { initBotAssistant } from "./bot-server";

const PORT = 9998;

const startServer = async () => {
    const app = express();
    const publicPath = path.join(__dirname, '..', 'public')

    app.get('/', (req, res) => {
        res.sendFile(path.join(publicPath, 'index.html'));
    });
    app.use("/scripts", express.static(path.join(publicPath, 'scripts')));
    app.use("/css", express.static(path.join(publicPath, 'css')));
    app.use("/img", express.static(path.join(publicPath, 'img')));

    const Server = http.createServer(app);
    await initBotAssistant(Server);

    Server.listen(PORT, () => {
        console.log(
            `started bot assistant server at http://localhost:${PORT}...`
        );
    });
};

process.on("uncaughtException", (e) => { console.log(e); process.exit(1); });
process.on("unhandledRejection", (e) => { console.log(e); process.exit(1); });

startServer();
{{< /highlight >}}

The code initializes the `bot-server` which needs access to HTTP server. Hence `initBotAssistant` is called after the `Server` is provisioned, and the server is passed through the function parameter.

## Start backend server

Let's start the server using the `bot-assistant.json` state machine file. Npm reads the command line parameters through environment variables:

```bash
PARAMS=state-machines/bot-assistant.json npm run start:server
```

The code should compile and run with the following console message:

```bash
4:35:59 PM - Starting compilation in watch mode...
4:36:01 PM - Found 0 errors. Watching for file changes.
┌───────────────────┬─────┬────────┐
│ (iteration index) │ Key │ Values │
├───────────────────┼─────┼────────┤
└───────────────────┴─────┴────────┘
started bot assistant server at http://localhost:9998...
```

## Step 3: Implement frontend client

The frontend client has two HTML components:
* `Bot` button,
* `Bot Assistant` dialog box

The `Bot` button is displayed on the lower right-hand side of the screen that opens the `Bot Assistant` dialog box. The dialog box is closed, the `Bot` button is shown again. In essence, the two components toggle each other on and off.

<img src="/blog/images/bot-assistant/bot-open-close.svg"
     alt="Bot Assistant Example"
     style="justify: center; max-width: 420px" />

The client builds the HTML components dynamically through javascript and it communicates with the web server through websocket.

The client is implemented in several steps:

* [Add `index.html` file](#add-indexhtml-file)
* [Add stylesheet file](#add-stylesheet-file)
* [Add assistant images](#add-assistant-images)
* [Add `assistant.js` script](#add-assistantjs-script)
* [Load `reconnecting-socket.js` file](#load-reconnecting-socketjs-file)
* [Test Bot Assistant (v1)](#test-bot-assistant-v1)

### Add `index.html` file

The front end client content is placed in the `public` directory. Let's add `index.html` file:

```bash
touch public/index.html
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

      <!-- debugging area - begin -->
      <textarea id="debugOutput" rows="20" cols="60" readonly></textarea>
      <!-- debugging area - end -->
   </body>
</html>
```

In the header we are referencing two files:
* `css/assistant.css` - styles file
* `scripts/assistant.js` - script file that builds the DOM elements.

In the body, there is a `div` with class named `assistant`. The script file looks-up this `div` to attach DOM elements. For troubleshooting, there is a `textarea` that prints debugging information.

### Add stylesheet file

The stylesheet controls the look and feel of the `Bot Assistant` button dialog box. 

Add a stylesheet called `assistant.css` to `public/css` directory:

```bash
mkdir public/css && touch public/css/assistant.css
```

Paste the following code in `assistant.css` file:

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
```

In summary the stylesheet has three sections, *Button* and *Chat Box*. 

The *Chat Box* has three subsections: a header with the bot icon, title and a close icon, the body area, and the footer. The footer has an editor for user input that is set to `read-only`.

### Add assistant images

The assistant button and chat dialog box uses several images to enhance the visualization. 

Let's create an `img` directory and use <a href="https://curl.se/docs/" target="_blank">curl</a> to download the images from github:

```bash
mkdir -p public/img/assistant
curl -L https://raw.githubusercontent.com/infinyon/fluvio-demo-apps-node/master/bot-assistant/public/img/assistant/note.svg --output public/img/assistant/note.svg
curl -L https://raw.githubusercontent.com/infinyon/fluvio-demo-apps-node/master/bot-assistant/public/img/assistant/bot.svg --output public/img/assistant/bot.svg
curl -L https://raw.githubusercontent.com/infinyon/fluvio-demo-apps-node/master/bot-assistant/public/img/assistant/redo.svg --output public/img/assistant/redo.svg
curl -L https://raw.githubusercontent.com/infinyon/fluvio-demo-apps-node/master/bot-assistant/public/img/assistant/close.svg --output public/img/assistant/close.svg
```

The script download 4 `svg` images: note, bot, redo and close.

### Add `assistant.js` script

The most important component of the frontend client is the `assistant.js` script. The script creates DOM elements, handles the user interaction, and communicates with the server.

Add the `assistant.js` file to the `public/scripts` directory:

```bash
mkdir public/scripts && touch public/scripts/assistant.js
```

Paste the following code in `assistant.js` file:

```javascript
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

    // Enable interactive chat
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

    // Capture editor keys
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
```

The script is invoked during <a href="https://developer.mozilla.org/en-US/docs/Web/API/Window/load_event" target="_blank">window.onload</a> event. The functions are as follows:
* **loadAssistant** - creates DOM elements for the button and editor, and attaches event listeners.
* **onOpenDialog** - shows dialog and hides button.
* **onCloseDialog** - shows button and hides dialog.
* **onResetSession** - clears the session cookie and establishes a new connection (a new cookie gets assigned).
* **openWsConnection** - connects to websocket server and attaches event listeners.
* **closeWsConnection** - closes the connection
* **onMessageFromServer** - parses messages received from the server and publishes the result in the chat editor.
* **sendWsMessage** - sends a message to the server.
* **loadScript** - loads another script into the DOM.
* **createElement** - a utility function that makes it easy to create DOM elements.

The other APIs are manipulating various DOM elements, enable/disable chat editor and clear the messages.

The script also loads `reconnecting-websocket.js` file which is discussed in the next section.

### Load `reconnecting-socket.js` file

The client is responsible for establishing and maintaining the connection to the server. While vanilla websocket offers the primitives to connect and disconnect to and from the server, it leaves it up to the user to implement reconnects.

<a href="https://github.com/joewalnes" target="_blank">Joe Walnes</a> has written a great utility called <a href="https://github.com/joewalnes/reconnecting-websocket" target="_blank">reconnecting-socket.js</a> that that that implements the reconnection logic under the hood.

Let's copy the file in the `public/scripts` directory:

```bash
curl -L https://raw.githubusercontent.com/infinyon/fluvio-demo-apps-node/master/bot-assistant/public/scripts/reconnecting-socket.js --output public/scripts/reconnecting-socket.js
```

Let's review the `public` directory hierarchy:

```bash
public
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
    ├── assistant.js
    └── reconnecting-socket.js
```

The frontend client is available for download in github.

### Test Bot Assistant (v1)

Ensure the server is running, otherwise run the following command:

```bash
PARAMS=state-machines/bot-assistant.json npm run start:server
```

In the web browser, open `http://localhost:9999/`, then click on "Bot Assistant` button.  Click on the choices and see the bot assistant traverse through our state machine:

<img src="/blog/images/bot-assistant/workflow-end-to-end.svg"
     alt="Workflow end-to-end"
     style="justify: center; max-width: 700px" />

Congratulations, `Bot Assistant` is up and running.


## Step 4: Add data streaming and persistency

As seen in the previous section, `Bot Assistant` works well, but it has some limitations. If the webserver restarts, all messages are lost and all user sessions are reset. 

We'll use [Fluvio](https://fluvio.io) to remediate this issue. Fluvio is a high throughput, low latency data streaming platform that scales horizontally to handle persistency for a large number of concurrent messages. 
 
We can deploy Fluvio between the connection proxy and the workflow controller, which also enables us to divide our monolith into two independent services (aka. microservices): `Proxy Service` and `Workflow Service`:

<img src="/blog/images/bot-assistant/architecture.svg"
     alt="Bot Assistant Architecture"
     style="justify: center; max-width: 780px" />

When services are bridged by Fluvio we gain additional benefits:

* **scale** the proxy and workflow independently of each other. 

* **handoff** a conversation to a human operator. We can do that by adding an `operator service` independently that interacts directly with the client through the data stream.

* **add-on services** such as: analytics, machine learning, or connectors to other products.

We can also remove the _circular reference_ hack we implemented between `session-controller` and `workflow-controller`.

-> **Prerequisites:** This section assumes you have access to a Fluvio cluster. If you don't have access to a cluster, check out the [getting started guide](/docs/getting-started) to create [Fluvio Cloud](/docs/getting-started/fluvio-cloud/) account.

To integrate Fluvio data streaming we'll make the following changes:

* [Add fluvio to `session-controller`](#add-fluvio-to-session-controller)
* [Add fluvio to `workflow-controller`](#add-fluvio-to-workflow-controller)
* [Add fluvio to `bot-server`](#add-fluvio-to-bot-server)
* [Add fluvio setup script](#add-fluvio-setup-script)

### Add fluvio to `session-controller`

In the `session-controller.ts` file we replace references to `workflow-controller` with fluvio producers. In addition to that, the session controller can now use fluvio to look-up all transaction for a specific session.

Update `src/proxy-service/session-controller.ts` with the following code changes:

{{< highlight typescript "hl_lines=4 11-12 16-17 22-23 26 27-29 33-35 38 48 59 63" >}}
import WS from "ws";
import { WsProxyOut } from "./proxy-out";
import { Message, SID, buildInitMessage, buildResponse, isRequest } from "../messages";
import { TopicProducer, PartitionConsumer, Offset } from "@fluvio/client";

type Messages = Array<Message>;

export class SessionController {
    private sessionMessages: Map<SID, Messages>;
    private proxyOut: WsProxyOut;
    private fluvioProducer: TopicProducer;
    private fluvioConsumer: PartitionConsumer;

    constructor(
        proxyOut: WsProxyOut,
        fluvioProducer: TopicProducer,
        fluvioConsumer: PartitionConsumer
    ) {
        this.sessionMessages = new Map();

        this.proxyOut = proxyOut;
        this.fluvioProducer = fluvioProducer;
        this.fluvioConsumer = fluvioConsumer;
    }

    public async init() {
        (await this.fluvioConsumer.fetch(Offset.FromBeginning())).toRecords().forEach(msg => {
            this.addMessageToSession(JSON.parse(msg));
        });

        this.show();

        this.fluvioConsumer.stream(Offset.FromEnd(), (msg: string) => {
            this.processBotMessage(msg);
        });
    }

    public async sessionOpened(sid: SID, ws: WS) {
        console.log(`start session - ${sid}`);

        this.proxyOut.addSession(sid, ws);

        const messages = this.sessionMessages.get(sid);
        if (messages) {
            this.sendMessagesToClient(messages);
        } else {
            const message = buildInitMessage(sid);
            await this.fluvioProducer.sendRecord(JSON.stringify(message), 0);
        }
    }

    public sessionClosed(sid: SID) {
        console.log(`end session - ${sid}`);

        this.proxyOut.closeSession(sid);
    }


    public async messageFromClient(sid: SID, clientMsg: string) {
        console.log(`${sid} <== ${clientMsg}`);

        const clientResponse = buildResponse(sid, JSON.parse(clientMsg));
        await this.fluvioProducer.sendRecord(JSON.stringify(clientResponse), 0);
    }

    public sendMessagesToClient(messages: Messages) {
        messages.forEach(message => {
            this.sendMessageToClient(message);
        });
    }

    public sendMessageToClient(message: Message) {
        if (message.payload) {
            const clientMessage = message.payload.message;
            this.proxyOut.sendMessage(message.sid, JSON.stringify(clientMessage));
        }
    }

    private addMessageToSession(message: Message) {
        const sid = message.sid;
        var messages = this.sessionMessages.get(sid);
        if (!messages) {
            messages = new Array();
        }
        messages.push(message);
        this.sessionMessages.set(sid, messages);
    }

    public processBotMessage(botMessage: string) {
        const message: Message = JSON.parse(botMessage);
        this.addMessageToSession(message);

        if (isRequest(message.payload)) {
            this.sendMessageToClient(message);
        }
    }

    private show() {
        let table = new Map();
        for (let [sid, value] of this.sessionMessages) {
            table.set(sid, value.length);
        }
        console.table(table, ["SID", "Messages"]);
    }
}
{{< /highlight >}}

The code changes are as follows:
* **constructor** - saves fluvio `topicProducer` and `topicConsumer` in a local variable.
* **init**:
    * made `async`,
    * to fetch fluvio messages and cache them in `sessionMessages` array,
    * to register `processBotMessage` callback to `fluvioConsumer`.
* **sessionOpened** - made `async` to write a new message to the fluvio data stream.
* **messageFromClient** - made `async` to write client messages to fluvio data stream (instead of calling workflow-controller).

That's it, `session-controller` can now be deployed as a stand-alone service without any dependencies on `workflow service`.

### Add fluvio to `workflow-controller`

Similarly, in the `workflow-controller.ts` file we replace references to `session-controller` with fluvio producers.

Update `src/workflow-service/workflow-controller.ts` with the following code changes:

{{< highlight typescript "hl_lines=11 16-17 21-22 27-28 31-35 37 39 42 45 103 108 113 123" >}}
import {
    SID,
    Message,
    ResponseMessage,
    ChoiceResponse,
    UserText,
    buildRequest,
    isRequest
} from "../messages";
import { StateMachine, State } from "./state-machine";
import { TopicProducer, PartitionConsumer, Offset } from "@fluvio/client";

export class WorkflowController {
    private stateMachine: StateMachine;
    private initState: string;
    private fluvioProducer: TopicProducer;
    private fluvioConsumer: PartitionConsumer;

    constructor(
        stateMachine: StateMachine,
        fluvioProducer: TopicProducer,
        fluvioConsumer: PartitionConsumer,
    ) {
        this.stateMachine = stateMachine;
        this.initState = stateMachine.keys().next().value;

        this.fluvioProducer = fluvioProducer;
        this.fluvioConsumer = fluvioConsumer;
    }

    public init() {
        this.fluvioConsumer.stream(Offset.FromEnd(), async (sessionMsg: string) => {
            await this.processProxyMessage(sessionMsg);
        });
    }

    private async processNewConnection(sid: SID) {
        const nextStates = this.processNext(this.initState);
        await this.sendMessages(sid, nextStates);
    }

    private async processNextState(sid: SID, response: ResponseMessage) {
        const state: string = this.getState(response);
        const nextStates = this.processNext(state);
        await this.sendMessages(sid, nextStates);
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

        var state = this.stateMachine.get(startState);
        while (state) {
            nextStates.push(state);

            const next = state.next || "";
            state = this.stateMachine.get(next);
            if (next.length > 0 && !state) {
                console.error(`Error: Cannot find next state: ${next}`);
            }
        }

        return nextStates;
    }

    private getChoiceResponseState(choiceResponse: ChoiceResponse) {
        for (let [key, state] of this.stateMachine.entries()) {
            if (state.matchResponse &&
                state.matchResponse.kind == choiceResponse.kind &&
                state.matchResponse.groupId == choiceResponse.groupId &&
                state.matchResponse.itemId == choiceResponse.itemId) {
                return key;
            }
        }

        console.error(`Error: cannot find choice ${JSON.stringify(choiceResponse)}`);
        return this.initState;
    }

    private getUserTextState(userText: UserText) {
        for (let [key, state] of this.stateMachine.entries()) {
            if (state.matchResponse &&
                state.matchResponse.kind == "UserText" &&
                state.matchResponse.sessionId == userText.sessionId) {
                return key;
            }
        }

        console.error(`Error: cannot find user session ${JSON.stringify(userText)}`);
        return this.initState;
    }

    private async sendMessages(sid: SID, nextStates: State[]) {
        for (let idx = 0; idx < nextStates.length; idx++) {
            const state = nextStates[idx];
            if (state.sendRequest) {
                const message = buildRequest(sid, state.sendRequest);
                await this.fluvioProducer.sendRecord(JSON.stringify(message), 0);
            }
        }
    }

    public async processProxyMessage(clientMessage: string) {
        const message: Message = JSON.parse(clientMessage);
        if (!isRequest(message.payload)) {
            const sid = message.sid;
            if (message.payload) {
                this.processNextState(
                    sid,
                    <ResponseMessage>message.payload.message
                );
            } else {
                await this.processNewConnection(sid);
            }
        }
    }
}
{{< /highlight >}}

The code changes are as follows:
* **constructor** - saves fluvio `topicProducer` and `topicConsumer` in a local variable.
* **init** - registers `processProxyMessage` to callback `fluvioConsumer`.
* **processNewConnection** - made `async` to use sendMessages.
* **processNextState** - made `async` to use sendMessages.
* **sendMessages** -  made `async` to write client message to the fluvio data stream (instead of calling session-controller).
* **processProxyMessage** - made `async` to `processNewConnection`.

The workflow controller is now a stand-alone service decoupled from `session-controller`. The Fluvio middle tier allows these services to be moved to a different machine and be scaled-up independently. However, this improvement is beyond the scope of this blog.

### Add fluvio to `bot-server`

The `bot-server` is responsible for the initialization of the producer and consumer. After initialization, the producer and the consumer are passed to the `session-controller` and `workflow-controller` for processing.

Update `src/bot-server.ts` with the following code changes:

{{< highlight typescript "hl_lines=7 9 11 13-16 19 24 26-27 40-46" >}}
import { Server } from "http";
import { WsProxyIn } from "./proxy-service/proxy-in";
import { WsProxyOut } from "./proxy-service/proxy-out";
import { StateMachine, loadStateMachine } from "./workflow-service/state-machine";
import { WorkflowController } from "./workflow-service/workflow-controller";
import { SessionController } from "./proxy-service/session-controller";
import Fluvio from '@fluvio/client';

const BOT_ASSIST_MESSAGES = "bot-assist-messages";

export const initBotAssistant = async (server: Server) => {

    const fluvio = await Fluvio.connect();
    await checkTopic(fluvio);
    const fluvioProducer = await fluvio.topicProducer(BOT_ASSIST_MESSAGES);
    const fluvioConsumer = await fluvio.partitionConsumer(BOT_ASSIST_MESSAGES, 0);

    const wsProxyOut = new WsProxyOut();
    const sessionController = new SessionController(wsProxyOut, fluvioProducer, fluvioConsumer);
    const wsProxyIn = new WsProxyIn(sessionController);

    let filePath = getFileName();
    const stateMachine: StateMachine = loadStateMachine(filePath);
    const workflowController = new WorkflowController(stateMachine, fluvioProducer, fluvioConsumer);

    await sessionController.init();
    workflowController.init();

    wsProxyIn.init(server);
};

const getFileName = () => {
    if (process.argv.length != 3) {
        console.log("Usage: node bot-assistant.js <state-machine.json>");
        process.exit(1);
    }
    return process.argv[2];
}

const checkTopic = async (fluvio: Fluvio) => {
    const admin = await fluvio.admin();
    if (!await admin.findTopic(BOT_ASSIST_MESSAGES)) {
        console.error("Error: Fluvio topic not found! Run `npm run setup`");
        process.exit(1);
    }
}
{{< /highlight >}}

The code changes are as follows:
* **BOT_ASSIST_MESSAGES** - defines bot assistant topic name.
* **fluvio** - connects to fluvio, checks topic existence, and provisions `fluvioProducer` and `fluvioConsumer`.
* **SessionController** - passes `fluvioProducer` and `fluvioConsumer` to session controller.
* **WorkflowController** - passes `fluvioProducer` and `fluvioConsumer` to workflow controller.

Congratulations! You made all code changes for `Bot Assistant`. Next, we'll add couple of scripts to add/remove topic and we are ready for testing.

### Add fluvio setup script

Fluvio needs a setup script to perform administrative operations such as add/remove topics. Let's add a couple of files to perform these operations and link the files with npm.

-> This section assumes that the [Fluvio CLI](/docs/getting-started/) is installed on your machine.

Create a `tools` directory and add `setup.sh` and `cleanup.sh` files:

```bash
mkdir tools
touch tools/setup.sh && chmod +x tools/setup.sh
touch tools/cleanup.sh && chmod +x tools/cleanup.sh
```

Paste the following in the `setup.sh` file:

```bash
#!/bin/bash
fluvio topic create bot-assist-messages
```

Paste the following in the `cleanup.sh` file:

```bash
#!/bin/bash
fluvio topic delete bot-assist-messages
```

Finally, update `package.json` file to link the script files:

{{< highlight json "hl_lines=8-9" >}}
{
  "name": "bot-assistant",
  "version": "1.0.0",
  "description": "",
  "main": "bot-assistant.js",
  "scripts": {
    "start:server": "tsc-watch --onSuccess \"node ./dist/bot-assistant.js $PARAMS\"",
    "setup": "sh ./tools/setup.sh",
    "cleanup": "sh ./tools/cleanup.sh"
  },
  "keywords": [],
  "author": "fluvio <admin@fluvio.io> (fluvio.io)",
  "license": "Apache 2.0",
  "dependencies": {
    "@fluvio/client": "^0.6.0-beta.3",
    "express": "^4.17.1",
    "typescript": "^4.1.3",
    "ws": "^7.4.2"
  },
  "devDependencies": {
    "@types/express": "^4.17.9",
    "@types/node": "^14.14.19",
    "@types/ws": "^7.4.0",
    "tsc-watch": "^4.2.9"
  }
}
{{< /highlight >}}

Let's run setup, to create the new topic:

```bash
npm run setup
```

If the command was successful, you should see the following message:

```bash
> bot-assistant@1.0.0 setup /projects/bot-assistant
> sh ./tools/setup.sh

topic "bot-assist-messages" created
```

Congratulations! You have completed changes in the for `Bot Assistant` project.

### Test Bot Assistant

Repeat the tests in [Test Bot Assistant (v1)](#test-bot-assistant-v1) and refresh the screen. Note that the messages are refreshed as they have been persistent by Fluvio.

The persistence also survives server reboots. Go ahead and reboot the server and refresh the browser screen. Notice how the messages are preserved.

Furthermore, you can now use Fluvio or other programs with a Fluvio consumer interface to read session messages.

Let's read the last 5 messages with [Fluvio CLI](/docs/getting-started/):

```bash 
fluvio consume bot-assist-messages --offset="-4"
```

The result is as follows:

```bash
{"sid":"fb7e2971d989070361c30d825bf6a853a406916e","payload":{"kind":"Response","message":{"kind":"ChoiceResponse","groupId":"others","itemId":"no","content":"No"}},"timestamp":"2021-01-05T17:58:46.511"}
{"sid":"fb7e2971d989070361c30d825bf6a853a406916e","payload":{"kind":"Request","message":{"kind":"BotText","content":"Great, thanks!"}},"timestamp":"2021-01-05T17:58:46.514"}
{"sid":"fb7e2971d989070361c30d825bf6a853a406916e","payload":{"kind":"Response","message":{"kind":"ResetSession"}},"timestamp":"2021-01-05T18:04:20.811"}
{"sid":"0fb1574b3b6d7c98c7089aa4a2c58a80894bbc6e","timestamp":"2021-01-05T18:04:20.815"}
{"sid":"0fb1574b3b6d7c98c7089aa4a2c58a80894bbc6e","payload":{"kind":"Request","message":{"kind":"BotText","content":"Hi, I'm Bot! Nice to meet you."}},"timestamp":"2021-01-05T18:04:20.817"}
```

When you use fluvio to transfer real-time messages between services, you gain much more than a transport layer. Benefits range from decoupling service to recovering from errors, from monitoring to troubleshooting and much more. 

## Conclusion

In this blog, we explored how to build a robot assistant that can interact with users in real-time. By using Fluvio, our application has a foundation that allows it to scale horizontally to meet the demands of a massive user audience. Since our backend services are stateless and decoupled, we can scale them independently to handle the particular load characteristics that we observe in production, preventing technical bottlenecks.

This project also just scratches the surface of what a real-time streaming application can do. By leveraging Fluvio's persistent data streams, we can build improvements to our application by simply writing new microservices that interact with topic data. This gives us the power to develop new real-time features, as well as to analyze historical data for purposes such as Machine Learning personalization use-cases.

<p>
We hope you enjoyed the blog! If you have any questions or comments, or if you just want to come say hi, you can find us on our community Discord channel →
<a href="https://discordapp.com/invite/bBG2dTz">
<img style="height:8%;width:auto;display:inline" src="https://img.shields.io/discord/695712741381636168?color=738ADB&logo=Discord" alt="Discord">
</a>
</p>
