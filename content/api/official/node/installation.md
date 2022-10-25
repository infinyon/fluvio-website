---
title: Node SDK Installation Guide
menu: Installation guide
weight: 10
---


## Install Node tools

Install [Node.js](https://nodejs.org/en/) (v**16.11.0** or above)

We are using [`npm`](https://nodejs.dev/en/learn/an-introduction-to-the-npm-package-manager/) as package manager.

## Create New Node project for Fluvio development

Run the following commands to set up your project for development:

%copy%

```bash
mkdir fluvio-demo && cd fluvio-demo
npm init -y
npm install -D typescript ts-node @types/node
npm install -S @fluvio/client
```

And your `package.json` should look similar to the following:

{{<code file="code-blocks/client-examples/node/package.json" lang="json" copy=true >}}