---
title: Partitions
weight: 20
---

**Partitions** are the unit of parallelism accessed independently by **producers** and **consumers** within a **topic**.

Each record stored in a partition is
given an offset, starting from zero and monotonically increasing by one for each new
record. 

Once a record is committed to a partition and an offset is assigned to it, the offset in that partition will _always_ refer to that record. Because of this, all records that are sent to a given partition are
guaranteed to remain ordered in the order they were committed.


**Partitions** are configuration objects managed by the system. Topics and partitions are linked through a **parent-child** relationship. The partition generation algorithm is described in the [SC Architecture].

[SC Architecture]: {{< ref "/docs/architecture/sc#partitions" >}}

<!-- TODO This Image Renders Poorly -->
<img src="/docs/architecture/images/topic-2-partitions.svg"
     alt="Topic 2 Assignment"
     style="justify: center; max-width: 640px" />

If a topic is deleted, all associated partitions are automatically removed.

---

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

- [Check out the multi-consumer example in the CLI reference]({{< ref "cli/commands/consume#consume-from-a-topic-with-multiple-partitions" >}})
