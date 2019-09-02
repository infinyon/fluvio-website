---
title: SPUs
weight: 30
---

The __SPUs__, also known as __Streaming Processing Units__, are the service engines responsible for processing data streams. A data stream has one leader and one or more followers. The leader and the followers are evenly distributed across the __SPUs__. For additional information on leader and follower replica distribution, see [Fluvio Architecture]({{< relref "../architecture/overview" >}}).

Fluio supports two types of SPUs:

* __Managed SPUs__: defined in [SPU-Groups CLI]({{< relref "spu-groups" >}}).
* __Custom SPUs__: defined in [Custom-SPUs CLI]({{< relref "custom-spus" >}})

SPU module defines the CLI operations that are common both SPU types: 

{{< code >}}
fluvio spu <SUBCOMMAND>

SUBCOMMANDS:
    list    List custom & managed SPUs
{{< /code >}}


## List SPUs

List __SPUs__ operation lists all managed and custom SPUs in a __Fluvio__ deployment. 

{{< code >}}
fluvio spu list [OPTIONS]

OPTIONS:
    -c, --sc <host:port>       Address of Streaming Controller
    -P, --profile <profile>    Profile name
    -O, --output <type>        Output [possible values: table, yaml, json]
{{< /code >}}

The options are defined as follows:

* <strong>{{< pre >}}--sc &lt;host:port&gt;{{< /pre >}}</strong>:
is the public interface of the Streaming Cotroller. The SC is an optional field and it is computed in combination with [CLI Profiles]({{< relref "overview#profiles" >}}).

* <strong>{{< pre >}}--profile &lt;profile&gt;{{< /pre >}}</strong>:
is the custom-defined profile file. The profile is an optional field used to compute a target service. For additional information, see [Target Service]({{< relref "overview#target-service" >}}) section.

* <strong>{{< pre >}}--output &lt;type&gt;{{< /pre >}}</strong>:
is the format to be used to display the SPUs. The output is an optional field and it defaults to __table__ format. Alternative format types are: __yaml__ and __json__.

### List SPUs Example

... Fluvio


{{< links "Related Topics" >}}
* [Produce CLI]({{< relref "produce" >}})
* [Consume CLI]({{< relref "consume" >}})
* [Custom SPU CLI]({{< relref "custom-spus" >}})
* [SPU-Groups CLI]({{< relref "spu-groups" >}})
* [Topics CLI]({{< relref "topics" >}})
{{< /links >}}