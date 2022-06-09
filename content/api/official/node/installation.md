---
title: Node SDK Installation Guide
menu: Installation guide
weight: 10
---

Run the following commands to set up your project for development:

%copy%

```bash
mkdir fluvio-demo && cd fluvio-demo
npm init -y
npm install -D typescript ts-node @types/node
npm install -S @fluvio/client
touch producer.ts consumer.ts
```

Your working directory should now contain the following files:

%copy first-line%

```bash
$ ls
consumer.ts  node_modules  package-lock.json  package.json  producer.ts
```

And your `package.json` should have the following:

{{<code file="code/node/package.json" firstLine="cat package.json" copy=true >}}

