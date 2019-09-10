---
title: Install and Setup Minikube for Fluvio Deployment
menu: Setup on Minikube
weight: 30
---

### Install Minikube

Follow the instructions on [Minikube Getting Started](https://minikube.sigs.k8s.io/docs/start) guide to install Minikube in your environment.

### Create Minikube Cluster

Fluvio currently support Kubernetes version 1.13.x.  
Run the following shell script to create a Minikube cluster with the right Kubernetes version number:

{{< cli yaml>}}
$ ./k8-util/minikube-start-mac.sh
{{< /cli>}}

### Install Minikube specific storage class

Fluvio need Minikube storage driver to save message on topic.  To install:
{{< cli yaml>}}
$ kubectl apply -f ./k8-util/crd/config/minikube-storageclass-spu.yaml
{{< /cli>}}


### Setup Minikube Tunnel

To expose Minikube ports to your environment, we need to setup a [tunnel](https://minikube.sigs.k8s.io/docs/tasks/loadbalancer/) to your local network.  
Run the following script to setup the tunnel:

{{< cli yaml>}}
$ ./k8-util/minikube-tunnel.sh
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
