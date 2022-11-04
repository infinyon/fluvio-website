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
<!-- TODO this links to something that doesn't talk about topics and partitions -->
[Producer documentation]: {{< ref "/docs/clients/producer" >}}

## Start Consuming

In order to get started with consuming streaming data, you'll need to:

- [Follow the "Start Producing" steps to produce data to a topic]({{< ref "/docs/clients/producer#start-producing" >}})
- Choose one of the following consumer interfaces to use:
  - [Fluvio CLI]({{< ref "/cli/commands/consume" >}})
  - [Rust]({{< ref "/api/official/rust/installation" >}})
  - [Node]({{< ref "/api/official/node/installation" >}})
  - [Python]({{< ref "/api/official/python/installation" >}})
  - [Java]({{< ref "/api/official/java/installation" >}})
