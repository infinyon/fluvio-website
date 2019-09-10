---
title: Install Fluvio on AWS EKS
menu: Install on EKS
weight: 20
---

### Set up AWS EKS

Follow the instructions on [AWS EKS](https://docs.aws.amazon.com/eks/latest/userguide/getting-started.html) guide to set up new cluster.


### Install AWS EKS specific storage class

Fluvio need AWS EKS storage driver to save message on topic.  To install:
{{< cli yaml>}}
$ ./k8-util/crd/config/gp2-storageclass-spu.yaml 
{{< /cli>}}



{{< links "Next Steps" >}}
* [Install Fluvio]({{< relref "install-fluvio" >}})
* [Install CLI]({{< relref "install-cli" >}})
{{< /links >}}
