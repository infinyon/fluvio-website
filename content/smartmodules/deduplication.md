---
title: Deduplication
---

The `Deduplication` feature in Fluvio is the way to deduplicate records based on their keys.

Deduplication can be used on a topic.
It will drop the duplicate records in a window which is defined by the configured `bounds`.

The current supported `bounds` are `age` and `count`, they are explained in the [bounds section](#bounds).

## Behavior

The deduplication function is deterministic and survives restarts. During restart the deduplication algorithm scans the data stream and rebuilds the memory object.

## Example topic config

Example configuration on topic:

%copy%
```yaml
# topic.yaml
version: 0.1.0
meta:
  name: topic-with-dedup
deduplication:
  bounds:
    count: 5 # remember at least 5 last records
    age: 5s # remember records for at least 5 seconds
  filter:
    transform:
      uses: infinyon-labs/dedup-filter@0.0.2 
```
A topic can be created using this config file like so:

%copy first-line%
```bash
$ fluvio topic create -c topic.yaml
```

After creating the topic, it can be tested like so:

%copy first-line%
```bash
$ fluvio produce topic-with-dedup --key-separator :
1:2
1:2
2:5
```

%copy first-line%
```bash
$ fluvio consume -B topic-with-dedup
2
5
```

## Bounds

| Parameter       | default | type   | optional | description                                           |
|:-------------|:--------| :---   | :---   |:------------------------------------------------------|
| count |    -    | Integer | false | Minimum number of records the filter will remember. It doesn't guarantee to remember records that came `count` records before now. |
| age   |    -    | Integer | true | Minimum amount of time this filter will remember a record for. It can be specified using this format: `15days 2min 2s`, or `2min 5s`, or `15ms` | |

## Implementation

The specific algorithm used for deduplication is defined by a smartmodule. Right now the only smartmodule for this is the `dedup-filter` smartmodule.

The `dedup-filter` splits the data into smaller chunks and keeps these chunks in memory. Each chunk has an age so it knows if it can
drop a chunk based on it's age.

When count of total records in memory exceeds the configured `bounds.count`, it will check if it can delete the oldest record chunk using the configured `bounds.age` and delete it if it can. This allows for fast deletion of old data with minimum amount of bookkeeping.

The chunking of data introduces a small memory usage overhead but allows the filter to have no spikes in execution time and memory usage.
