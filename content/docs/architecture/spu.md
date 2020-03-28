---
title: Streaming Processing Unit (SPU) Architecture
menu: SPU
weight: 30
---

**Streaming Processing Unit (SPU)** server is responsible for processing data streams in real-time.  
SPUs were designed for horizontal scale. New SPUs can be gradually added to the cluster to accommodate higher data throughput. Each SPU manages sets of replicas which represent the lowest unit of a data stream. Each replica can a leader or a follower. Replicas are evenly distributed across SPUs for optimal performance. 

{{< image src="spu-architecture.svg" alt="SPU Architecture" justify="center" width="600" type="scaled-98">}}


#### Default Ports

Fluvio SPU has a **public** and a **private** server that are attached to the following ports:

* **Public Port**: 9005
* **Private Port**: 9006


## Object Workflows

**SPU** is designed for distributed data processing and responsible with the following:

* manage replicas
* form peer-to-peer network with other SPUs
* receive and store data from Producers
* send processed data to Consumers

The following diagram describes the **SPU architecture**

{{< image src="spu-workflow.svg" alt="SC Controller" justify="center" width="640" type="scaled-90">}}

1. Spec Dispatcher
    * receives SPU and Partition specs
    * spawns Leader and Follower Controllers
    * dispatches specs to Leader or Follower Controller
2. Leader Controller
    * recevies message from Producers
    * sends messages to Consumers
    * syncs replica information with Followers
    * sends LRS status to SC
3. Follower Controller
    * syncs replica information with Leader
4. Stream Dispatcher
    * dispatches replica information from other SPU leaders to local follower
    * dispatches replica information to other SPU followers to local leader
5. Stream Update
    * sends replica information from local leader to other SPU followers
    * sends replica information from local followers to other SPU leader


## Controllers

SPU has a similar architecture with the SC, where controllers are responsible for core data processing. Each controller can have multiple instances that run in parallel to ensure maximum concurrency.

There are two types of controllers:

* Leader Controller
* Follower Controller

### Leader Controller

For each leader replica, a leader controller is spawned. When the replica is removed the leader controller is terminated. When a controller starts up, it creates a storage area dedicated to the replica in the local SPU.  The controller accepts requests from follower in other SPU. Follower controllers always initiates duplex connections.

Leader controllers are solely responsible for the interaction with producers and consumers. 

When a leader controller receives records from a producer, it performs the following operations:

* appends new records to local storage
* sends updated offsets to sc and follower controllers
* for each consumer, it sends records as stated in their request type:
  * committed records - records that have been replicated across followers
  * uncommitted records - records that have been persisted on local storage

Leader and Followers sync their offsets with each other. If followers fall behind, the leader sends missing records until the followers catch-up. 

When the leaders receives an offset index from the follower, the leader computes the lagging indicator, used to detect if records can be committed. Lagging indicator is also used to identify the followers that are behind. This information is sent to the **SC** in the Live Replicas (LRS) message.


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



