---
title: Replica Election
weight: 10
---

[Replica Assignment](../replica-assignment) _assigns_ SPUs to a replica set and [Replica Election](#replica-election-algorithm) _coordinates_ their roles. The election algorithm manages replica sets in an attempt to designate one active leader at all times. SPUs have a powerful <ins>multi-threaded engine</ins> that can process a large number of leaders and followers at the same time.

If an SPU becomes incapacitated, the election algorithm identifies all impacted replica sets and triggers a re-election. The following section describes the algorithm utilized by each replica set as it elects a new leader. 


## Roles and Responsibilities

The `Leader` and `Followers` of a **Replica Sets** have different responsibilities.

<img src="architecture/election-leader-followers-brief.svg"
     alt="Leader/Follower"
     style="justify: center; max-width: 520px" />

`Leader` responsibilities:
* ingests data from producers
* stores the data in the local store
* sends data to consumers
* forwards incremental data changes to followers
* keeps live replica sets (**LRS**) updated

`Followers` responsibilities:
* establishes connection to the leader (and run periodic health-checks)
* receives data changes from the leader
* stores data in the local store

All followers are in hot-standby and ready to take-over as leader.

## Replica Election Algorithm

Each data stream has a **Live Replica Set (LRS)** that describes the SPUs actively replicating data records in their local data store. **LRS status** can be viewed in `show partitions` CLI command.

<img src="architecture/election-overview.svg"
     alt="Election Overview"
     style="justify: center; max-width: 820px" />

Replica election covers two core cases:
* SPU `goes offline`
* SPU that was previously part of the cluster `comes back online`

### SPU goes Offline

When an SPU goes offline, the SC identifies all impacted `Replica Sets` and triggers an election:

* set `Replica Set` status to _Election_
* choose <ins>leader candidate</ins> from the _followers_ in **LRS** based on smallest lag behind the previous leader:

    * <ins>leader candidate</ins> found:
        * set `Replica Set` status to _CandidateFound_
        * notifies all follower SPUs
        * start _wait-for-response_ timer


    * no eligible <ins>leader candidate</ins> available:
        * set `Replica Set` status to _Offline_        

###### Follower SPUs receive `Leader Candidate` Notification

All SPUs in the Replica Set receive proposed `leader candidate` and perform the following operations:

*  SPU that matches `leader candidate` tries to promote follower replica to leader replica:

    * `follower` to `leader` promotion successful
        * SPU notifies the SC
    
    * `follower` to `leader` promotion failed
        * no notification is sent

* Other SPUs ignore the message


###### SC receives `Promotion Successful` from `Leader Candidate` SPU

The SC perform the follower operations:
* set `Replica Set` status to _Online_
* update **LRS**
* notifies all follower SPUs in the **LRS** list


###### SC 'wait-for-response' timer fired

The SC chooses the next `Leader Candidate` from the **LRS** list and the process repeats.

If no eligible <ins>leader candidate</ins> left:
* set `Replica Set` status to _Offline_


###### SPUs receive new `LRS`

All SPU followers update their *LRS*:
* reconnect to new leader
* synchronize internal checkpoints with new leader (as described below).
* wait for changes from new leader


### SPU comes back Online

When an known SPU comes back Online, the SC identifies all impacted `Replica Sets` and triggers a refresh.
For all Replica Sets with status _Offline_, the SC performs the following operations:

* set `Replica Set` status to _Election_
* choose <ins>leader candidate</ins> from _follower_ membership list based on smallest lag behind the previous leader:

    * <ins>leader candidate</ins> found:
        * set `Replica Set` status to _CandidateFound_
        * notifies all follower SPUs (see above)
        * start _wait-for-response_ timer


    * no eligible <ins>leader candidate</ins> left:
        * set `Replica Set` status to _Offline_  

The algorithm repeats the same steps as in the "SPU goes Offline" section.


## Leader/Follower Synchronization

Each SPU has a _Leader Controller_ that manages leader replicas, and a _Follower Controller_  that manages follower replicas. SPU utilizes Rust **async framework** to run a virtually unlimited number of leader and follower operations simultaneously. 

### Communication Channels

Each **Replica Set** has a communication channel where for the leader and followers exchange replica information. It is the responsibility of the followers to establish a connection to the leader. Once a connection between two SPUs is created, it is shared by all replica sets.

For example, three replica sets **a**, **b**, and **c** that are distributed across `SPU-1`, `SPU-2`, and `SPU-3`: 

<img src="architecture/election-connection.svg"
     alt="Election Connection"
     style="justify: center; max-width: 400px" />

The first follower (**b**, or **c**) from `SPU-1` that tries to communicate with its leader in `SPU-2` generates a TCP connection. Then, all subsequent communication from `SPU-1` to `SPU-2`, irrespective of the replica set, will reuse the same connection.

Hence, each SPU pair will have at most 2 connections. For example:
* `SPU-1` <=> `SPU-2`
    * `SPU-1` followers => `SPU-2` leaders
    * `SPU-2` followers => `SPU-1` leaders

### Synchronization Algorithm

 Replicas use **offsets** to indicate the position of a record in a data stream. Offsets starts at `zero` and are incremented by one anytime a new record is appended.

 **Log End Offset** (LEO) represents the offset of last record in the local store of a replica. A records is considered **committed** only when replicated by all live replicas. **Live Replica Sets (LRS)** is the set of active replicas in the membership list. **High Watermark** (HW) is the last offset of the record committed by the **LRS**. 
 
 **Synchronization algorithm** collects the **LEOs**, computes the **HW**, and manages the **(LRS)**. 

<img src="architecture/election-sync-overview.svg"
     alt="Election Overview"
     style="justify: center; max-width: 480px" />

In this example:
* LRS = 3
* HW = 2
* LEO (Leader = 4, Follower-1 = 3, Follower-2 = 2)

If Follower-2 goes offline: LRS = 2 and HW = 3.

###### Leader/Follower Synchronization

All replica <ins>followers</ins> send their replica status, `LEO` and `HW`, to their <ins>leader</ins>. The leader: 

* uses `LEO` to compute the missing records and send to <ins>follower</ins>
* computes the 'new' `HW` (from min replica `LEOs`).
* sends `HW`, `LEO`, and `LRS` to <ins>all followers</ins>

Replica <ins>followers</ins> receive the data records, `LEO`, and `HW` from the <ins>leader</ins> and perform the following operations:

* append records to local stores
* update local `LEO` and `HW`
* send updated status to <ins>leader</ins>

And the cycle repeats.


###### Lagging Follower

If the <ins>leader</ins> detects any of <ins>follower's</ins> `HW` is less than the **LRS** `HW` by a maximum number of records, the leader removes the follower from the **LRS**. 

Followers removed from the **LRS** are ineligible for election but continue to receive records. If follower catches up with the leader it is added back to **LRS** and once again becomes eligible for election.


###### Leader Failure

If a <ins>leader</ins> goes offline, an [election is triggered](#replica-election-algorithm) and one of the <ins>followers</ins> takes over as <ins>leader</ins>. The rest of the followers connect to the <ins> new leader</ins> and synchronize their data store. 

When the failed leader rejoins the replica set, it detects the new leader and turns itself into a follower. The replica set continue under the new leadership until a new election is triggered.


### Consumer Consistency Model

Replica <ins>leaders</ins> receive data records from producers and sends them to consumers. 

Consumers can choose to receive either COMMITTED or UNCOMMITTED records. The second method is discouraged as it cannot deterministically survive various failure scenarios. 

By default only COMMITTED messages are sent to consumers.