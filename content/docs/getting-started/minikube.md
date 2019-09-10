---
title: Install and Setup Minikube for Fluvio Deployment
menu: Setup on Minikube
weight: 30
---

### Install Minikube and Setup Cluster

Follow the instructions on [Minikube Getting Started](https://minikube.sigs.k8s.io/docs/start) guide to install Minikube in your environment and set up local Kubernetes cluster.


### Check Minikube Cluster  version

To check minikube has created the right version of the cluster. 
{{< cli yaml>}}
$ kubectl version
Client Version: version.Info{Major:"1", Minor:"15", GitVersion:"v1.15.3", GitCommit:"2d3c76f9091b6bec110a5e63777c332469e0cba2", GitTreeState:"clean", BuildDate:"2019-08-19T12:36:28Z", GoVersion:"go1.12.9", Compiler:"gc", Platform:"darwin/amd64"}
Server Version: version.Info{Major:"1", Minor:"13", GitVersion:"v1.13.7", GitCommit:"4683545293d792934a7a7e12f2cc47d20b2dd01b", GitTreeState:"clean", BuildDate:"2019-06-06T01:39:30Z", GoVersion:"go1.11.5", Compiler:"gc", Platform:"linux/amd64"}
{{< /cli>}}

Server version should at least "v1.13.x"

### Install Minikube specific storage class

Fluvio need Minikube storage driver to save message on topic.  To install:
{{< cli yaml>}}
$ kubectl apply -f ./k8-util/crd/config/minikube-storageclass-spu.yaml
{{< /cli>}}


### Setup Minikube Tunnel

To expose Minikube ports to your environment, we need to setup a [tunnel](https://minikube.sigs.k8s.io/docs/tasks/loadbalancer/) to your local network.  
Run the following script to setup the tunnel:

{{< cli yaml>}}
$ sudo ./k8-util/minikube-tunnel.sh
{{< /cli>}}


### Help with Hyperkit Driver
Minikube may hang when running macOS/Hyperkit with message “Starting VM…” message displayed at the command line prompt. In that case, you need to remove hyperkit pid and rebuild the cluster.

{{< cli yaml>}}
$ rm ~/.minikube/machines/minikube/hyperkit.pid
$ minikube stop
$ minikube delete
{{< /cli>}} 

{{< links "Next Steps" >}}
* [Install Fluvio]({{< relref "install-fluvio" >}})
* [Install CLI]({{< relref "install-cli" >}})
{{< /links >}}
