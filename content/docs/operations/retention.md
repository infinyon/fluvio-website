---
title: Data Retention
weight: 20
---

## Overview

Topic data is automatically pruned when **any** of following criteria is true:
1. Partition size exceeds the configured max partition size
2. Elapsed time since the last write to segment has passed the configured retention time

-> Data eviction operates on the segment level. If any above conditions are met, the entire segment gets removed. Only previous segments can be pruned. If your data resides in the active segment, it won't be evicted unless the segment turns to a historical (read-only) segment.


## Configuring retention

Retention is configured per-topic at the time of topic creation with `fluvio topic create`.

```shell
$ fluvio topic create -h
Create a Topic with the given name

fluvio-stable topic create [FLAGS] [OPTIONS] <name>
        [...]
        --retention-time <time>
            Retention time (round to seconds) Ex: '1h', '2d 10s', '7 days' (default)

        --segment-size <bytes>
            Segment size (by default measured in bytes) Ex: `2048`, '2 Ki', '10 MiB', `1 GB`

        --max-partition-size <bytes>
            Max partition size (by default measured in bytes) Ex: `2048`, '2 Ki', '10 MiB', `1 GB`

ARGS:
    <name>    The name of the Topic to create
```

## Retention time

Retention time duration can be provided as a free-form string. Check out the [`humantime` docs](https://docs.rs/humantime/2.1.0/humantime/fn.parse_duration.html) for the supported time suffixes.

The default retention time is `7 days`

### Example retention configurations
* Delete old segments that are `6 hours` old 

%copy first-line%
```bash
$ fluvio topic create test1 --retention-time "6h"
```

%copy first-line%
```bash
$ fluvio topic list
 NAME         TYPE      PARTITIONS  REPLICAS  RETENTION TIME  STATUS                   REASON 
 test1        computed      1          1            6h        resolution::provisioned
```

* Delete old segments that are a day old

%copy first-line%
```bash
$ fluvio topic create test2 --retention-time "1d"
```

%copy first-line%
```bash
$ fluvio topic list
 NAME         TYPE      PARTITIONS  REPLICAS  RETENTION TIME  STATUS                   REASON 
 test2        computed      1          1           1day       resolution::provisioned 
```

* A very specific duration that is 1 day, 2 hours, 3 minutes and 4 seconds long

%copy first-line%
```bash
$ fluvio topic create test3 --retention-time "1d 2h 3m 4s"
```

%copy first-line%
```bash
$ fluvio topic list
 NAME         TYPE      PARTITIONS  REPLICAS  RETENTION TIME  STATUS                   REASON 
 test3        computed      1          1      1day 2h 3m 4s   resolution::provisioned
```

## Segment size

Produced records persist on the SPU in file chunks that cannot exceed the segment size.

If adding a new record to the active segment will would result in exceeding the segment size, it is saved into a new segment.

Older segments are still available for consumption until they get pruned when the eviction condition is met.

Segment size can be provided as a free form string. Check out the [`bytesize` docs](https://github.com/hyunsik/bytesize/) 
for the supported size suffixes.

The default segment size is `1 GB`

### Example retention configurations
* 25 MB segment size w/ 7 day retention time

%copy first-line%
```bash
$ fluvio topic create test4 --segment-size 25000000
```

* 36 GB segment size w/ 12 hr retention time

%copy first-line%
```bash
$ fluvio topic create test5 --segment-size "36 GB" --retention-time 12h
```

## Max partition size

Fluvio keeps tracking all memory that a partition occupies on **SPU** node. It includes the payload and all bookkeeping data.
If partition size exceeds the max partition size property Fluvio triggers segments eviction. The oldest segment is deleted first. 
The size enforcing operation provides `best-effort` guarantee. There might be time windows when the actual partition size may
exceed the configured max partition size. It is recommended to configure max partitions sizes to cover up to 80% of the disk size.
If the disk is full before the retention period is triggered, the SPU stops accepting messages and the overall health of the system 
may be compromised.

Max partition can be provided as a free-form string. Check out the [`bytesize` docs](https://github.com/hyunsik/bytesize/) 
for the supported size suffixes.

The default max partition size is `100 GB`.  
The max partition size must not be less than segment size. 

### Example retention configurations

* 10 GB max partition size w/ 1 GB segment size (only 10 segments are allowed at any time)

%copy first-line%
```bash
fluvio topic create test6 --max-partition-size '10 GB' --segment-size '1 GB'
```

## Example data lifecycle

For a given topic with a retention of `7 days` using `1 GB` segments

* Day 0: 2.5 GB is written (total topic data: 2.5 GB)

| Topic Segment # | Segment size | Days since last write |
|-----------------|--------------|-----------------------|
| 0               | 1 GB         | 0                     |
| 1               | 1 GB         | 0                     |
| 2               | 0.5 GB       | N/A                   |

* Day 6: Another 2 GB is written (total topic data: 4.5 GB,)

| Topic Segment # | Segment size | Days since last write |
|-----------------|--------------|-----------------------|
| 0               | 1 GB         | 6                     |
| 1               | 1 GB         | 6                     |
| 2               | 1 GB         | 0                     |
| 3               | 1 GB         | 0                     |
| 4               | 0.5 GB       | N/A                   |

* Day 7: 2 segments from Day 0 are 7 days old. They are pruned (total topic data: 2.5 GB)

| Topic Segment # | Segment size | Days since last write |
|-----------------|--------------|-----------------------|
| 2               | 1 GB         | 1                     |
| 3               | 1 GB         | 1                     |
| 4               | 0.5 GB       | N/A                   |

* Day 14: 2 segments from Day 7 are 7 days old. They are pruned (total topic data: 0.5 GB)

| Topic Segment # | Segment size | Days since last write |
|-----------------|--------------|-----------------------|
| 4               | 0.5 GB       | N/A                   |

The newest segment is left alone and only begins to age once a new segment is being written to.

For a given topic with max partition size is `3 GB` and `1 GB` segments
* 2.5 GB is written (total partition data: 2.5 GB)

| Topic Segment # | Segment size |
|-----------------|--------------|
| 0               | 1 GB         |
| 1               | 1 GB         |
| 2               | 0.5 GB       |

* 600 MB is written. The total size becomes 3.1 GB. The first segment is pruned.

| Topic Segment #     | Segment size |
|---------------------|--------------|
| 1                   | 1 GB         |
| 2                   | 1 GB         |
| 3                   | 0.1 GB       |