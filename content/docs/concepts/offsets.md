---
title: Offsets 
weight: 40
---

When a record is assigned an offset, that offset permanently identifies
that record. The offset that identified that record will never be reused for another record.

In order to begin consuming records, a consumer must specify the topic and
partition to consume from, as well as the offset into the partition where
it should begin reading.

The offset of a record is its total
position within its parent partition.

There are multiple ways that
an offset may be derived in more convenient ways:

- Directly, as an absolute index into the partition, starting from zero
- As a relative distance from the beginning of the partition
- As a relative distance from the end of the partition


There is
 a  difference between an absolute offset and a relative offset
from the beginning of the partition.

When consumers specify a relative offset, the offset given by the consumer
is used to calculate the actual total offset into the partition.

When a record is assigned an offset, that offset permanently identifies
that record, but this does not necessarily mean that the record will always be available.

If a partition has a retention policy
that causes it to begin deleting records from the beginning, then the
relative-from-beginning offset will count forward from the oldest record
that is still available.

