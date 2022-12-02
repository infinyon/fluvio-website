---
title: Data Consistency
weight: 80
---
Data in this context is a set of records that producers send to the partition leader. The leader is responsible for receiving
messages from the producer, sending messages to the consumer, and replicating them to followers. There can be only **one leader
per partition** at any point in time. Depending on the cluster and topic configurations, there can be many or zero followers.

Messages get the **order** that the leader observes, and **the reordering is proscribed**. Records order inside the message 
is kept and cannot be changed. Each record gets assigned to a **unique monotonically increased** number called **offset**.

After a record gets accepted by the leader, it can be in one of two states: COMMITTED or UNCOMMITTED. COMMITTED denotes 
that all followers have acknowledged the record. 
If there are no followers in the partition, the state is always COMMITTED once
a record gets acknowledged. Records are UNCOMMITTED in all other cases. 
See more details about [Synchronization Algorithm]({{< ref "/docs/architecture/replica-election#synchronization-algorithm" >}}).

Neither the leader nor the follower waits for **data persistence** (fsync) before sending an acknowledgment of the record. It means that
**uncommitted** records may be lost if Leader crashes.

The leader does not uphold an **atomicity guarantee** for the entire message. Records are processed one by one. If an error occurs, 
the operation aborts, response with an error message is returned, but Fluvio does not roll back previous records from the batch. 

What records state to use is a configurable option for both producers and consumers.

## Producer Isolation
Isolation is a configuration parameter of Producer that has two values:

1. `ReadCommitted` - Leader waits for records to get committed before sending acknowledgement to Producer.
```bash
$ fluvio produce greetings --isolation read_committed
```
<!-- TODO recommend change read-commited to ack_committed -->

2. `ReadUncommitted` - Leader does not wait for records to get committed before sending acknowledgement to Producer.
```bash
$ fluvio produce greetings --isolation read_uncommitted
```

`ReadUncommitted` isolation gives **lower latency** but has **weaker guarantees**. 

If not specified, `ReadUncommitted` isolation is used by default.

-> Producer Isolation determines when a successful delivery has been made for **at-least-once** delivery semantic. [Read details]({{< ref "/docs/concepts/delivery-semantics#at-least-once" >}}).

## Consumer Isolation
Isolation is a configuration parameter of Consumer that has two values:

1. `ReadCommitted` - Read COMMITTED records only. Leader doesn't send UNCOMMITTED records to Consumer.
```bash
$ fluvio consume greetings --isolation read_committed
```

2. `ReadUncommitted` - Read all records regardless of the state.
```bash
$ fluvio consume greetings --isolation read_uncommitted
```

If not specified, `ReadUncommitted` isolation is used by default.



