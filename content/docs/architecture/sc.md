---
title: Streaming Controller (SC) Architecture
menu: SC
weight: 20
---

**Streaming Controller (SC)** is the central coordinator and the **authoritative** entity of the cluster. It intermediates configuration changes with the **Key-Value (KV)** store, performs SPU replica assignment, coordinates communication with external clients, and sends periodic reconciliation updates.

{{< image src="sc-architecture.svg" alt="SC Coordinator" justify="center" width="530" type="scaled-90">}}







The SC leverages a **Key-Value (KV) store** to persist cluster object configurations.

### Key-Value Store

**Fluvio** is designed to work seamlessly with **{{< target-blank title="Kubernetes" url="https://kubernetes.io" >}}** and **{{< target-blank title="etcd" url="https://etcd.io" >}}** **KV** store. The **KV interface** is store agnostic and can be extended to support alternative implementations such {{< target-blank title="Consul" url="https://consul.io" >}}, {{< target-blank title="Zookeeper" url="https://zookeeper.apache.org" >}}, or in-memory stores.



### System Initialization

At startup, the **SC** connects to the **KV** store and provisions its internal **global store**. SC global store caches all configuration data: SPUs, topic/partition, and replica assignments.


It starts servers...


### Reconciliation

* it can start in any order
* reconciliation
* pushes instructions to the SPU

Augmenting the **SC** dynamically

Restarting the **SC**



