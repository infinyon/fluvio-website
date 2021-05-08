---
title: Topic/Partitions
weight: 30
---

A **topic** is the basic primitive for data stream and the **partitions** is the unit of parallelism accessed independently by **producers** and **consumers**. A topic may be split in any number of partitions.


### Topics

**Topics** define a **data streams**, and **partitions** the number of data slices for each stream. Topics also have a **replication factor** that defines durability, the number of copies for each data slice.

-> **Note**: Replication factor must be less or equal to the number of SPUs.  While topics that exceed the number of available SPUs may be created, they are not provisioned until additional SPUs join the cluster.

Replicas have a leader and one or more followers and distributed across all available SPUs according to the [replica assignment algorithm](../replica-assignment).

For example, when provisioning a topic with **2** partitions and **3** replicas:

```bash
$ fluvio topic create --topic topic-a --partitions 2 --replication 3
```

**Leaders** maintain the primary data set and **followers** store a copy of the data. Leaders and followers map to independent **SPUs**:

<img src="architecture/assignment-leader-followers.svg"
     alt="Leader, Followers"
     style="justify: center; max-width: 640px" />

* topic-a/0
    * **leader** on SPU-1
    * **followers** on SPU-2 and SPU-3
* topic-a/1
    * **leader** on SPU-2
    * **followers** on SPU-1 and SPU-3

### Partitions

**Partition** are configuration objects managed by the system. Topics and partitions are linked through a **parent-child** relationship. Partition generation algorithm is described in the [SC Architecture](../sc/#partitions).

<img src="architecture/topic-2-partitions.svg"
     alt="Topic 2 Assignment"
     style="justify: center; max-width: 680px" />

If a topic is deleted, all child partitions are automatically removed.

#### Related Topics
-------------------
* [SC Architecture](../sc)
* [SPU Architecture](../spu)
* [Replica Election](../replica-election)
* [Client Library](../client)