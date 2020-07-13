---
title: Replica Election
toc: true
weight: 40
---

**Replicas** are responsible for the distribution of data streams across SPUs. **Replica Sets** are the SPUs assigned to each data stream. Each replica set is responsible for storing identical replicas of records in their local store. 

[Replica Assignment](../replica-assignment) _assigns_ SPUs to a replica set and [Replica Election](#replica-election-algorithm) _coordinates_ their roles. The election algorithm ensures all replica sets have one leader and one or more followers. SPUs have a powerful <ins>multi-threaded implementation</ins> that can process a large number of leaders and followers at the same time.

If an SPU becomes incapacitated, the election algorithm identifies all impacted replica sets and triggers a re-election. The following section describes the algorithm utilized by each replica set as it elects a new leader. 


## Roles and Responsibilities

The `Leader` and `Followers` of a **Replica Sets** have different responsibilities.

{{< image src="architecture/election-leader-followers-brief.svg" alt="Leader/Follower" justify="center" width="520" type="scaled-90">}}

`Leader` responsibilities:
* listen for connections from followers
* ingest data from producers
* store producer data in the local store
* send data to consumers
* forward incremental data changes to followers

`Followers` responsibilities:
* establish connection to the leader (and run periodic health-checks)
* receive data changes from the leader
* store data in the local store

All followers are in hot-standby and ready to take-over as leader.

## Replica Election Algorithm

Each data stream has a **Live Replica Set (LRS)** that describes the SPUs actively replicating data records in their local data store. **LRS status** can be viewed in `show partitions` CLI command.

{{< image src="architecture/election-overview.svg" alt="Election Overview" justify="center" width="820" type="scaled-98">}}

Replica election covers two core cases:
* SPU `goes offline`
* SPU that was previously part of the cluster `comes back online`

### SPU goes Offline

When an SPU goes offline, the SC identifies all impacted `Replica Sets` and triggers an election:

* set `Replica Set` status to _Election_
* choose <ins>leader candidate</ins> from _follower_ membership list based on smallest lag behind the previous leader:

    * <ins>leader candidate</ins> found:
        * set `Replica Set` status to _CandidateFound_
        * notifies all follower SPUs
        * start _wait-for-response_ timer


    * no eligible <ins>leader candidate</ins> left:
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

All SPU followers update their *LRS*, point to the new leader, and are ready to receive data stream messages.



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

Each SPUs has a _Leader Controller_ that manages leader replicas, and a _Follower Controller_  that manages follower replicas. SPU utilizes Rust **async framework** to run a virtually unlimited number of leader and follower operations in parallel. 

### Communication Channels

SPUs are multiplexing the TCP channel to communicate with each other. When an SPU comes online, it initializes the leader and follower Controllers. Each follower in the SPU **Replica Sets** generates a bi-directional TCP connection to the leader SPU. Once a connection is established, all subsequent SPU-pair exchanges will utilize the same channel.

Given a pair of SPUs there is maximum of two possible connections (<ins>one per direction</ins>):
* SPU-1 follower -> SPU2 leader
* SPU-2 follower -> SPU-1 leader

 If the connection is lost, the follower re-initiates connection to the leaders. Leaders must wait for the connection to be established to communicate with followers.

### Synchronization Algorithm

Synchronization algorithm uses the following variables to track data synchronization in a **Replica Set**:

* **LEO** - Log End Offset - offset of the last record written on the physical disk by <ins>a replica</ins>.
* **HW** - High Watermark - offset of the last record committed by <ins>all replicas</ins> (leader & followers).

###### Leader/Follower Synchronization

Replica <ins>leaders</ins> are responsible for receiving data records from producers and feed them to consumers. In addition, leaders are responsible for the data synchronization with the followers. Replica <ins>followers</ins> receive data records from the <ins>leader</ins> and reply with their last known `LEO` and `HW`.

A replica <ins>leader</ins> receives the `LEO` and `HW` form all follower replicas, and performs the following operations:

* computes the smallest `LEO` received and updates its internal `HW`
* send followers the new `HW`
    
###### Lagging Followers

If a follower `LEO` is less than the `HW`, the follower has fallen behind. The leaders helps the follower catch-up by sending all missing records from `LEO` offset to the `HW` offset. If the follower falls behind for a prolonged period of time or it exceeds a maximum number of records, the leader may decide to temporarily remove the follower from the **LRS** set. Followers removed from **LRS** list are ineligible for election but they continue receiving records. If the follower catches-up with the leader it is moved back to **LRS** set and it becomes eligible for election.

The election algorithm is designed to recover from failure cases and bring **Replica Set** back in sync.

###### Follower Failure

If the follower goes temporarily offline or in the event of a network failure, the follower will attempt to re-establish connection to the leader and synchronize its data store. If a new leader is elected, the follower will connect to the new leader and re-synchronize it data store.


#### Related Topics
-------------------
* [SC Architecture](../sc)
* [SPU Architecture](../spu)
* [Replica Assignment](replica-assignment)
* [Client Library](../client)
* [References](../references)