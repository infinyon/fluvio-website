---
title: Consumer
weight: 50
---

Commands for consumers offsets management.

## `fluvio consumer list`

Prints basic information about each consumer offset existing in the cluster.

{{% inline-embed file="embeds/cli/help/fluvio-consumer.md" %}}

### Example usage:

%copy first-line%
```bash
$ fluvio consumer list
  CONSUMER  TOPIC        PARTITION  OFFSET  LAST SEEN
  c1        hello-topic  0          2       3m 51s
  c2        hello-topic  0          2       2m 21s
```

## `fluvio consumer delete`
The command deletes a consumer offset identified by its name. Users have the option to delete offsets for individual partitions or for all partitions within a specific topic.

{{% inline-embed file="embeds/cli/help/fluvio-delete.md" %}}

### Example usage:

%copy first-line%
```bash
$ fluvio consumer delete c1
consumer "c1" on topic "hello-topic" and partition "0" deleted
```

%copy first-line%
```bash
$ fluvio consumer delete c2 --topic hello-topic
consumer "c2" on topic "hello-topic" and partition "0" deleted
consumer "c2" on topic "hello-topic" and partition "1" deleted
```


%copy first-line%
```bash
$ fluvio consumer delete c3 --topic hello-topic --partition 1
consumer "c3" on topic "hello-topic" and partition "1" deleted
```

More information about the **Consumer Offsets** can be found in the details [here].


[here]: {{< ref "/docs/concepts/producer-consumer#consumer-offsets" >}}
 

