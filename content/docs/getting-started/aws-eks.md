---
title: Setup AWS EKS for Fluvio Deployment
menu: Setup on AWS EKS
weight: 20
---

### Set up AWS EKS

Follow the instructions on [Getting Started with Amazon EKS](https://docs.aws.amazon.com/eks/latest/userguide/getting-started.html) to set up new cluster.  

 {{< idea >}} 
Note that Fluvio requires Kubernetes version 1.13.x or greater.
 {{</ idea >}} 

### Check AWS Cluster  version

To check AWS EKS has created the right version of the cluster. 
{{< cli yaml>}}
$ kubectl version
Client Version: version.Info{Major:"1", Minor:"15", GitVersion:"v1.15.3", GitCommit:"2d3c76f9091b6bec110a5e63777c332469e0cba2", GitTreeState:"clean", BuildDate:"2019-08-19T12:36:28Z", GoVersion:"go1.12.9", Compiler:"gc", Platform:"darwin/amd64"}
Server Version: version.Info{Major:"1", Minor:"13+", GitVersion:"v1.13.10-eks-5ac0f1", GitCommit:"5ac0f1d9ab2c254ea2b0ce3534fd72932094c6e1", GitTreeState:"clean", BuildDate:"2019-08-20T22:39:46Z", GoVersion:"go1.11.13", Compiler:"gc", Platform:"linux/amd64"}
{{< /cli>}}

Server version should be “v1.13.x” or higher.

### Install AWS EKS specific storage class

Fluvio needs access to storage to save messages for topic/partitions.  To install an AWS EKS storage driver, run:
{{< cli yaml>}}
$ kubectl apply -f ./k8-util/crd/config/gp2-storageclass-spu.yaml 
{{< /cli>}}



{{< links "Next Steps" >}}
* [Install Fluvio]({{< relref "install-fluvio" >}})
* [Install CLI]({{< relref "install-cli" >}})
{{< /links >}}
