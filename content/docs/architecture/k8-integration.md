---
title: Kubernetes Integration
weight: 110
---

Fluvio uses {{< target-blank title="custom resources" url="https://kubernetes.io/docs/concepts/extend-kubernetes/api-extension/custom-resources" >}} and {{< target-blank title="operators" url="https://kubernetes.io/docs/concepts/extend-kubernetes/operator" >}} to integrate **managed SPUs** with Kubernetes. Custom operators utilize {{< target-blank title="replica sets" url="https://kubernetes.io/docs/concepts/workloads/controllers/replicaset" >}} to govern the number of SPUs that Kubernetes should provision and maintain. Managed SPU provisioned through replica sets cannot be manually modified. For additional information, checkout [Kubernetes Integration]({{< relref "k8-integration" >}}) section.

* Kubernetes
    * use Kubernetes API to communicate with KV store
    * at startup we read KubeCtl configuration file to detect Kubernetes cluster host.