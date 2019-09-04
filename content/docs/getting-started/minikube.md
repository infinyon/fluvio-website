---
title: Install and Setup Minikube
menu: Setup Minikube
weight: 20
---

### Installing Minikube

Follow [Minikube doc](https://minikube.sigs.k8s.io/docs/start) to install minikube for your environment.

### Creating Minikube cluster

Fluvio currently support Kubernetes version 1.13.x.

To create minikube cluster with the right version:

{{< cli yaml>}}
$ ./k8-util/minikube-start-mac.sh
{{< /cli>}}

### Setting up Minikube tunnel

To expose minikube ports to your environment, we need to set up [tunnel](https://minikube.sigs.k8s.io/docs/tasks/loadbalancer/) to local network.

{{< cli yaml>}}
$ ./k8-util/minikube-tunnel.sh
{{< /cli>}}


### Help for Hyperkit driver
Minikube may hang when running macOS/Hyperkit with message “Starting VM…”.  
In that case, you need to rebuild the cluster after removing hyperkit pid.

{{< cli yaml>}}
rm ~/.minikube/machines/minikube/hyperkit.pid
minikube stop
minikube delete
{{< /cli>}} 

{{< links "Next Steps" >}}
* [Install Fluvio]({{< relref "install-fluvio" >}})
* [Install CLI]({{< relref "install-cli" >}})
{{< /links >}}
