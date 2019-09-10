---
title: Setup AWS EKS for Fluvio Deployment
menu: Setup on AWS EKS
weight: 20
---

### Set up AWS EKS

Follow the instructions on [Getting Started with Amazon EKS](https://docs.aws.amazon.com/eks/latest/userguide/getting-started.html) to set up new cluster.


### Install AWS EKS specific storage class

Fluvio needs access to storage to save messages for topic/partitions.  To install an AWS EKS storage driver, run:
{{< cli yaml>}}
$ kubectl apply -f ./k8-util/crd/config/gp2-storageclass-spu.yaml 
{{< /cli>}}



{{< links "Next Steps" >}}
* [Install Fluvio]({{< relref "install-fluvio" >}})
* [Install CLI]({{< relref "install-cli" >}})
{{< /links >}}
