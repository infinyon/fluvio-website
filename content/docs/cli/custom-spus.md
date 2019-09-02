---
title: Custom SPUs
weight: 50
---

__Custom SPUs__ allow Fluvio __Streaming Controller__ (__SC__) to identify and manage __SPU__ engines that are provisioned out-of-band. The __Custom-SPU__ informs the __SC__ that an __SPU__ engine will attach to the deployment at some point in the future. The __Custom-SPU__ is used in the replica assigmenet as soon as it is configured. Inially it is marked offline until the __SPU__ service connects to the __SC__. 

{{< caution >}}
Defining multiple Custom-SPUs without an associated __SPU__ service will yield a suboptimal replica assigment. Use caution when provisioning them.
{{< /caution >}}

Custom-SPU module defines the following CLI operations: 

{{< code >}}
fluvio custom-spu <SUBCOMMAND>

SUBCOMMANDS:
    create    Create custom SPU
    delete    Delete custom SPU
    list      List custom SPUs
{{< /code >}}

## Create Custom-SPU

Create __Custom-SPU__ operation adds a custom SPU to a __Fluvio__ deployment. 

{{< code >}}
fluvio custom-spu create [OPTIONS] --id <id> --private-server <host:port> --public-server <host:port>

OPTIONS:
    -i, --id <id>                       SPU id
    -n, --name <string>                 SPU name
    -r, --rack <string>                 Rack name
    -p, --public-server <host:port>     Public server::port
    -v, --private-server <host:port>    Private server::port
    -c, --sc <host:port>                Address of Streaming Controller
    -P, --profile <profile>             Profile name
{{< /code >}}

The options are defined as follows:

* <strong>{{< pre >}}--id &lt;id&gt;{{< /pre >}}</strong>:
is the id of the SPU that is authorized to attach to this deployment. The SPU id is compared against the __SPU__ service id when the service connects to the __SC__. __SPU__ services that do not have a matching __Custom-SPU__ id are rejected. __Id__ is a mandatory option and it must be unique to the Fluvio deployment.

* <strong>{{< pre >}}--name &lt;string&gt;{{< /pre >}}</strong>:




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
is the custom-defined profile file. The profile is an optional field used to compute a target service. For additional information, see [Target Service]({{< relref "overview#target-service" >}}) section.
