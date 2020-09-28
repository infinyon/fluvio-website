---
title: Getting Started
folder: Getting Started
menu: Overview
toc: true
weight: 10
---

Welcome! Thanks for your interest in Fluvio, the high-performance, low-latency
streaming platform for real-time applications. In this guide, we're going to
walk through the setup process for getting started with Fluvio. There are two
main steps:

- Installing the Fluvio CLI tool
- Getting access to a Fluvio cluster

There are two main ways to get access to a Fluvio cluster. The quickest way is
by creating a [free Fluvio Cloud account], but we'll also cover how to install
your own Fluvio cluster locally if you prefer that type of setup.

## Installing the Fluvio CLI

The Fluvio CLI (or _command-line interface_) is an all-in-one tool for setting
up, managing, and interacting with Fluvio. You can download the CLI from our
[github releases] page. You'll need to open the archive, then move the `fluvio`
executable into a directory in your `PATH`.

```shell script
$ sudo mv fluvio /usr/local/bin/
```

{{< idea >}}

**Note**: Run the `fluvio version` command to make sure it worked, but keep in
mind that you'll probably see different versions than what's shown here.

```shell script
$ fluvio version
Fluvio version : 0.6.0
Git Commit     : 3ff1ee52a31f51c2a3dc7d8fe3996d694fdd585d
OS Details     : Darwin 19.6.0 x86_64
Rustc Version  : 1.46.0 (04488af 2020-08-24)
```

{{< /idea >}}

[github releases]: https://github.com/infinyon/fluvio/releases

# Getting access to a Fluvio cluster

The easiest way to get started with a Fluvio cluster is by creating a
[free Fluvio Cloud account] since we take care of setting up all the moving parts.
If you go the Fluvio Cloud route, you can skip the rest of this page and continue on
to the [Hello Fluvio] example.

[free Fluvio Cloud account]: https://fluvio.io/signup

## Installing a Fluvio cluster

Fluvio is built to run on [Kubernetes], an open-source system that automates the
deployment of containerized apps. If you haven't heard of Kubernetes or containers,
just think of them as tools that help run applications on many computers at once.
To get started with Fluvio locally, we'll need to install a version of Kubernetes 
called Minikube which is meant for testing out Kubernetes apps locally.

If you've worked with Kubernetes before and you already have `kubectl` and
`minikube` set up, feel free to scroll down to [installing fluvio on minikube].

[installing fluvio on minikube]: #installing-fluvio-on-minikube

### Installing Minikube

Head on over to the [Minikube installation page] and follow the instructions to
download and install `minikube`.

[Kubernetes]: https://kubernetes.io/
[Minikube installation page]: https://minikube.sigs.k8s.io/docs/start/

{{< caution >}}

**Note**: If you've never used a container technology like Docker before,
you might run into trouble at the `minikube start` step that looks like this

```shell script
$ minikube start
😄  minikube v1.13.1 on Ubuntu 20.04
👎  Unable to pick a default driver. Here is what was considered, in preference order:
    ▪ docker: Not installed: exec: "docker": executable file not found in $PATH
    ▪ kvm2: Not installed: exec: "virsh": executable file not found in $PATH
    ▪ none: Not installed: exec: "docker": executable file not found in $PATH
    ▪ podman: Not installed: exec: "podman": executable file not found in $PATH
    ▪ virtualbox: Not installed: unable to find VBoxManage in $PATH
    ▪ vmware: Not installed: exec: "docker-machine-driver-vmware": executable file not found in $PATH

❌  Exiting due to DRV_NOT_DETECTED: No possible driver was detected. Try specifying --driver, or see https://minikube.sigs.k8s.io/docs/start/
```

If you run into this, try [installing docker] as described on the [minikube drivers]
page, then try again using `minikube start --driver=docker`

[installing docker]: https://hub.docker.com/search?q=&type=edition&offering=community&sort=updated_at&order=desc
[minikube drivers]: https://minikube.sigs.k8s.io/docs/drivers/docker/

{{< /caution >}}

Once you're done, you should be able to run the following command to
check that minikube is installed correctly.

-> **Note**: The version that you see may be different from the one shown here

```shell script
$ minikube version
minikube version: v1.13.0
commit: eeb05350f8ba6ff3a12791fcce350c131cb2ff44
```

### Installing Kubectl

Minikube is nothing more than just a _mini_ Kubernetes. That means that we need to get
the Kubernetes tool `kubectl` in order to configure and interact with our mini cluster
just like you would with a regular cluster. Head on over to the [installing kubectl]
page to get that installed. Once you're done, you should be able to run the following
command to check that it's installed correctly.

[installing kubectl]: https://kubernetes.io/docs/tasks/tools/install-kubectl/

-> Remember, the versions you see may not match the versions shown here

```shell script
$ kubectl version --short
Client Version: v1.19.1
Server Version: v1.19.0
```

{{<idea>}}

**Note**: Minikube can download the right version of `kubectl` for you, but you have to
access it by running `minikube kubectl -- <args>` rather than the typical `kubectl <args>`.
If you don't want to install another tool feel free to use it this way, but just remember
if we say to run something like `kubectl get pods`, you'll want to run
`minikube kubectl -- get pods`

{{</idea>}}

### Installing Fluvio on Minikube

TODO
