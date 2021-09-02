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

1) [Docker]({{< ref "#install-docker" >}})
2) [Minikube]({{< ref "#install-minikube" >}})
3) [Kubectl]({{< ref "#install-kubectl" >}})
4) [Helm]({{< ref "#install-helm" >}})

If you have `docker`, `kubectl`, `helm`, and `minikube` already set up, then continue to steps for [running a local Fluvio cluster].

[running a local Fluvio cluster]: {{< ref "/docs/get-started/mac.md#start-fluvio-cluster" >}}

### Install Docker

Docker is a container engine which is used by Minikube to run a local Kubernetes cluster.

Download and install [Docker Desktop for Mac]. Select the version that matches your chipset (Intel or Apple).

[Docker Desktop for Mac]: https://hub.docker.com/editions/community/docker-ce-desktop-mac 

### Install Minikube

Minikube is a tool for running a local Kubernetes cluster

Install `minikube` with the [Brew Package Manager] by running the following in a terminal window:

%copy first-line%

```bash
$ brew install minikube
```

Or follow the instructions at the [Minikube installation page] to download and install `minikube` on MacOS.

[Brew Package Manager]: https://brew.sh/
[Minikube installation page]: https://minikube.sigs.k8s.io/docs/start/

#### Start a Kubernetes cluster
Start a Kubernetes cluster locally with minikube by running the following in a terminal window:

%copy first-line%
```bash
$ minikube start
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

You can start a Fluvio cluster by running `fluvio cluster start`.

If this is your first time starting a cluster in your session, be prepared to enter your password.

%copy first-line%
```bash
$ fluvio cluster start --local
```

### Verify cluster is running

We can check the status of the SPUs in the cluster with the following command:

```bash
$ fluvio cluster spu list
 ID    NAME             STATUS  TYPE      RACK  PUBLIC          PRIVATE
 5001  custom-spu-5001  Online  "custom"   -    localhost:9010  localhost:9011
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
