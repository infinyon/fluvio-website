---
title: Setup Minikube
weight: 20
---

### Installing Minikube

Follow [Minikube setup](https://minikube.sigs.k8s.io/docs/start) to set up minikube for your environment.

### Creating Minikube cluster

Fluvio current support Kubernetes version 1.13.x.

This will create minikube cluster with right version:

```
./k8-util/minikube-start-mac.sh
```

### Setting up Minikube tunnel

In order to expose minikube ports to your environment, we need to set up [tunnel](https://minikube.sigs.k8s.io/docs/tasks/loadbalancer/)

```
./k8-util/minikube-tunnel.sh
```


### Help for Hyperkit driver
Minikube may hang when running macOS/Hyperkit with message "Starting VM...", you need to rebuild the cluster

First delete hyperkit id
```
rm ~/.minikube/machines/minikube/hyperkit.pid
minikube stop
minikube delete
``` 



#### Next Steps
* [Install Fluvio]({{< relref "install-fluvio" >}})
* [Install CLI]({{< relref "install-cli" >}})
