---
title: Build your own Robot Assistant using data streams
toc: true
---


This guide provides an example of using `@fluvio/client` to write a fully functional robot assistant using Node.js. 

To build the robot assistant one step at a time, checkout our blog:
* [Build Your Own Custom Robot Assistant](/blog/2021/01/bot-assistant/)

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

Once cloned, navigate to the `bot-assistant` directory:

```bash
cd ./fluvio-demo-apps-node/bot-assistant
```

The rest of the commands are ran in the `bot-assistant` directory.

## Application Overview

Robot assistant has a client and a server. The client runs in the web browser and controls the frontend user interaction, while the server manages the WebSocket proxy and the state machine. The client and the server communicate with each other through a WebSocket.

{{< image src="tutorials/bot-assistant/architecture.svg" alt="Bot Assistant Architecture" justify="center" width="780" type="scaled-98">}}

The server uses `@fluvio/client` Node.js API to communicate with Fluvio. The API reference guide is available 
<a href="https://infinyon.github.io/fluvio-client-node/" target="_blank">here</a>.

## Running the Robot Assistant App

In a terminal window, from `bot-assistant` directory, run the following command:

```bash
npm run setup
```

This command runs `tools/setup.sh` script, which creates `bot-assist-messages` topic. Next, install the packages:

```bash
npm install
```

With the packages installed, we are ready to start the server.


## **Run the Server**

The server uses a state machine to control the robot assistant. The states are defined in a custom JSON file. There are a couple of template files available in the `state-machine` directory. For detailed information checkout [add-state-machine](/blog/2021/01/bot-assistant/#add-state-machine) section in the blog.

In the terminal window, run the following command:

```bash
PARAMS=state-machines/fluvio.json npm run start:server
```

The server reads the state machine, connects to Fluvio, and listens for client messages. If the server successfully started, you should see the following message:

```bash
Found 0 errors. Watching for file changes.
┌───────────────────┬─────┬────────┐
│ (iteration index) │ Key │ Values │
├───────────────────┼─────┼────────┤
└───────────────────┴─────┴────────┘
started bot assistant server at http://localhost:9998...
```

Open the web browser at [http://localhost:9998](http://localhost:9998) to interact with bot assistant.


## Using the Application

Bot Assistant writes all messages to a debugging window to make the application easier to track. In the initial state, Bot Assistant displays a small icon on the bottom right side of the screen. 

{{< image src="tutorials/bot-assistant/start-here.svg" alt="Bot Assistant" justify="center" width="450" type="scaled-75">}}

Just follow the instructions and the bot should guide you through the rest of the workflow.

{{< image src="tutorials/bot-assistant/bot-assistant.svg" alt="Bot Assistant" justify="center" width="360" type="scaled-75">}}

Click `restart` to reset the session. Fluvio also offers persistence, reset the server and watch it initialize its internal state and resume from where it left off.


## Fluvio Data Streaming

Fluvio stores messages/events in a topic:

* **bot-assist-messages**

While Robot Assistant uses these events for persistence, they are also available for other purposes. For example, if another service is interested in user activity, the service can hook-up a consumer and receive real-time events.

The data stream can also serve as a mediation layer for other producers such as `human operators`. Adding human operator is relatively straightforward and left as an exercise for the user.

The messages can be viewed in Fluvio:

```bash
fluvio consume bot-assist-messages -B
```

Option `-B` display messages from the beginning:

```json
{"sid":"5bf749550eb989aaeb924dc1b4c143daec71986f","timestamp":"2021-03-11T22:29:18.905"}
{"sid":"5bf749550eb989aaeb924dc1b4c143daec71986f","payload":{"kind":"Request","message":{"kind":"BotText","content":"Hi, I'm Bot! Nice to meet you."}},"timestamp":"2021-03-11T22:29:19.077"}
{"sid":"5bf749550eb989aaeb924dc1b4c143daec71986f","payload":{"kind":"Request","message":{"kind":"ChoiceRequest","groupId":"lang","question":"What programming language do you use in your hobby projects?","choices":[{"itemId":"rust","content":"Rust"},{"itemId":"go","content":"Go"},{"itemId":"other","content":"Other"}]}},"timestamp":"2021-03-11T22:29:19.346"}
{"sid":"e34678b5ce0586560bfd034468f76c210e282855","timestamp":"2021-03-11T22:35:44.261"}
{"sid":"e34678b5ce0586560bfd034468f76c210e282855","payload":{"kind":"Request","message":{"kind":"BotText","content":"Hi, I'm Bot! Nice to meet you."}},"timestamp":"2021-03-11T22:35:44.438"}
{"sid":"e34678b5ce0586560bfd034468f76c210e282855","payload":{"kind":"Request","message":{"kind":"ChoiceRequest","groupId":"lang","question":"What programming language do you use in your hobby projects?","choices":[{"itemId":"rust","content":"Rust"},{"itemId":"go","content":"Go"},{"itemId":"other","content":"Other"}]}},"timestamp":"2021-03-11T22:35:44.716"}
{"sid":"e34678b5ce0586560bfd034468f76c210e282855","payload":{"kind":"Response","message":{"kind":"ChoiceResponse","groupId":"lang","itemId":"go","content":"Go"}},"timestamp":"2021-03-11T22:35:46.373"}
{"sid":"e34678b5ce0586560bfd034468f76c210e282855","payload":{"kind":"Request","message":{"kind":"ChoiceRequest","groupId":"others","question":"Any other?","choices":[{"itemId":"yes","content":"Yes"},{"itemId":"no","content":"No"}]}},"timestamp":"2021-03-11T22:35:46.548"}
```

Each session exchanged are grouped by the session id.

## Conclusion

Fluvio data streaming platform can turn simple prototypes into powerful event drive applications. In this example, we used Fluvio for persistence and inter service communication. When services communicate with each other in real-time, it opens the door for a new class of services beyond the capability of any database.


For additional information, checkout the following related articles:
* [Build Your Own Custom Robot Assistant](/blog/2021/01/bot-assistant/)
* [Node API documentation](https://infinyon.github.io/fluvio-client-node/)