---
title: Producers
weight: 10
---

Producers are applications that "produce" streaming data.
These applications may be monitoring the status of a system, collecting sensor
data, watching traffic to a website, or otherwise observing events in some way.
Producers may be general-purpose tools such as [the Fluvio CLI], or they may be
special-purpose programs built to meet a specific need, in which case the
producer program would leverage [one of the programmatic APIs]. Let's talk about
the core concepts that producers deal with in order to better understand how
they work.

[the Fluvio CLI]: {{< ref "/cli" >}}
[one of the programmatic APIs]: {{< ref "/api" >}}

## Records, Topics, and Partitions

In order to understand producers, we need to have a good idea about the data they
interact with (records), the means by which they logically organize that data (topics),
and the strategies they use in order to efficiently distribute and process that data
(partitions). These are general ideas seen in many streaming systems, but we're going
to review them in case you're not familiar with them.

### Records

A **Record** is simply a piece of data that is indexed and stored for later use.
In streaming applications, a record typically communicates the fact that a particular
event has occurred, such as a measurement being taken by a sensor, or a button click in
a mobile app. Any given record belongs to exactly one topic and one partition, and
when it is stored it is assigned an offset, which is the absolute position of the
record within it's parent partition.

In addition to carrying arbitrary data, records may optionally be created with a key.
Record keys are used in order to determine which partition within a topic the record
should be sent to. The golden rule is: any two records with the same key are always
sent to the same partition. If a record does not have a key, it is assigned to a
partition based on some configured strategy, such as round-robin.

### Topics

As we have alluded to, **Topics** are a tool for organizing records in a streaming system.
You can think of a topic as the streaming equivalent of a table in a database. These
are typically created by the cluster administrator rather than an application.
Each time you produce a record, you must specify a topic for it to be sent to.
Topics may be used to keep different types of records separate from one another, and
be organized in a way that aligns nicely with your application's domain model.

In addition to logically separating records, topics may be individually configured
with parameters to tune the performance and semantics of record delivery. Two
notable parameters are the number of partitions the topic has as well as the
replication factor, which describes the minimum number of copies of the data in the
topic that must be maintained at all times.

### Partitions

**Partitions** are a mechanism by which the load of traffic to and from a given topic may
be distributed and balanced between many machines. A single partition is a concrete,
ordered log that is stored to disk. As a log, each record stored in a partition is
given an offset, starting from zero and monotonically increasing by one for each new
record. Once a record is committed to a partition and an offset is assigned to it,
that offset (in that partition) will _always_ refer to that record - it is a permanent
assignment. Because of this, all records that are sent to a given partition are
guaranteed to remain ordered in the order they were committed.

There are a few important observations to note about the behavior of partitions based
on the properties we just covered:

- When a record is assigned an offset, that offset permanently identifies
  that record, but this does not necessarily mean that the record will always be available.
  Topics generally define a _retention policy_, which describes how long records will be retained.
  If a record lives beyond its retention policy, it may be deleted to make space for other data.
  However, the offset that identified that record will never be reused for another record.
- It is important to grasp that strong record ordering guarantees only apply for records within
  a single partition. If records are sent to two different partitions in the same topic, there
  is no way to establish ordering between the records living in the distinct partitions. For
  this reason, it is important to select a key for your records based on your ordering needs.
  Since records with the same key are always assigned to the same partition, any records that
  share a key will always be totally ordered with respect to each other.

## Batching
Fluvio producer tries to send records in batches to reduce the number of messages sent and improve throughput. A producer has some configurations that can be set to improve performance for a specific use case. For instance, they can be used to reduce disk usage, reduce latency, improve throughput, among other reasons.
As of today, batching behavior in Fluvio Producer can be modified with the following configurations:

- `batch_size`: Indicates the maximum amount of bytes that can be accumulated in a batch.
- `linger`: Time to wait before sending messages to the server. Defaults to 100 ms.
- `compression`: Compression algorithm used by the producer to compress each batch before sending it to the SPU. Supported compression algorithms are none, gzip, snappy and lz4.
  
In general, each one of these configurations has its drawbacks. For instance, with the compression algorithm, it is a trade-off between disk usage in the server and CPU usage in the producer and the consumer for compression and decompression. Typically, the compression ratio is improved when the payload is large, therefore a larger `batch_size` could be used to improve the compression ratio. A `linger` equals `0` means that each record is sent as soon as possible. A `linger` time larger than zero introduces latency but improves throughput.

The ideal parameters for the `batch_size`, `linger` and `compression` depends on your application needs.

## Start Producing

In order to get started with producing streaming data, you'll need to:

- [Have a Fluvio cluster up and running]({{< ref "/docs/get-started" >}}),
- [Create a Topic to produce data to]({{< ref "/cli/commands/topic#fluvio-topic-create" >}}), then
- Choose one of the following producer interfaces to use:
  - [Fluvio CLI]({{< ref "/cli/commands/produce" >}})
  - [Rust]({{< ref "/api/official/rust" >}})
  - [Node]({{< ref "/api/official/node" >}})
  - [Python]({{< ref "/api/official/python" >}})
  - [Java]({{< ref "/api/official/java" >}})

