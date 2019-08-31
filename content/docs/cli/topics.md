---
title: Topics
weight: 60
---

The __topic__ is the primary construct for creating data streams. A topic coupled with a partition creates a unique identifier for a data stream in a Fluvio deployment. The __topic/partition__ unique identifier is used by the producers and the consumers to exchange messages over a data stream.

## Topic CLI

Topics module has the following CLI operations: 

{{< code >}}
fluvio topic <SUBCOMMAND>

SUBCOMMANDS:
    create      Create a topic
    delete      Delete a topic
    describe    Show details of a topic
    list        Show all topics
{{< /code >}}


### Create Topic

__Create topic__ operation can add a topic to a __Fluvio__ or a __Kafka__ deployment. 

{{< code >}}
fluvio topic create [FLAGS] [OPTIONS] --partitions <integer> --replication <integer> --topic <string>

FLAGS:
    -i, --ignore-rack-assignment    Ignore racks while computing replica assignment
    -v, --validate-only             Validates configuration, does not provision

OPTIONS:
    -t, --topic <string>                    Topic name
    -p, --partitions <integer>              Number of partitions
    -r, --replication <integer>             Replication factor per partition
    -f, --replica-assignment <file.json>    Replica assignment file
    -c, --sc <host:port>                    Address of Streaming Controller
    -k, --kf <host:port>                    Address of Kafka Controller
    -P, --profile <profile>                 Profile name
{{< /code >}}

The flags and options are defined as follows:

* <strong>{{< pre >}}--ignore-rack-assignment{{< /pre >}}</strong>: 
[rack](create_topic) labels are optional parameters assiged to SPUs. The rack label is used during replica assignment to compute rack-aware replica distribution. Apply *ignore_rack_assignment* flag to skip rack-aware assignment. If the SPUs do not have rack labels assigned, the flag is ignored.

* <strong>{{< pre >}}--validate-only{{< /pre >}}</strong>: 
allows you to check if the configuration is correct without applying the changes to the system. This flag is particularly useful to ensure a custom {{< pre >}}--replica-assignment{{< /pre >}} is correct before applying it to the system.

#### Fluvio Example

... Fluvio

#### Kafka Example

... Kafka

