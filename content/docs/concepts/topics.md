---
title: Topics
weight: 10
---
**Topics** define a **data stream**. You can think of a topic as the streaming equivalent of a table in a database. Topics can be parallelized into any number of data slices called [partitions].

[partitions]: {{< ref "docs/concepts/partitions">}}

In addition to logically separating records, topics may be individually configured
with parameters to tune the performance and semantics of record delivery.

Topics also have a **replication factor** that defines durability, the number of copies for each data slice.

-> **Note**: Replication factor must be less or equal to the number of SPUs ([Stream Processing Units]).  While topics that exceed the number of available SPUs may be created, they are not provisioned until additional SPUs join the cluster.

[Stream Processing Units]: {{< ref "docs/architecture/spu">}}

Replicas have a leader and one or more followers and distributed across all available SPUs according to the [replica assignment algorithm].

[replica assignment algorithm]: {{< ref "docs/architecture/replica-assignment" >}}

For example, when provisioning a topic with **2** partitions and **3** replicas:

```bash
$ fluvio topic create topic-a --partitions 2 --replication 3
```

**Leaders** maintain the primary data set and **followers** store a copy of the data. Leader and follower replications are assigned to independent **SPUs**:

<img src="/docs/architecture/images/assignment-leader-followers.svg"
     alt="Leader, Followers"
     style="justify: center; max-width: 640px" />

* topic-a/0 is the first partition
    * **leader** on SPU-1
    * **followers** on SPU-2 and SPU-3
* topic-a/1 is the second partition
    * **leader** on SPU-2
    * **followers** on SPU-1 and SPU-3
