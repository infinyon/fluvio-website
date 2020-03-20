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



* Key value store
* K8 paradigm
    * each object has a Spec & Status
    * manage through SC API or directly through K8
* Objects lifecycle: SPUs, SPU-Group, Topic, Partitions.
    * Create, update, delete Patch ... describe workflow
            * receive from KV store... distribute to SPUs
        * Primary purpose of the SC is to push metadata to SPUs
        * Getting status from SPUs and push back to KV store.
* Controllers (implementation detail)
    * Each object has a controller which manages object life cycle
    * Each controller caches data for object types
    * Every increment changes form KV store... it computes the new action... that are propagated through the systems...
        * some actions update the state
        * some actions create or modify other objects
        * actions to distribute data to SPUs
    ```Principles```:
        * Cache is a read-only component and it is only modified by KV store.
    * Each controller is independent from each other.
* Local Cache (Store)

* SPU Connection Manager
    * SC never initiates Communication
    * Duplex Channel
    * Sends Spec to SPU & Receives status from SPU
        * Replica topology
    * Leader/follower election
    * Detects 
        * SPU online/offline

* Replica Distribution
    * link... 

* Reconciliation
    * all components are designed to be recoverable.
    * compares last know state with the current state and reconcile differences.
    * reconciliation is done out of band with no impact to traffic.

* HA for SC
    * one active SC.. can be multiple backups (upcoming)
    * Availability of the SC has no impact on SPU streaming services.
        * Traffic continues... on all existing connections until SC comes online.
    * SPU 

* Configuration startup command
    * private & public port, K8...

* Kubernetes
    * use Kubernetes API to communicate with KV store
    * at startup we read KubeCtl configuration file to detect Kubernetes cluster host.
    * Link to K9 section (namespaces)

* API Servers
    * External
        * API server, default port...
    * Internal