---
title: Kubernetes Integration
menu: Kubernetes
toc: true
weight: 60
---

Fluvio uses 

<a href="https://kubernetes.io/docs/concepts/extend-kubernetes/api-extension/custom-resources" target="_blank">custom resources</a> and <a href="https://kubernetes.io/docs/concepts/extend-kubernetes/operator" target="_blank">operators</a> to integrate **managed SPUs** with Kubernetes. Custom operators utilize <a href="https://kubernetes.io/docs/concepts/workloads/controllers/replicaset" target="_blank">replica sets</a> to govern the number of SPUs that Kubernetes should provision and maintain. Managed SPU provisioned through replica sets cannot be manually modified.


* Kubernetes
    * use Kubernetes API to communicate with KV store
    * at startup we read KubeCtl configuration file to detect Kubernetes cluster host.


#### Related Topics
-------------------
* [SC Architecture](../sc)
* [SPU Architecture](../spu)
* [Replication](../replication)
* [Deployment Models](../deployments)