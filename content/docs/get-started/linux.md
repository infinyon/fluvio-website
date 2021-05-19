---
title: Linux Installation
menu: Linux
weight: 20
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

Follow the instructions for your Linux distro to install [Docker engine for Linux].

[Docker engine for Linux]: https://docs.docker.com/engine/install/#server 

### Install Minikube

Minikube is a tool for running a local Kubernetes cluster

Follow the instructions at the [Minikube installation page] to download and install `minikube` for your Linux distro.

[Minikube installation page]: https://minikube.sigs.k8s.io/docs/start/

#### Start a Kubernetes cluster
Start a Kubernetes cluster locally with minikube by running the following in a terminal window:

%copy first-line%
```bash
$ minikube start
```

### Install Kubectl

`kubectl` is the Kubernetes command-line tool. It is used to run commands against Kubernetes clusters.

Follow the instructions at the [kubectl installation page] and follow the instructions to download and install `kubectl` on Linux.

[kubectl installation page]: https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/ 

### Install Helm

Helm is the package manager for Kubernetes. 

Follow the instructions at the [helm installation page] and follow the instructions to download and install `helm` on Linux.

[helm installation page]: https://v3.helm.sh/docs/intro/install/ 

Now that we've got all of our tools installed, let's continue on to [running a local Fluvio cluster]!

If you run into any problems along the way, make sure to check out our [troubleshooting]
page to find a fix.

[troubleshooting]: {{< ref "/docs/operations/troubleshoot.md" >}}
