---
title: Topic/Partitions
weight: 70
---

A **topic** is the basic primitive for data stream and the **partitions** is the unit of parallelism accessed independently by **producers** and **consumers**. A topic may be split in any number of partitions.


### Topics

**Topics** define a **data streams**, and **partitions** the number of data slices for each stream. Topics also have a **replication factor** that defines durability, the number of copies for each data slice.

-> **Note**: Replication factor must be less or equal to the number of SPUs.  While topics that exceed the number of available SPUs may be created, they are not provisioned until additional SPUs join the cluster.

Replicas have a leader and one or more followers and distributed across all available SPUs according to the [replica assignment algorithm].

[replica assignment algorithm]: {{< ref "./replica-assignment" >}}

For example, when provisioning a topic with **2** partitions and **3** replicas:

```bash
$ fluvio topic create --topic topic-a --partitions 2 --replication 3
```

**Leaders** maintain the primary data set and **followers** store a copy of the data. Leaders and followers map to independent **SPUs**:

<img src="../images/assignment-leader-followers.svg"
     alt="Leader, Followers"
     style="justify: center; max-width: 640px" />

* topic-a/0
    * **leader** on SPU-1
    * **followers** on SPU-2 and SPU-3
* topic-a/1
    * **leader** on SPU-2
    * **followers** on SPU-1 and SPU-3

### Partitions

**Partition** are configuration objects managed by the system. Topics and partitions are linked through a **parent-child** relationship. Partition generation algorithm is described in the [SC Architecture].

[SC Architecture]: {{< ref "./sc/#partitions" >}}

<img src="../images/topic-2-partitions.svg"
     alt="Topic 2 Assignment"
     style="justify: center; max-width: 640px" />

If a topic is deleted, all child partitions are automatically removed.

#### Producing with Multiple Partitions

When producing records to a Topic that has multiple partitions, there are two cases to
consider when determining the partitioning behavior. These cases are:

- When producing a record that **has a key**, and
- When producing a record that **has no key**

##### Key/value records

When producing records with keys, the producers will use _hash partitioning_,
where the partition number is derived from the hash of the record's key. This
is used to uphold the golden rule of key-based partitioning:

> **Records with the same key are always sent to the same partition**

The current implementation of key hashing uses the **sip-2-4** hashing algorithm,
but that is subject to change in the future.

- [Check out the key partitioning example in the CLI reference]({{< ref "/cli/commands/produce#example-3-produce-keyvalue-records-to-multiple-partitions" >}})

##### Records with no keys

When producing records with no keys, producers will simply assign partition numbers
to incoming records in a round-robin fashion, spreading the load evenly across the
available partitions.

- [Check out the round-robin partitioning example in the CLI reference]({{< ref "/cli/commands/produce#example-4-producing-to-multiple-partitions-using-round-robin" >}})

#### Consuming with Multiple Partitions

Currently, consumers are limited to reading from one partition at a time. This means
that in order to read all records from a given topic, it may be necessary to instantiate
one consumer per partition in the topic.

- [Check out the multi-consumer example in the CLI reference]({{< ref "cli/commands/consume#example-4-consume-from-a-topic-with-multiple-partitions" >}})
