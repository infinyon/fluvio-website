---
title: Partition
weight: 40
---

Commands for partition management.

## `fluvio partition list`

Prints basic information about each partition in the cluster, such as
which topic it belongs to, which SPU is leading the partition, and the
various offsets the partition keeps track of.

{{% inline-embed file="embeds/cli/help/fluvio-partition.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio partition list
 TOPIC     PARTITION  LEADER  REPLICAS  RESOLUTION  SIZE  HW  LEO  LRS  FOLLOWER OFFSETS 
 greeting  0          0       []        Online      86 B  1   1    0    [] 
```


More information about the columns HW, LEO, and LRS can be found in the details regarding the [Synchronization Algorithm].


[Synchronization Algorithm]: {{< ref "/docs/architecture/replica-election#synchronization-algorithm" >}}
 
