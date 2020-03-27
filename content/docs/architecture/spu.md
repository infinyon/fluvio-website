---
title: Streaming Processing Unit (SPU) Design
menu: SPU
weight: 30
---

**Streaming Processing Unit (SPU)** is server responsible for processing streaming data in real-time.  
The SPU can be scaled horizontally in order to handle more data or increase throughput.  Each SPU manages sets of **replicas** which represents lowest unit of data stream.  The replica can be either leader or follower of the partition.   SC balances replica distribution such that each SPU can handle leaders and follower replicas evenly. 


{{< image src="spu-architecture.svg" alt="SPU Architecture" justify="center" width="600" type="scaled-98">}}

SPU receives SPU/Partition spec from SC and send out partition status back.

#### Default Ports

Fluvio SPU has a **public** and a **private** server that are attached to the following ports:

* **Public Port**: 9005
* **Private Port**: 9006


## Controllers

Like SC, core processing are done by controllers.  There are two controllers:

* Leader Controller
* Follower Controller

### Leader Controller

For each leader replica, a leader controller is spawned.  If leader replica is removed, leader controller is terminated.  When a controller starts up, it set up storage for replica in the local storage of the SPU.  The controller accepts requests from follower controller in other SPU.  Follower controller always initiates duplex connection

Only leader controller interacts with producer or consumer. 

When leader controller receives records from producer
* appends new records to storage
* send updated offsets to sc, follower controllers
* for each consumer, send relevant records back depends on their request type
  * commit type (send any new records or committed records)
  * starting offset

When follower controller is connected to leader, offsets are sync with each other.  When leader detects follower is behind, it sends delta records to follower until follower is fully catch up.  Sync is trigger whenever any changes to leader's offset.

For each offsets receives from follower, leader computes high watermark which result in updated committed offset if enough followers have caught up with leader.  if high watermark is updated, it is also send to followers, consumers and SC as LRS.


### Follower Controller

Follower replica is grouped by their leader.  For each leader, follower controller is spawned.  When controller is starts, it sets up storage for all follower replica it contains. 

Follower controller then goes into reconciliation loop:
*  Ensure connection to leader is established. if it is not, it initiates connection
*  Creates/Update follower replica when receives updates from SC
*  Sync offsets with leader
*  Adds records to follower replica if receives from leader


## Replica Storage

Replica is stored in an append only file.  The files are split into smaller files (segment) if size exceed limit.  This make it easy to implement retention policy. By default, files are stored in "/tmp/fluvio/spu-<spu_id>".  

To allow faster access to records in replica, index files are maintained.  Index file is memory mapped to b-tree structure to allow fast access.   Index files are re-built from records as necessary.

## Zero Copy

Records in replica are send to consumer using zero copy mechanism.  This avoid need to copy records in memory.




