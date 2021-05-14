---
title: Before starting server 
menu: Before starting server
weight: 20
---

## Prerequisites

Before getting started, make sure you have the [Fluvio CLI] installed, as
we'll be using that to interact with your cluster and make sure everything
is working as expected.

[Fluvio CLI]: {{< ref "/docs/get-started/cli.md" >}}

## Required Packages

1) [Install Docker](#installing-docker)
2) [Install Minikube](#installing-minikube)
3) [Install Kubectl](#installing-kubectl)
4) [Install Helm](#installing-helm)

If you already have `docker`, `kubectl`, `helm`, and `minikube` set up, continue onto [Start Server].

[Start Server]: {{< ref "/docs/get-started/start-server.md" >}}

### Installing Docker

Docker is a container engine that Minikube can use to run Kubernetes apps. Go to
the [Docker downloads page] and choose the edition built for your OS. Follow all
of the installation steps, then verify that you're able to run `docker` without
using sudo:

```bash
$ docker run hello-world
```

~> **Note**: On Linux, make sure you run the [post-install steps] or this won't work without sudo

[Docker downloads page]: https://hub.docker.com/search?q=&type=edition&offering=community&sort=updated_at&order=desc
[post-install steps]: https://docs.docker.com/engine/install/linux-postinstall/

### Installing Minikube

Head on over to the [Minikube installation page] and follow the instructions to
download and install `minikube`. Make sure it is installed correctly by checking
the version:

-> **Note**: Make sure that the version you see is greater than or equal to the one below

```bash
$ minikube version
minikube version: v1.17.1
commit: 043bdca07e54ab6e4fc0457e3064048f34133d7e
```

To start your minikube cluster, run the following. This will vary depending on what
OS you use, so make sure you are following the instructions for your platform.

[Minikube installation page]: https://minikube.sigs.k8s.io/docs/start/

{{< tabs tabTotal="2" tabID="1" tabName1="Mac" tabName2="Linux">}}

{{< tab tabNum="1" >}}

In your Mac terminal:

```bash
minikube start --driver=hyperkit --kubernetes-version=1.19.6
```

-> On Mac, `hyperkit` is provided by Docker Desktop, so you still need to install docker

{{< /tab >}}

{{< tab tabNum="2" >}}

In your Linux terminal:

```bash
minikube start --driver=docker --kubernetes-version=1.19.6
```

{{< /tab >}}

{{< /tabs >}}


{{< caution >}}

**Note**: If you see an error with `❌  Exiting due to DRV_NOT_DETECTED`, [make sure you installed docker correctly]

[make sure you installed docker correctly]: {{< ref "/docs/operations/troubleshoot.md" >}}

{{< /caution >}}

### Installing Kubectl

Minikube is nothing more than just a _mini_ Kubernetes. That means that we need to get
the Kubernetes tool `kubectl` in order to configure and interact with our mini cluster
just like you would with a regular cluster. Head on over to the [installing kubectl]
page to get that installed. Once you're done, you should be able to run the following
command to check that it's installed correctly.

[installing kubectl]: https://kubernetes.io/docs/tasks/tools/install-kubectl/

-> Remember, the versions you see may not match the versions shown here

```bash
$ kubectl version --short
Client Version: v1.20.1
Server Version: v1.19.6
```

{{<idea>}}

**Note**: Minikube can download the right version of `kubectl` for you, but you have to
access it by running `minikube kubectl -- <args>` rather than the typical `kubectl <args>`.
If you don't want to install another tool feel free to use it this way, but just remember
if we say to run something like `kubectl get pods`, you'll want to run
`minikube kubectl -- get pods`

{{</idea>}}

### Installing Helm

Helm is a package manager that makes it easy to install apps for Kubernetes. Fluvio is
packaged as a "Helm chart" (the name for a package in Helm), and the Fluvio CLI can
install that chart for you by calling the `helm` executable with all the right options.
In order to do this, you need to have `helm` installed.

You can install `helm` from the [helm releases page]. Once it's installed, make sure it
works by printing the version.

-> Once again, the version you see might not be the same!

```bash
$ helm version --short
v3.3.4+ga61ce56
```

[helm releases page]: https://github.com/helm/helm/releases

