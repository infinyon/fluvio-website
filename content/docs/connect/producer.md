---
title: Producers
weight: 10
---

"Producer" is the name we give to applications who "produce" streaming data.
These applications may be monitoring the status of a system, collecting sensor
data, watching traffic to a website, or otherwise observing events in some way.
Producers may be general-purpose tools such as [the Fluvio CLI], or they may be
special-purpose programs built to meet a specific need, in which case the
producer program would leverage [one of the programmatic APIs]. Let's talk about
the core concepts that producers deal with in order to better understand how
they work.

[the Fluvio CLI]: /cli
[one of the programmatic APIs]: /api

## Records, Topics, and Partitions

In order to understand producers, we need to have a good idea about the data they
interact with (Records), the means by which they logically organize that data (Topics),
and the strategies they use in order to efficiently distribute and process that data
(Partitions).

### Records

In Fluvio, like in many other streaming systems, a Record is simply a piece of data
that is indexed and stored for later use. In streaming applications, a Record typically
communicates the fact that a particular event has occurred, such as a measurement being
taken by a sensor, or a button click in a mobile app.

- Role of keys
  - Hashing keys and round-robin
- Values as bytes/json/etc

### Topics

- Topics are like tables in a database
- Topics define properties such as number of partitions and replication factor
- Topics may categorize different domains of data
- A Producer must specify a Topic for each record being sent

### Partitions

- The mechanism for scaling up data throughput
- Partitions are subdivisions of a Topic
- All records sent to a specific partition are ordered
