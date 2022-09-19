---
title: Linux Installation
menu: Linux
weight: 20
---

## Install Fluvio CLI

The Fluvio CLI (_command-line interface_) is an all-in-one tool for setting up, interacting, and managing with Fluvio clusters.

Install the Fluvio CLI by running the following command:

%copy first-line%
```bash
curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

## Setting up a Fluvio cluster on Kubernetes

With CLI, you can create and manage fluvio clusters on Kubernetes.  Before you can create a Fluvio cluster, you need to have a Kubernetes cluster up and running.

## Installing Kubernetes cluster

Either you can create a cluster on your local machine or you can deploy it to a cloud provider.

For installing on your local machine, here are suggested Kubernetes installation options:

1) [K3d](https://k3d.io)
2) [Kind](https://kind.sigs.k8s.io)
3) [Minikube](https://minikube.sigs.k8s.io/docs/start/)

After installing Kubernetes, you can run the following command to check if your Kubernetes cluster is up and running:

%copy first-line%
```bash
$ kubectl config current-context
minikube
```

Some of Kubernetes installation will install `kubectl` and `helm`.  You can check it by:

%copy first-line%
```bash
$ kubectl version
Client Version: version.Info{Major:"1", Minor:"21", GitVersion:"v1.21.3", GitCommit:"ca643a4d1f7bfe34773c74f79527be4afd95bf39", GitTreeState:"clean", BuildDate:"2021-07-15T21:04:39Z", GoVersion:"go1.16.6", Compiler:"gc", Platform:"linux/amd64"}
Server Version: version.Info{Major:"1", Minor:"21", GitVersion:"v1.21.2", GitCommit:"092fbfbf53427de67cac1e9fa54aaa09a28371d7", GitTreeState:"clean", BuildDate:"2021-06-16T12:53:14Z", GoVersion:"go1.16.5", Compiler:"gc", Platform:"linux/amd64"}
```

%copy first-line%
```bash
$ helm version
version.BuildInfo{Version:"v3.6.2", GitCommit:"ee407bdf364942bcb8e8c665f82e15aa28009b71", GitTreeState:"clean", GoVersion:"go1.16.5"}
```

If you didn't install `kubectl` and `helm`, you can install them in the following way:

### Install Kubectl

`kubectl` is the Kubernetes command-line tool. It is used to run commands against Kubernetes clusters.

Follow the instructions at the [kubectl installation page] and follow the instructions to download and install `kubectl` on Linux.

[kubectl installation page]: https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/

### Install Helm

Helm is the package manager for Kubernetes.

Follow the instructions at the [helm installation page] and follow the instructions to download and install `helm` on Linux.

[helm installation page]: https://v3.helm.sh/docs/intro/install/

## Start Fluvio cluster on Kubernetes

You can start a Fluvio cluster by running `fluvio cluster start`.

%copy first-line%
```bash
$ fluvio cluster start
📝 Running pre-flight checks
     ✅ Kubernetes config is loadable
     ✅ Supported helm version is installed
     ✅ Fixed: Missing Fluvio system charts.
     ✅ Previous fluvio installation not found
🛠️  Installing Fluvio
     ✅ Fluvio app chart has been installed
🔎 Found SC service addr: 172.19.0.2:30814
👤 Profile set
🤖 SPU group launched (1)
     ✅ All SPUs confirmed
🎯 Successfully installed Fluvio!
```

### Verify cluster is running

You can start a Fluvio cluster by running:

%copy first-line%
```bash
$ fluvio version
Fluvio CLI           : 0.9.0
Fluvio CLI SHA256    : 170c6d4bad98e961b1f14d0fd052900dcbc92d736757bad3c7dcae2095151861
Fluvio Platform      : 0.9.0 (minikube)
Git Commit           : 5ff06169660c2f111bde3bdfcab9b83f569f9960
OS Details           : Ubuntu 18.04 (kernel 5.4.0-1054-aws)
=== Plugin Versions ===
Fluvio Cloud CLI (fluvio-cloud) : 0.1.5
Fluvio Runner (fluvio-run)     : 0.2.1
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

[topic]: {{< ref "/cli/commands/topic.md" >}}
[produce]: {{< ref "/cli/commands/produce.md" >}}
[consume]: {{< ref "/cli/commands/consume.md" >}}

---

If you run into any problems along the way, make sure to check out our [troubleshooting]
page to find a fix.

[troubleshooting]: {{< ref "/docs/operations/troubleshoot.md" >}}
