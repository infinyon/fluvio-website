---
title: Fluvio for Contributors
menu: Overview
weight: 100
---

Thank you for your interest in joining Fluvio Development Community!  

Follow the following setup instructions to run Fluvio on Kubernetes:

* [Minikube](https://minikube.sigs.k8s.io/)
* [AWS EKS](https://aws.amazon.com/eks/)

The Fluvio repo provides setup scripts to provision each infrastructure.

### Clone Fluvio Repo

Open a terminal and clone the Fluvio repository from [github](https://github.com/infinyon/fluvio):

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

To continue setup, choose one of the following instructions:

* [Setup on AWS EKS]({{< relref "aws-eks">}})
* [Setup on Minikube]({{< relref "minikube" >}})


{{< links "Related Topics" >}}
* [Install Fluvio]({{< relref "install-fluvio" >}})
* [Install CLI]({{< relref "install-cli" >}})
{{< /links >}}

