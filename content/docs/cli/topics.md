---
title: Topics
weight: 60
---

The __topic__ is the primary construct for creating a data stream. A topic coupled with a partition creates a unique identifier for a data stream in a Fluvio deployment. The __topic/partition__ unique identifier is used by the producers and the consumers to exchange messages over a data stream.  

Topics module has the following CLI operations: 

{{< code >}}
fluvio topic <SUBCOMMAND>

SUBCOMMANDS:
    create      Create a topic
    delete      Delete a topic
    describe    Show details of a topic
    list        Show all topics
{{< /code >}}


## Create Topic

__Create topic__ operation adds a topic to a __Fluvio__ or a __Kafka__ deployment. 

{{< code >}}
fluvio topic create [FLAGS] [OPTIONS] --topic <string> --partitions <integer> --replication <integer> 

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

* <strong>{{< pre >}}--topic &lt;string&gt;{{< /pre >}}</strong>:
is the name of the topic to be created. Topic is a mandatory option and it must be unique to the Fluvio or Kafka deployment.

* <strong>{{< pre >}}--partitions &lt;integer&gt;{{< /pre >}}</strong>:
is the number of partitions for the topic. Partitions is mandatory but mutually exclusive with *replica-assigment*. It must be a number greater than 0.

* <strong>{{< pre >}}--replication &lt;integer&gt;{{< /pre >}}</strong>:
is the replication factor for the topic. Replication is mandatory but mutually exclusive with *replica-assigment*. For example, a replication factor of 3 ensures that each message is replicated to three different SPUs. Replication must be a number greater than 0.

* <strong>{{< pre >}}--replica-assignment &lt;file.json&gt;{{< /pre >}}</strong>:
is the custom-defined replica assignment file. Replica-assignment is mutually exclusive with *partitions* and *replication*. The replica assignment file allows you to replace Fluvio's built-in replication algorithm with your custom SPU map for each topic/partitions.  

    The replica-assignment JSON file syntax is as follows:

    {{< code toml >}}
{ 
    "partitions": [
        {
            "id": <partition-id>,
            "replicas": [
                <spu-id>, 
                ...
            ]
        }, 
        ...
    ]
}
{{< /code >}}

    The following example shows a replica assignment file with 2 partitions and 3 replicas: 

    {{< code toml >}}
{
    "partitions": [
        {
            "id": 0,
            "replicas": [
                0,
                2,
                3
            ]
        },
        {
            "id": 1,
            "replicas": [
                1,
                3,
                2
            ]
        }
    ]
}
{{< /code >}}

* <strong>{{< pre >}}--sc &lt;host:port&gt;{{< /pre >}}</strong>:
is the public interface of the Streaming Cotroller. The SC is optional and mutually exclusive with {{< pre >}}--kf{{< /pre >}}. The SC is computed in combination with [CLI Profiles]({{< relref "overview#profiles" >}}).

* <strong>{{< pre >}}--kf &lt;host:port&gt;{{< /pre >}}</strong>:
is the public interface of the Kafka Cotroller. The KF is optional and mutually exclusive with {{< pre >}}--sc{{< /pre >}}. The KF is computed in combination with [CLI Profiles]({{< relref "overview#profiles" >}}).

* <strong>{{< pre >}}--profile &lt;profile&gt;{{< /pre >}}</strong>:
is the custom-defined profile file. The profile is an optional field used to compute a target service. For additional information, check out [Target Service]({{< relref "overview#target-service" >}}) section.

### Create Topic Examples 

#### Create a Fluvio Topic

... Fluvio

#### Create a Kafka Topic

... Kafka


## Delete Topic

__Delete topic__ operation deletes a topic from a __Fluvio__ or a __Kafka__ deployment. 

{{< code >}}
fluvio topic delete [OPTIONS] --topic <string>

OPTIONS:
    -t, --topic <string>       Topic name
    -c, --sc <host:port>       Address of Streaming Controller
    -k, --kf <host:port>       Address of Kafka Controller
    -P, --profile <profile>    Profile name
{{< /code >}}

The options are defined as follows:

* <strong>{{< pre >}}--topic &lt;string&gt;{{< /pre >}}</strong>:
is the name of the topic to be deleted. Topic is a mandatory option.

* <strong>{{< pre >}}--sc &lt;host:port&gt;{{< /pre >}}</strong>:
Defined in [Create Topic](#create-topic)

* <strong>{{< pre >}}--kf &lt;host:port&gt;{{< /pre >}}</strong>:
Defined in [Create Topic](#create-topic)

* <strong>{{< pre >}}--profile &lt;profile&gt;{{< /pre >}}</strong>:
Defined in [Create Topic](#create-topic)


### Delete Topic Examples 

#### Delete a Fluvio Topic

... Fluvio

#### Delete a Kafka Topic

... Kafka

## Describe Topics

__Describe topics__ operation describes one or more a topics in a __Fluvio__ or a __Kafka__ deployment. 

{{< code >}}
fluvio topic describe [OPTIONS]

OPTIONS:
    -t, --topics <string>...   Topic names
    -c, --sc <host:port>       Address of Streaming Controller
    -k, --kf <host:port>       Address of Kafka Controller
    -P, --profile <profile>    Profile name
    -O, --output <type>        Output [possible values: table, yaml, json]
{{< /code >}}

The options are defined as follows:

* <strong>{{< pre >}}--topics &lt;string&gt;{{< /pre >}}</strong>:
are the names of the topics to be described. A list of one or more topic names is required.

* <strong>{{< pre >}}--sc &lt;host:port&gt;{{< /pre >}}</strong>:
Defined in [Create Topic](#create-topic)

* <strong>{{< pre >}}--kf &lt;host:port&gt;{{< /pre >}}</strong>:
Defined in [Create Topic](#create-topic)

* <strong>{{< pre >}}--profile &lt;profile&gt;{{< /pre >}}</strong>:
Defined in [Create Topic](#create-topic)

* <strong>{{< pre >}}--output &lt;type&gt;{{< /pre >}}</strong>:
is the format to be used to display the topics. The output is an optional field and it defaults to __table__ format. Alternative format types are: __yaml__ and __json__.


### Describe Topics Examples 

#### Describe Fluvio Topics

... Fluvio

#### Describe Kafka Topics

... Kafka

## List Topics

__List topics__ operation lists all topics in a __Fluvio__ or a __Kafka__ deployment. 

{{< code >}}
fluvio topic list [OPTIONS]

OPTIONS:
    -c, --sc <host:port>       Address of Streaming Controller
    -k, --kf <host:port>       Address of Kafka Controller
    -P, --profile <profile>    Profile name
    -O, --output <type>        Output [possible values: table, yaml, json]
{{< /code >}}

The options are defined as follows:

* <strong>{{< pre >}}--sc &lt;host:port&gt;{{< /pre >}}</strong>:
Defined in [Create Topic](#create-topic)

* <strong>{{< pre >}}--kf &lt;host:port&gt;{{< /pre >}}</strong>:
Defined in [Create Topic](#create-topic)

* <strong>{{< pre >}}--profile &lt;profile&gt;{{< /pre >}}</strong>:
Defined in [Create Topic](#create-topic)

* <strong>{{< pre >}}--output &lt;type&gt;{{< /pre >}}</strong>:
Defined in [Describe Topics](#describe-topics)


### List Topics Examples 

#### List Fluvio Topics

... Fluvio

#### List Kafka Topics

... Kafka

{{< links "Related Topics" >}}
* [Produce CLI]({{< relref "produce" >}})
* [Consume CLI]({{< relref "consume" >}})
* [SPUs CLI]({{< relref "spus" >}})
* [Custom SPU CLI]({{< relref "custom-spus" >}})
* [SPU-Groups CLI]({{< relref "spu-groups" >}})
{{< /links >}}