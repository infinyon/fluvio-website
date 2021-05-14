---
title: Partition
weight: 40
---

Commands for partition management.

## `fluvio partition list`

Prints basic information about each partition in the cluster, such as
which topic it belongs to, which SPU is leading the partition, and the
various offsets the partition keeps track of.

```
List all of the Partitions in this cluster

fluvio partition list [OPTIONS]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -O, --output <type>    Output [default: table]  [possible values: table,
                           yaml, json]
```

Example usage:

```
$ fluvio partition list
 TOPIC     PARTITION  LEADER  REPLICAS  RESOLUTION  HW  LEO  LSR  FOLLOWER OFFSETS
 greeting  0          5001    []        Online      1   1    0    []
```
