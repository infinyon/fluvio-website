---
title: Build a Node App
weight: 20
---

In this guide we’ll cover: how to set up your {{< target-blank title="Node.js" url="https://nodejs.org" >}} environment, and how to build a simple data streaming App.

## Setup Node Environment

In addition to Node.js, Fluvio also requires a Rust compiler and a conversion tool that binds Fluvio calls to Node. If you have Node installed ensure it runs **version 13** or above.

#### Install Node.js

Node.js installation varies depending on your operating system.

{{< api-table >}}
|   Operating System     |         Instructions           |
|---|---|
| MacOS      | Use the official installer from {{< target-blank title="Node.js" url="https://nodejs.org" >}} to install on **macOS**.  |
| Windows    | Use the official installer from {{< target-blank title="Node.js" url="https://nodejs.org" >}} to install on **Windows**. |
| Linux     | Use the instructions provided by your **Linux** package manager. <br/> Node.js maintains a list of supported packages {{< target-blank title="here" url="https://nodejs.org/en/download/package-manager" >}}.  |
{{< /api-table >}}

#### Install Rust

Rust language provides installer which different installation instructions based on your operating system. Refer to {{< target-blank title="rustup" url="https://rustup.rs" >}} to instructions.


## Build a simple data streaming App

This section provides a step-by-step on how to build a simple data streaming app. If you'd like to download the app instead, skip ahead to [Download Fluvio data streaming App]({{< relref "#download-fluvio-data-streaming-app" >}}).


#### Start a new Node project

Create a directory for **fluvio-app**:

{{< code style="light" >}}
$ mkdir fluvio-app
$ cd fluvio-app
{{< /code >}}

Run npm to create a **node project** and generate package.json file:

{{< code lang="json" style="light" >}}
$ npm init -y
Wrote to /Users/user/fluvio-app/package.json:
{
  "name": "fluvio-app",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [],
  "author": "fluvio <user@fluvio.io> (fluvio.io)",
  "license": "Apache 2.0"
}
{{< /code >}}



## Download Fluvio data streaming App

The app is available for download on {{< target-blank title="github" url="https://github.com/infinyon/node-demo-apps" >}}. 



# ----

Congratulation! You have successfully sent your first message!

{{< links "Related Topics" >}}
* [Install Fluvio]({{< relref "install-fluvio" >}})
* [Install CLI]({{< relref "install-cli" >}})
{{< /links >}}