---
title: Election
toc: true
weight: 40
---

## Election Algorithm

SC detects SPU goes offline and it triggers an SPU election.

1. if an SPU goes offline  - for all replicas where SPU was a leader, the SC triggers election
    1. Replica status is set to "Election"
    2. SC looks-up an SPU leader candidate (has to be alive and has smallest lag behind the leader - done by a scoring algorithm)
        * if SPU candidate is found, set Replica state to "CandidateFound"
        * if no eligible SPU is found, set Replica state to "Offline"
    3. SC notifies all SPUs of replica status changes
    4. Each SPU receives replicas eligible for promotion from the SC.
        * SPU tries to promote these follower replicas to leader replicas.
        * If successful, it notifies the SC that Replica promotion succeeded.
        * SC set replica status to "Online" and all SPUs get notified.
    5. SPU followers get notification of the new Replica leader.
        * Follower replicas are now listening for messages from the new leader.

* 2. if an SPU comes online - for all Replicas that in state "Offline", the SC triggers election
    1. SC identifies if SPU can be a leader candidate (has to be alive and has smallest lag behind the leader - done by a scoring algorithm)
        * if SPU candidate is found, set Replica state to "CandidateFound"
    2. SC notifies all SPUs of replica status changes
    3. Each SPU receives replicas eligible for promotion from the SC.
        * SPU tries to promote these follower replicas to leader replicas.
        * If successful, it notifies the SC that Replica promotion succeeded.
        * SC set replica status to "Online" and all SPUs get notified.
    4. SPU followers get notification of the new Replica leader.
        * Follower replicas are now listening for messages from the new leader.


## Leader/Follower Synchronization

Each SPUs maintains a large number of Leader and Follower Replicas in simultaneously.

Each Leader Replica has one Leader Controller and each group of Follower Replicas of the same leader has one Follower Controller. Rust async framework allows Fluvio to scale to virtually unlimited number of Replica Controllers.

### Communication Channels

Only follower initiate communication to the leader. All follower SPUs for a replica set uses a single TCP channel to the leader SPU and form 

* follower always initiates connection to leader
* if there is a communication issue, follower reconnects
* given 2 SPUs there is maximum of 2 possible leader/follower channel (one per direction)
* all replica communication between two SPUs share the same channel.

Each pair of SPUs share a communication channel 

### Synchronization Algorithm

LEO - last offset of the physical record written to disk for the replica
HW - high watermark is the last committed offset for the replica (leader & followers)

* when a follower connects to the leader
    * follower sends the leader its LEO and HW
* the leader receives LEO and HW form all follower replicas
    * it updates its internal HW
    * the leader identifies the smallest LEO from all followers (and meet the min-in-sync-replica), the leader updates it HW (high watermark)
* the leader send followers the new HW (high watermark) 
    * if the followers LEO is less than the HW (when followers fall behind), the leaders sends followers the delta from LEO to the HW to catch-up.
* the follower receives the new records and HW
    * it writes the new records to the disk
    * it updates its own HW
    * sends new LEO to the leader

The entire Replica set will eventually converge.

If the follower goes temporarily offline or in the event of a network failure, the follower will attempt to re-establish connection to the leader and synchronize its data store.

If a new leader is elected, the follower will connect to the new leader and re-synchronize it data store.

Live replicas are the set of replicas that in-sync with the leader. If a follower falls behind the leader with more than x number of messages (identified by a pre-configured gap), the follower will be marked inelligeable for leadership election. Once the follower catches-up below the pre-defined gap, it is moved back to live replica group and becomes elligeble for election.

