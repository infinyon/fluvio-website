---
title: Records 
weight: 30
---

A **Record** is data that is indexed and stored for later use.

Topics generally define a _retention policy_, which describes how long records will be retained. 
If a record lives beyond its retention policy, it may be deleted to make space for other data.

In streaming applications, a record typically communicates  that an
event has occurred, such as a measurement being taken by a sensor, or a button click in
a mobile app.

Any given record belongs to exactly one topic and one partition, and
when it is stored it is assigned an offset.

In addition to carrying arbitrary data, records may optionally be created with a key.

Record keys are used in order to determine which partition within a topic the record
should be sent to.

The golden rule is: any two records with the same key are always
sent to the same partition. If a record does not have a key, it is assigned to a
partition based on some configured strategy, such as round-robin.


