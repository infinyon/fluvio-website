---
title: Consume
weight: 20
---


The __Consumer__ is responsible for reading messages from data streams in a __Fluvio__ or a __Kafka__ deployment. The messages are written to the queue by the __Producers__.


## Consume Messages

__Consume__ command can operate in two modes:

* __read-one__,
* __read-continuously__.

__Consume__ CLI command has the following operations: 

{{< code >}}
fluvio consume [FLAGS] [OPTIONS] --partition <integer> --topic <string>

FLAGS:
    -g, --from-beginning      Start reading from this offset
    -C, --continuous          Read messages in a infinite loop
    -s, --suppress-unknown    Suppress items items that have an unknown output type

OPTIONS:
    -t, --topic <string>         Topic name
    -p, --partition <integer>    Partition id
    -b, --maxbytes <integer>     Maximum number of bytes to be retrieved
    -c, --sc <host:port>         Address of Streaming Controller
    -u, --spu <host:port>        Address of Streaming Processing Unit
    -k, --kf <host:port>         Address of Kafka Controller
    -P, --profile <profile>      Profile name
    -O, --output <type>          Output [possible values: dynamic, text, binary, json, raw]
{{< /code >}}

The flags and options are defined as follows:

* <strong>{{< pre >}}--from-beginning{{< /pre >}}</strong>: is a flag that instructs the system read from the beginning fo the queue. This is an optional flag; if blank the CLI will wait for the __Producer__ to write to the queue.

* <strong>{{< pre >}}--continuous{{< /pre >}}</strong>: is a flag that instructs the CLI to read from the data stream queue in an infinite loop. Press Ctrl-C, or send SIGINT, to exit loop.

* <strong>{{< pre >}}--suppress-unknown{{< /pre >}}</strong>: is a flag that instructs the CLI to skip messages that were not parsed correctly. Suppress-unknown is used with queues that contain message with mixed types where some messages cannot be successfully parsed. This is an optional flag.

* <strong>{{< pre >}}--topic &lt;string&gt;{{< /pre >}}</strong>:
is the name of the topic from which to read the messages. The topic is a mandatory option and it is used in combination with {{< pre >}}--partition{{< /pre >}} to uniquely identify a data stream.

* <strong>{{< pre >}}--partition &lt;integer&gt;{{< /pre >}}</strong>:
is the partition index of a topic from which to read the messages. The partition is a mandatory option and it is used in combination with {{< pre >}}--topic{{< /pre >}} to uniquely identify a data stream.

* <strong>{{< pre >}}--maxbytes &lt;integer&gt;{{< /pre >}}</strong>:
is the maximum umber of bytes that should be retrieved in one read. The maxbytes field is optional.

* <strong>{{< pre >}}--sc &lt;host:port&gt;{{< /pre >}}</strong>:
is the public interface of the Streaming Controller. The SC is optional and mutually exclusive with {{< pre >}}--spu{{< /pre >}} and {{< pre >}}--kf{{< /pre >}}. The SC is used in combination with [CLI Profiles]({{< relref "overview#profiles" >}}) to compute a target service.

* <strong>{{< pre >}}--spu &lt;host:port&gt;{{< /pre >}}</strong>:
is the public interface of the Streaming Processing Unit. The SPU is optional and mutually exclusive with {{< pre >}}--sc{{< /pre >}} and {{< pre >}}--kf{{< /pre >}}. The SPU is used in combination with [CLI Profiles]({{< relref "overview#profiles" >}}) to compute a target service.

* <strong>{{< pre >}}--kf &lt;host:port&gt;{{< /pre >}}</strong>:
is the public interface of the Kafka Controller. The KF is optional and mutually exclusive with {{< pre >}}--sc{{< /pre >}} and {{< pre >}}--spu{{< /pre >}}. The KF is used in combination with [CLI Profiles]({{< relref "overview#profiles" >}}) to compute a target service.

* <strong>{{< pre >}}--profile &lt;profile&gt;{{< /pre >}}</strong>:
is the custom-defined profile file. The profile is an optional field used to compute a target service. For additional information, see [Target Service]({{< relref "overview#target-service" >}}) section.

* <strong>{{< pre >}}--output &lt;type&gt;{{< /pre >}}</strong>:
is the format to be used to display the messages. The output is an optional field and it defaults to __dynamic__ format, where the parser will guess the message type. Alternative formats are: __text__, __binary__, __json__, __raw__.


### Consume Messages Examples 

#### Consume Messages from Fluvio SC

... Fluvio SC

#### Consume Messages from Fluvio SPU

... Fluvio SPU

#### Consume Messages for Kafka

... Kafka



{{< links "Related Topics" >}}
* [Produce CLI]({{< relref "produce" >}})
* [SPUs CLI]({{< relref "spus" >}})
* [Custom SPU CLI]({{< relref "custom-spus" >}})
* [SPU-Groups CLI]({{< relref "spu-groups" >}})
* [Topics CLI]({{< relref "topics" >}})
{{< /links >}}