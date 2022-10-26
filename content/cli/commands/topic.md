---
title: Topic
weight: 30
---

The `fluvio topic` subcommands are used to create and delete topics, as
well as to view basic information about existing topics.

## `fluvio topic create`

This command is used to create new Fluvio topics.

{{% inline-embed file="embeds/cli/help/fluvio-topic-create.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio topic create greeting
topic "greeting" created
```

### Retention

Retention is a policy for how data is cleaned up from a topic. 

* For a time-based policy, use  `--retention-time`
* For a segment-size based policy, use  `--segment-size`

Check [the docs for more info about data retention]({{<ref "/docs/operations/retention.md">}})

Example usage:

In this example, the last segment of 500k will be deleted after 30 days.

%copy first-line%
```bash
$ fluvio topic create my-topic --retention-time '30 days' --segment-size 500000  
topic "my-topic" created
```


### Compression

This configuration will set compression at a topic level. When set producers are forced to use a compression algorithm that matches with the topic configuration. The SPU will reject any Produce request
that does not match with the topic configuration.

If `--compression-type any` is used, SPU will accept any compression algorithm.

possible values:
* `any`(default)
* `none`
* `gzip`
* `lz4`
* `snappy`

Example usage:

%copy first-line%
```bash
$ fluvio topic create my-topic --compression-type gzip
topic "my-topic" created
```

In this example, the topic `my-topic` will be created with compression type `gzip`.

## `fluvio topic list`

This command shows you all the existing topics in your cluster, as well as some basic
information about them, including how many partitions a topic has and how many times it
is replicated.

{{% inline-embed file="embeds/cli/help/fluvio-topic-list.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio topic list
 NAME      TYPE      PARTITIONS  REPLICAS  IGNORE-RACK  STATUS                   REASON
 greeting  computed      1          1                   resolution::provisioned
```

---

## `fluvio topic describe`

This command prints more detailed information about a specific topic.

{{% inline-embed file="embeds/cli/help/fluvio-topic-describe.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio topic describe greeting
 Name                    :  greeting
 Type                    :  computed
 Partition Count         :  1
 Replication Factor      :  1
 Ignore Rack Assignment  :  false
 Status                  :  provisioned
 Reason                  :
 -----------------
```

---

## `fluvio topic delete`

This command deletes an existing Fluvio topic and all data associated with it.
This data may not be recovered, so use this with care.

{{% inline-embed file="embeds/cli/help/fluvio-topic-delete.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio topic delete greeting
topic "greeting" deleted
```
