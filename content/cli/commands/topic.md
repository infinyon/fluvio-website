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

```
Create a Topic with the given name

fluvio topic create [FLAGS] [OPTIONS] <name>

FLAGS:
    -i, --ignore-rack-assignment
            Ignore racks while computing replica assignment

    -d, --dry-run                   Validates configuration, does not provision
    -h, --help                      Prints help information

OPTIONS:
    -p, --partitions <partitions>
            The number of Partitions to give the Topic [default: 1]

    -r, --replication <integer>
            The number of full replicas of the Topic to keep [default: 1]

    -f, --replica-assignment <file.json>    Replica assignment file

    --retention-time <time>
            Retention time (round to seconds) Ex: '1h', '2d 10s', '7 days' (default)

    --segment-size <bytes>              
        Segment size in bytes

ARGS:
    <name>    The name of the Topic to create
```

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

## `fluvio topic list`

This command shows you all the existing topics in your cluster, as well as some basic
information about them, including how many partitions a topic has and how many times it
is replicated.

```
List all of the Topics in the cluster

fluvio topic list [OPTIONS]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -O, --output <type>    Output [default: table]  [possible values: table,
                           yaml, json]
```

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

```
Print detailed information about a Topic

fluvio topic describe [OPTIONS] <name>

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -O, --output <type>    Output [default: table]  [possible values: table,
                           yaml, json]

ARGS:
    <name>    The name of the Topic to describe
```

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

```
Delete a Topic with the given name

fluvio topic delete <name>

FLAGS:
    -h, --help    Prints help information

ARGS:
    <name>    The name of the Topic to delete
```

Example usage:

%copy first-line%
```bash
$ fluvio topic delete greeting
topic "greeting" deleted
```