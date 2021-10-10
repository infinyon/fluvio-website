---
title: MacOS Installation
menu: MacOS
weight: 10
---

## Install Fluvio CLI

The Fluvio CLI (_command-line interface_) is an all-in-one tool for setting up, interacting, and managing with Fluvio clusters.

Install the Fluvio CLI by running the following command:

%copy first-line%
```bash
curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

## Required Packages for Local Fluvio cluster

1) [Minikube]({{< ref "#install-minikube" >}})
2) [Kubectl]({{< ref "#install-kubectl" >}})
3) [Helm]({{< ref "#install-helm" >}})

If you have `kubectl`, `helm`, and `minikube` already set up, then continue to steps for [running a local Fluvio cluster].



### Install Minikube

Minikube is a tool for running a local Kubernetes cluster

Install `minikube` with following command:

%copy first-line%

```bash
$ curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-darwin-amd64
$ sudo install minikube-darwin-amd64 /usr/local/bin/minikube
```

For detail instruction see:
[Minikube installation page]: https://minikube.sigs.k8s.io/docs/start/

#### Start a Kubernetes cluster
Start a Kubernetes cluster locally with minikube by running the following in a terminal window:

%copy first-line%
```bash
$ minikube start --driver=virtualbox
```

Other minikube driver may work but we have not tested them.  Please open issue if need to support other drivers.

### Install Kubectl

`kubectl` is the Kubernetes command-line tool. It is used to run commands against Kubernetes clusters.

Install `kubectl` with the [Brew Package Manager] by running the following in a terminal window:

%copy first-line%

```bash
$ brew install kubectl 
```

Or follow the instructions at the [kubectl installation page] and follow the instructions to download and install `kubectl` on MacOS.

[kubectl installation page]: https://kubernetes.io/docs/tasks/tools/install-kubectl-macos/ 

### Install Helm

Helm is the package manager for Kubernetes. 

Install `helm` with the [Brew Package Manager] by running the following in a terminal window:

%copy first-line%

```bash
$ brew install helm 
```

Or follow the instructions at the [helm installation page] and follow the instructions to download and install `helm` on MacOS.

[helm installation page]: https://v3.helm.sh/docs/intro/install/ 
## Start Fluvio cluster 

You can start a Fluvio cluster by running `fluvio cluster start`.

If this is your first time starting a cluster in your session, be prepared to enter your password.

%copy first-line%
```bash
$ fluvio cluster start
```

### Verify cluster is running

We can check the fluvio cluster by checking version and status with the following command:

```bash
$ fluvio version

Fluvio CLI           : 0.9.10
Fluvio CLI SHA256    : a2d5cdd58511c94ee35963acc6b9b7d334d2bbc2571663d958a8e0db7d1af37c
Fluvio Platform      : 0.9.10 (minikube)
Git Commit           : c00a1ee2cd28545443f9a7cbf2ca9d053e67845b
OS Details           : Darwin 10.16 (kernel 20.6.0)
=== Plugin Versions ===
Fluvio Runner (fluvio-run)     : 0.0.0
Infinyon Cloud CLI (fluvio-cloud) : 0.1.6

```

## Hello, Fluvio!

Congratulations, you've successfully installed Fluvio on your local machine! 

Let's use the Fluvio CLI to play with some basic functionality.

The first thing we need to do is create a [topic].

%copy first-line%
```bash
$ fluvio topic create greetings
topic "greetings" created
```

Now that we have a topic, we can [produce] some messages!

Use the following command to send a message to the `greetings` topic:

%copy first-line%
```bash
$ echo "Hello, Fluvio" | fluvio produce greetings
```

Finally, we can [consume] messages back from the topic

%copy first-line%
```bash
$ fluvio consume greetings -B -d
Consuming records from the beginning of topic 'greetings'
Hello, Fluvio
```

Way to go! You're well on your way to writing real-time distributed apps with Fluvio!

Next, check out our [Tutorials page] to see real-world examples of Fluvio in action.

[topic]: {{< ref "/cli/commands/topic.md" >}}
[produce]: {{< ref "/cli/commands/produce.md" >}}
[consume]: {{< ref "/cli/commands/consume.md" >}}
[Tutorials page]: https://www.infinyon.com/tutorials 

#### Related Topics
----------------

- ["Hello World" in Java](https://www.infinyon.com/tutorials/java/hello-world/)
- ["Hello World" in Node.js](https://www.infinyon.com/tutorials/node/hello-world/)
- ["Hello World" in Python](https://www.infinyon.com/tutorials/python/hello-world/)
- ["Hello World" in Rust](https://www.infinyon.com/tutorials/rust/hello-world/)

---

If you run into any problems along the way, make sure to check out our [troubleshooting]
page to find a fix.

[troubleshooting]: {{< ref "/docs/operations/troubleshoot.md" >}}
