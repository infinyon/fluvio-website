---
title: Streaming Controller (SC) Architecture
menu: SC
weight: 20
---

**Streaming Controller (SC)** is the central coordinator of a Fluvio cluster. It works in conjunction with a **Key-Value (KV) store** to translate configuration changes into system updates.

#### Key-Value Store

**Fluvio** is designed to work seamlessly with **{{< target-blank title="Kubernetes" url="https://kubernetes.io" >}}** container management system and **{{< target-blank title="etcd" url="https://etcd.io" >}}** key-value store. The **KV interface** is flexible and can be extended to support alternative stores such {{< target-blank title="Consul" url="https://consul.io" >}}, {{< target-blank title="Zookeeper" url="https://zookeeper.apache.org" >}}, or an in-memory database.


## SC Architecture 

Fluvio **SC** is the central coordinator and the **authoritative** entity of the cluster. It intermediates configuration changes with the **KV** store, performs SPU replica assignment, coordinates communication with external clients, and sends periodic reconciliation updates.

{{< image src="sc-architecture.svg" alt="SC Coordinator" justify="center" width="530" type="scaled-90">}}


### System Initialization

At startup, the **SC** connects to the **KV** store and provisions its internal **global store**. SC global store caches all configuration data: SPUs, topic/partition, and replica assignments.


It starts servers...


### Reconciliation

* it can start in any order
* reconciliation
* pushes instructions to the SPU

Augmenting the **SC** dynamically

Restarting the **SC**



