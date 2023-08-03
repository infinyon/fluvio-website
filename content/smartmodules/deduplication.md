---
title: Deduplication
---

The `Deduplication` feature in Fluvio is the way to deduplicate records based on their keys.

Deduplication can be used on a topic. It will drop the duplicate records on a configured window.

This window is defined by the parameters passed to the filter smartmodule.

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

## Parameters

| Parameter       | default | type   | optional | description                                           |
|:-------------|:--------| :---   | :---   |:------------------------------------------------------|
| count |    -    | Integer | false | Minimum number of records the filter will remember. It doesn't guarantee to remember records that came `count` records before now. |
| age   |    -    | Integer | true | Minimum amount of time this filter will remember a record for. It can be specified using this format: `15days 2min 2s`, or `2min 5s`, or `15ms` | |
