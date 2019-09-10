---
title: Overview
weight: 10
---

Fluvio is a Cloud-native platform that was designed from the ground up to work with [Kubernetes](https://kubernetes.io/). While we expect Fluvio to run on any Kubernetes cluster, the setup instructions cover the following infrastructures:

* [Minikube](https://minikube.sigs.k8s.io/)
* [AWS EKS](https://aws.amazon.com/eks/)

The Fluvio repo provides setup scripts to provision each infrastructure. While it is possible to download the scripts independently, we recommend cloning the full repository.

### Clone Fluvio Repo

Open a terminal and clone the Fluvio repository from [github](https://github.com/infinyon/fluvio) to target machine:

{{< cli yaml>}}
$ git clone https://github.com/infinyon/fluvio.git
{{< /cli>}}  

Navigate to fluvio root directory:

{{< cli yaml>}}
$ cd fluvio
{{< /cli>}}

The setup scripts are located in the __k8_util__ directory:

{{< cli yaml>}}
$ ls
Cargo.lock		future-helper		metadata
Cargo.toml		future-helper-03	rust-toolchain
DEVELOPER.md	k8-client		    rustfmt.toml
LICENSE			k8-config		    sc-server
Makefile		k8-diff			    spu-server
README.md		k8-metadata		    storage
api			    k8-util			    test-helper
cli			    kf-protocol		    types
dev-tools		kf-service		    utils
future-aio		kf-socket
{{< /cli>}}

Choose your infrastructure and navigate to the setup instructions below.

{{< links "Next Steps" >}}
* [Setup on AWS EKS]({{< relref "aws-eks">}})
* [Setup on Minikube]({{< relref "minikube" >}})
* [Install Fluvio]({{< relref "install-fluvio" >}})
* [Install CLI]({{< relref "install-cli" >}})
{{< /links >}}

