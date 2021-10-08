---
title: Consumers
weight: 20
---

Consumers are applications that "consume" records from a particular
topic and partition<sup>[1]</sup>. Typically, a consumer will perform actions based
on the events it receives, such as sending a notification or updating
a database. There may be many consumers reading data from a particular
partition at any given time: since the records are persisted, they do
not expire after being consumed.

-> [1] For an overview of Topics and Partitions, see the [Producer documentation]

[Producer documentation]: {{< ref "/docs/clients/producer" >}}

## Offsets

In order to begin consuming records, a consumer must specify the topic and
partition to consume from, as well as the offset into the partition where
it should begin reading. Remember, the offset of a record is its total
position within its parent partition. However, there are multiple ways that
an offset may be derived in more convenient ways:

- Directly, as an absolute index into the partition, starting from zero
- As a relative distance from the beginning of the partition
- As a relative distance from the end of the partition

When consumers specify a relative offset, the offset given by the consumer
is used to calculate the actual total offset into the partition. There is
also an important difference between an absolute offset and a relative offset
from the beginning of the partition. If a partition has a retention policy
that causes it to begin deleting records from the beginning, then the
relative-from-beginning offset will count forward from the oldest record
that is still available.

## Start Consuming

In order to get started with consuming streaming data, you'll need to:

- [Follow the "Start Producing" steps to produce data to a topic]({{< ref "/docs/clients/producer#start-producing" >}})
- Choose one of the following consumer interfaces to use:
  - [Fluvio CLI]({{< ref "/cli/commands/consume" >}})
  - [Rust]({{< ref "/api/fluvio/rust" >}})
  - [Node]({{< ref "/api/fluvio/node" >}})
  - [Python]({{< ref "/api/fluvio/python" >}})
  - [Java]({{< ref "/api/fluvio/java" >}})
