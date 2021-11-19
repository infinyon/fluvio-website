---
title: MacOS/M1 Installation
menu: MacOS on M1
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

1) [Kind]({{< ref "#install-kind" >}})
2) [Kubectl]({{< ref "#install-kubectl" >}})
3) [Helm]({{< ref "#install-helm" >}})

If you have `kubectl`, `helm`, and `kind` already set up, then continue to steps for [running a local Fluvio cluster].


### Install Kind

Kind is a tool for running a local Kubernetes cluster.

Install `kind` package:

%copy first-line%

```bash
$ brew install kind
```

#### Start a Kubernetes cluster

To configure kind cluster creation, you will need to create a YAML config file. This file follows Kubernetes conventions for versioning etc.
Create a file named: `config.yaml` with the following content:

```yaml
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
  - role: control-plane
    # port forward 80 on the host to 80 on this node
    extraPortMappings:
      - containerPort: 31003 #sc
        hostPort: 31003
        protocol: TCP
      - containerPort: 31004 #spu 1
        hostPort: 31004
        protocol: TCP
      - containerPort: 31005 #spu 2
        hostPort: 31005
        protocol: TCP
      - containerPort: 31006 #spu 3
        hostPort: 31006
        protocol: TCP

```
Start a Kubernetes cluster locally with by running the following in a terminal window:

%copy first-line%
```bash
$ kind create cluster --config k8-util/cluster/kind.yaml 
```

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

You can start a Fluvio cluster by running following command:


%copy first-line%
```bash
$ fluvio cluster start --proxy-addr  127.0.0.1

📝 Running pre-flight checks
     ✅ Kubernetes config is loadable
     ✅ Supported helm version is installed
     ✅ Fluvio system charts are installed
     ✅ Previous fluvio installation not found
🛠️  Installing Fluvio
     ✅ Fluvio app chart has been installed
🔎 Found SC service addr: 127.0.0.1:31003
 -
👤 Profile set
🤖 SPU group launched (1)
     ✅ All SPUs confirmed
🎯 Successfully installed Fluvio!
```

### Verify cluster is running

We can check the fluvio cluster by checking version and status with the following command:

```bash
$ fluvio version

fluvio version
Fluvio CLI           : 0.9.13
Fluvio CLI SHA256    : de53060048aa7a10def4940741e25c8ddefdcdc3ef511562ec915bdb456e1770
Fluvio Platform      : 0.9.13 (kind-kind)
Git Commit           : 88b0d793f3143af3eb0c57bda3bc3b4cf44fd741
OS Details           : Darwin 10.16 (kernel 21.1.0)
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
