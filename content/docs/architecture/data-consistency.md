---
title: Data Consistency
weight: 80
---
Data in this context is a set of records that Producer sends to the partition leader. The leader is responsible for receiving
messages from Producer, sending messages to Consumer, and doing the replication to followers. There can be only **one Leader
per partition** at any point in time. There can be many or zero followers depending on the cluster and topic configurations.

Messages get the **order** that Leader observes and **can not be reordered**. Records order inside the message is kept and can not
be changed. Each record gets assigned to a **unique monotonically increased** number called **offset**.

After a record is accepted by Leader, it can be in one of two states: COMMITTED or UNCOMMITTED. COMMITTED denotes 
that the record has been replicated to all followers. More precisely, the offset of the record is less or equal to **HW**
of the partition. See more details about [Synchronization Algorithm]({{< ref "/docs/architecture/replica-election#synchronization-algorithm" >}}).
UNCOMMITTED records are not replicated yet. If there are no followers in the partition, the state is always COMMITTED once
a record gets acknowledged.

Neither Leader nor Follower waits for **data persistence** (fsync) before sending an acknowledgment of the record. It means that
**uncommitted** records may be lost if Leader crashes.

Leader does not uphold an **atomicity guarantee** for the entire message. Records are processed one by one. If an error occurs, 
the operation is aborted, response with an error message is returned but previous records from the batch **are not rolled back**. 

Both Producer and Consumer can be configured to a specific state of records that it wants to deal with.

## Producer Isolation
Isolation is a configuration parameter of Producer that has two values:

1. `ReadCommitted` - Leader waits for records to be committed before sending ack to Producer.
```bash
$ fluvio produce greetings --isolation read_committed
```

2. `ReadUncommitted` - Leader does not wait for records to be committed before sending ack to Producer.
```bash
$ fluvio produce greetings --isolation read_uncommitted
```

`ReadUncommitted` isolation gives **lower latency** but has **weaker guarantees**. 

If not specified, `ReadUncommitted` isolation is used by default.

## Consumer Isolation
Isolation is a configuration parameter of Consumer that has two values:

1. `ReadCommitted` - Read COMMITTED records only. UNCOMMITTED records won't be sent to Consumer from Leader.
```bash
$ fluvio consume greetings --isolation read_committed
```

2. `ReadUncommitted` - Read all records regardless of the state.
```bash
$ fluvio consume greetings --isolation read_uncommitted
```

If not specified, `ReadUncommitted` isolation is used by default.



