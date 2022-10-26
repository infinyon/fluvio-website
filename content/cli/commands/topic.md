---
title: Topic
weight: 30
---

The `fluvio topic` family of commands is used to create and delete topics, as
well as to view basic information about existing topics.

## `fluvio topic create`

This command is used to create new Fluvio topics. A Fluvio topic is a stream where
you send related messages. Different topics have unique names and store their data
independently. They may also be divided into multiple partitions, which can
increase the message throughput of the topic.

{{% inline-embed file="embeds/cli/help/fluvio-topic-create.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio topic create greeting
topic "greeting" created
```

### Retention

If  you want to set a retention time for the topic, you can use the `--retention-time` parameter. In fluvio, the records are organized in segments. Each segment has a fixed size, and it can be configured with the `--segment-size` param. Any segments older than the retention time will be deleted.

Example usage:

%copy first-line%
```bash
$ fluvio topic create my-topic --retention-time '30 days' --segment-size 500000  
topic "my-topic" created
```

In this example, the last segment of 500k will be deleted after 30 days.

### Compression

If you want to set topic level compression, you can use the `--compression-type` parameter, possible values are `any`(default), `none`, `gzip`, `lz4` and `snappy`.
This configuration will enforce producers to use a compression algorithm that matches with the topic configuration. The SPU will reject any Produce request
that does not match with the topic configuration. If `--compression-type any` is used, SPU will accept any compression algorithm.

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
