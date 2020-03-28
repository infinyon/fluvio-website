---
title: Streaming Processing Unit (SPU) Architecture
menu: SPU
weight: 30
---

**Streaming Processing Unit (SPU)** is responsible for processing data streams in real-time. The SPUs is designed for **horizontal scale**, where SPUs are gradually added to the cluster to accommodate higher data throughput. Each SPU **manages replicas** which represent the lowest unit of a data stream. Replicas are copies of data streams that are evenly distributed across SPUs. 

{{< image src="spu-architecture.svg" alt="SPU Architecture" justify="center" width="660" type="scaled-90">}}


#### Default Ports

SPUs have a **public** and a **private** server that are attached to the following ports:

* **Public Port**: 9005
* **Private Port**: 9006


## Object Workflows

The **SPU** is a high performance streaming processing unit that works in unisom with other SPUs to perform the following **core tasks**:

* manage replicas
* manage local storage
* receive data from Producers
* send data to Consumers
* send copies of the data to peer SPUs

The following diagram describes the **SPU architecture**

{{< image src="spu-workflow.svg" alt="SC Controller" justify="center" width="800" type="scaled-98">}}

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

### Leader Controller

At startup, a **Core Leader Controller** is created. This controller is responsible for maintenance tasks such as, creating a dedicated storage areas for replicas in the local SPU.

A **Replica Leader Controller (RLC)** is spawned anytime a new Replica is created. Each **RLC** is responsible for managing the Replica Leader This **RLC** is terminated when the Replica is removed. 

#### Inter-SPU Connections 

Similar to the SC-SPU connection setup, **RLC** waits for the Follower SPU to initiate a full duplex connection before it can communicate.

#### Producer/Consumers

Each **RLC** is solely responsible for the interaction with producers and consumers. When the **RLC** receives messages from **Producers**, it performs the following operations:

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


## continue here -------


### Follower Controller

Follower replica is grouped by their leader.  For each leader, follower controller is spawned.  When controller is starts, it sets up storage for all follower replica it contains. 

Follower controller then goes into reconciliation loop:
*  Ensure connection to leader is established. if it is not, it initiates connection
*  Creates/Update follower replica when receives updates from SC
*  Sync offsets with leader
*  Adds records to follower replica if receives from leader

## Election

... goes here

## Replica Storage

Replica is stored in an append only file.  The files are split into smaller files (segment) if size exceed limit.  This make it easy to implement retention policy. By default, files are stored in "/tmp/fluvio/spu-<spu_id>".  

To allow faster access to records in replica, index files are maintained.  Index file is memory mapped to b-tree structure to allow fast access.   Index files are re-built from records as necessary.

## Zero Copy

Records in replica are send to consumer using zero copy mechanism.  This avoid need to copy records in memory.



