---
title: Streaming Processing Unit (SPU) Architecture
menu: SPU
toc: true
weight: 30
---

**Streaming Processing Unit (SPU)** is responsible for processing data streams in real-time. The SPUs is designed for **horizontal scale**, where SPUs are gradually added to the cluster to accommodate higher data throughput. Each SPU **manages replicas** which represent the lowest unit of a data stream. Replicas are copies of data streams that are evenly distributed across SPUs. 

{{< image src="architecture/spu-architecture.svg" alt="SPU Architecture" justify="center" width="660" type="scaled-90">}}


#### Default Ports

SPUs have a **public** and a **private** server that are attached to the following ports:

* **Public Port**: 9005
* **Private Port**: 9006


## Workflows

The **SPU** is a high performance streaming processing unit that works in unisom with other SPUs to perform the following **core tasks**:

* manage replicas
* manage local storage
* receive data from Producers
* send data to Consumers
* send copies of the data to peer SPUs

The following diagram describes the **SPU architecture**

{{< image src="architecture/spu-workflow.svg" alt="SC Controller" justify="center" width="800" type="scaled-98">}}

1. Leader Controller
    * receives SPU and Partition specs from SC Dispatcher
    * creates local storage for Replicas
    * syncs Replica info with Followers
      * sends Replica info to Follower SPU Multicast writer
      * receives Replica info from Follower SPU Dispatcher
    * sends LRS status to SC Updater
    * receives message from Producers
    * sends messages to Consumers
2. Follower Controller
    * receives Partition specs from SC Dispatcher
    * creates local storage for Replicas
    * syncs Replica info with Leader
      * sends Replica info to Leader SPU Multicast dispatcher
      * receives Replica info from Leader SPU Dispatcher
3. Dispatch SPU/Partition
    * receives SPU and Partition specs from the SC
    * dispatches SPU and Partition specs to Leader Controller
    * dispatches Partition spec to Follower Controllers
    * spawns Leader and Follower Controllers (if needed)
4. Update LRS
    * receives LRS info from Leader Controller and sends to SC    
5. Dispatch Replica Info
    * receives Replica Info from Leader SPU and sends to Follower Controller
    * receives Replica Info from Follower SPU and sends to Leader Controller  
6. Multicast Replica Info
    * receives Replica info from Follower Controller and unicats to Leader SPU
    * receives Replica info from Leader Controller and multicasts to Follower SPUs


## Controllers

SPU Controllers are responsible for core data processing. Unlike SC Controllers, which are single instances provisioned at startup, SPU Controllers are dynamically allocated. The SPU Controllers are optimized for maximum concurrency and can run a large numbers of instances in parallel.

As show in the diagram above, there are two types of Controllers:

* Leader Controller
* Follower Controller

At startup, the **SC dispatcher** manages the Leader and Follower Controllers and forwards the **SPU** and **Partition** Specs.


### Leader Controller

A **Leader Controller** is spawned by the **SC Dispatcher** when a new Leader Replica Spec is received. Each **LC** is responsible for managing the Replica Leader and the storage areas dedicated for the replica. When the Replica is removed, the **LC** is terminated but the storage is preserved. If a Replica with the same id is created again, the storage is reattached. 

#### Inter-SPU Connections 

Similar to the SC-SPU connection setup, **LC** waits for the Follower SPU to initiate a full duplex connection before it can communicate.

#### Producer/Consumers

Each **LC** is solely responsible for the interaction with producers and consumers. When the **LC** receives messages from **Producers**, it performs the following operations:

* appends new records to local storage
* sends updated offsets to sc and follower controllers
* for each consumer, it sends records as stated in their request type:
  * committed records - records that have been replicated across followers
  * uncommitted records - records that have been persisted on local storage

#### Offset Handling

Leader and Followers sync their offsets with each other. If followers fall behind, the leader sends missing records until the followers catch-up. 

When the leaders receives an offset index from the follower, the leader computes the lagging indicator. This indicator is used:

* to detect if records can be committed. 
* to identify the followers that are behind. 

Replica information such as committed records and lagging indicators are sent to the **SC** in the Live Replicas (LRS) message.


### Follower Controller

A **Follower Controller (FC)** managed all Follower Replicas grouped by a **Leader**. The **FC** is spawned by the **SC Dispatcher** when the both conditions are met:

* Follower Spec is new.
* No **FC** has previously been created.

The **FC** is terminated when the last Follower Spec is removed. Each **FC** is responsible for the storage areas dedicated for all follower replicas. The storage is preserved when the **FC** is terminated.

#### Workflow

**FC** event loop engine performs the following operations:

*  Ensures connection to **Leader SPU** is established, otherwise connect.
*  Creates or reattach local storage when new Replica Follower is received.
*  Syncs offsets with **Leader SPU**.
*  Adds records to Replica from **Leader SPU**.


## Election

... goes here


## Replica Storage

Replica is stored in an append only file.  The files are split into smaller files (segment) if size exceed limit.  This make it easy to implement retention policy. By default, files are stored in "/tmp/fluvio/spu-<spu_id>".  

To allow faster access to records in replica, index files are maintained.  Index file is memory mapped to b-tree structure to allow fast access.   Index files are re-built from records as necessary.

## Zero Copy

Records in replica are send to consumer using zero copy mechanism.  This avoid need to copy records in memory.


#### Related Topics
-------------------
* [SC Architecture](../SC)
* [Replication](../replication)
* [Kubernetes Integration](../k8-integration)
* [Deployment Models](../deployments)