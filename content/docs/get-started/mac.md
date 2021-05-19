---
title: MacOS Installation
menu: MacOS
weight: 10
---

## Install Fluvio CLI

The Fluvio CLI (_command-line interface_) is an all-in-one tool for setting up, managing, and interacting with Fluvio clusters.

Before getting started, install the [Fluvio CLI]. We'll be using the CLI to interact with your cluster.

[Fluvio CLI]: {{< ref "/download/_index.md" >}}

## Required Packages for Local Fluvio cluster

1) [Docker](#install-docker)
2) [Minikube](#install-minikube)
3) [Kubectl](#install-kubectl)
4) [Helm](#install-helm)

If you have `docker`, `kubectl`, `helm`, and `minikube` already set up, then continue to steps for [running a local Fluvio cluster].

[running a local Fluvio cluster]: {{< ref "/docs/get-started/kubernetes.md" >}}

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

Now that we've got all of our tools installed, let's continue on to [running a local Fluvio cluster]!

If you run into any problems along the way, make sure to check out our [troubleshooting]
page to find a fix.

[troubleshooting]: {{< ref "/docs/operations/troubleshoot.md" >}}