---
title: Producers and Consumers
weight: 10
---

## Producer

Producers are applications that "produce" streaming data.
These applications may be monitoring the status of a system, collecting sensor
data, watching traffic to a website, or otherwise observing events in some way.
Producers may be general-purpose tools such as [the Fluvio CLI], or they may be
special-purpose programs built to meet a specific need, in which case the
producer program would leverage [one of the programmatic APIs].

[the Fluvio CLI]: {{< ref "/cli" >}}
[one of the programmatic APIs]: {{< ref "/api" >}}


## Consumer 
Consumers are applications that "consume" records from a particular
topic and partition<sup>[1]</sup>. Typically, a consumer will perform actions based
on the events it receives, such as sending a notification or updating
a database. There may be many consumers reading data from a particular
partition at any given time: since the records are persisted, they do
not expire after being consumed.

-> [1] For an overview of Topics and Partitions, see the [Topic documentation]

[Topic documentation]: {{< ref "/docs/concepts/topics" >}}
