---
title: Start Server
menu: Start Server 
weight: 30
---

Now that we've got all of our tools installed, let's actually get Fluvio up and running!
If you run into any problems along the way, make sure to check out our [troubleshooting]
page to find a fix.

[troubleshooting]: {{< ref "/docs/operations/troubleshoot.md" >}}

## Local server
Currently, the local server still requires the usage of Kubernetes. Currently Fluvio stores server metadata using Kubernetes. However you can run the Fluvio processes directly on the host

```bash
$ fluvio cluster start --local
```

## Kubernetes server

```bash
$ fluvio cluster start
✅ ok: Supported helm version is installed
✅ ok: Fluvio system charts are installed
✅ ok: Previous fluvio installation not found
Waiting up to 120 seconds for Fluvio cluster version check...
Successfully installed Fluvio!
```

You can check that everything worked by listing out the cluster's SPUs:

```bash
$ fluvio cluster spu list
 ID  NAME    STATUS  TYPE       RACK  PUBLIC  PRIVATE 
  0  main-0  Online  "managed"   -    :9005   fluvio-spg-main-0.fluvio-spg-main:9006
```

## Hello, Fluvio!

Congratulations, you've successfully installed Fluvio on your local machine! 
Let's use the Fluvio CLI to play with some basic functionality.

The first thing we need to do is create a Fluvio topic. A Topic is like a
category where related events live together. We can create a new topic with
the following command:

```bash
$ fluvio topic create greetings
topic "greetings" created
```

Now that we have a topic, we can produce some messages! Use the following
command to send a message to the `greetings` topic:

```bash
$ echo "Hello, Fluvio" | fluvio produce greetings
Ok!
```

Finally, we can consume messages back from the topic

```bash
$ fluvio consume greetings -B -d
Hello, Fluvio
```

Way to go! You're well on your way to writing real-time distributed apps
with Fluvio! Next, check out our [Tutorials page] to see real-world examples
of Fluvio in action.

[Tutorials page]: https://www.infinyon.com/tutorials 

#### Related Topics
----------------

- ["Hello World" in Java](https://www.infinyon.com/tutorials/java/hello-world/)
- ["Hello World" in Node.js](https://www.infinyon.com/tutorials/node/hello-world/)
- ["Hello World" in Python](https://www.infinyon.com/tutorials/python/hello-world/)
- ["Hello World" in Rust](https://www.infinyon.com/tutorials/rust/hello-world/)