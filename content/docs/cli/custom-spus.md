---
title: Custom SPUs
weight: 50
---

__Custom SPUs__ allow Fluvio __Streaming Controller__ (__SC__) to identify and manage __SPU__ services that are provisioned out-of-band. The __Custom-SPU__ informs the __SC__ that an __SPU__ service will attach to the deployment at some point in the future. The __Custom-SPU__ is used in the replica assignment as soon as it is configured. Initially it is marked offline until the __SPU__ service connects to the __SC__. 

{{< caution >}}
Defining multiple Custom-SPUs without an associated __SPU__ service will yield a suboptimal replica assignment. Use caution when provisioning them.
{{< /caution >}}

## Generate a Custom-SPU binary

Fluvio publishes and maintains SPU images in Docker Container format, other binary formats must be compiled from source code. 

* [fluvio-sc](https://hub.docker.com/r/infinyon/fluvio-sc)
* [fluvio-spu](https://hub.docker.com/r/infinyon/fluvio-spu)

While __fluvio-spu__ container can be used as a __Custom-SPU__, it is more common to compile the SPU image from source code for the intended target.

{{< idea >}}
Fluvio [Developer Guide](https://github.com/infinyon/fluvio/blob/master/DEVELOPER.md) provides step-by-step instructions to compile an SPU image from source code.
{{< /idea >}}

The next steps must be performed in the following sequence:

1. Register __custom-spu__ with the __SC__
2. Run __spu-server__ binary compiled above


## Custom-SPU CLI

Custom-SPU module defines the following CLI operations: 

{{< code >}}
fluvio custom-spu <SUBCOMMAND>

SUBCOMMANDS:
    register    Register custom SPU
    unregister  Unregister custom SPU
    list        List custom SPUs
{{< /code >}}

## Register Custom-SPU

Register __Custom-SPU__ operation informs the __SC__ that a custom __SPU__ with the specific id is authorized to join to a __Fluvio__ deployment. 

{{< code >}}
fluvio custom-spu register [OPTIONS] --id <id> --private-server <host:port> --public-server <host:port>

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
is the identifier of the SPU that is authorized to be managed by a Fluvio deployment. The Custom-SPU id is compared with the SPU service id every time the service connects to the SC. SPU services that do not have a matching Custom-SPU id are rejected. The id is mandatory and it must be unique to the Fluvio deployment.

* <strong>{{< pre >}}--name &lt;string&gt;{{< /pre >}}</strong>:
is the name of the Custom-SPU. The name is optional and it is automatically generated if left empty. The format for auto-generated Custom-SPU names is: _spu-[id]_.

* <strong>{{< pre >}}--rack &lt;string&gt;{{< /pre >}}</strong>:
is the rack label of the Custom-SPU. Rack names have an impact on the *replica-assignment* when new topics are provisioned. The rack is an optional field.

* <strong>{{< pre >}}--public-server &lt;host:port&gt;{{< /pre >}}</strong>:
is the public interface of the Custom-SPU services. The public server information is used by Produce/Consumer to connect with the leader of a topic/partition. The public server is a mandatory field.

* <strong>{{< pre >}}--private-server &lt;host:port&gt;{{< /pre >}}</strong>:
is the private interface of the Custom-SPU service. SPUs establish private connections to negotiate leader election and replicate data from leaders to followers. The private server is a mandatory field.

* <strong>{{< pre >}}--sc &lt;host:port&gt;{{< /pre >}}</strong>:
is the public interface of the Streaming Controller. The SC is an optional field used in combination with [CLI Profiles]({{< relref "overview#profiles" >}}) to compute a target service.

* <strong>{{< pre >}}--profile &lt;profile&gt;{{< /pre >}}</strong>:
is the custom-defined profile file. The profile is an optional field used to compute a target service. For additional information, see [Target Service]({{< relref "overview#target-service" >}}) section.

### Register Custom-SPU Example




## Unregister Custom-SPU

Unregister __Custom-SPU__ operation informs the __SC__ that the __SPU__ is no longer authorized to participate in this __Fluvio__ deployment. The __SC__ rejects all new connections from the __SPU__ service associated with this __Custom-SPU__.

{{< code >}}
fluvio custom-spu unregister [OPTIONS] --id <id>

OPTIONS:
    -i, --id <id>              SPU id
    -n, --name <string>        SPU name
    -c, --sc <host:port>       Address of Streaming Controller
    -P, --profile <profile>    Profile name
{{< /code >}}

The options are defined as follows:

* <strong>{{< pre >}}--id &lt;id&gt;{{< /pre >}}</strong>:
is the identifier of the Custom-SPU to be detached. Id is a mandatory and mutually exclusive with {{< pre >}}--name{{< /pre >}}.

* <strong>{{< pre >}}--name &lt;string&gt;{{< /pre >}}</strong>:
is the name of the Custom-SPU to be detached. Name is a optional and mutually exclusive {{< pre >}}--id{{< /pre >}}.

* <strong>{{< pre >}}--sc &lt;host:port&gt;{{< /pre >}}</strong>:
See [Register Custom-SPU](#register-custom-spu)

* <strong>{{< pre >}}--profile &lt;profile&gt;{{< /pre >}}</strong>:
See [Register Custom-SPU](#register-custom-spu)

### Unregister Custom-SPU Example

... Fluvio


## List Custom-SPUs

List __Custom-SPUs__ operation lists all custom SPUs in a __Fluvio__ deployment. 

{{< code >}}
fluvio custom-spu list [OPTIONS]

OPTIONS:
    -c, --sc <host:port>       Address of Streaming Controller
    -P, --profile <profile>    Profile name
    -O, --output <type>        Output [possible values: table, yaml, json]
{{< /code >}}

The options are defined as follows:

* <strong>{{< pre >}}--sc &lt;host:port&gt;{{< /pre >}}</strong>:
See [Register Custom-SPU](#register-custom-spu)

* <strong>{{< pre >}}--profile &lt;profile&gt;{{< /pre >}}</strong>:
See [Register Custom-SPU](#register-custom-spu)

* <strong>{{< pre >}}--output &lt;type&gt;{{< /pre >}}</strong>:
is the format to be used to display the Custom-SPUs. The output is an optional field and it defaults to __table__ format. Alternative formats are: __yaml__ and __json__.

### List Custom-SPUs Example

... Fluvio


{{< links "Related Topics" >}}
* [Produce CLI]({{< relref "produce" >}})
* [Consume CLI]({{< relref "consume" >}})
* [SPUs CLI]({{< relref "spus" >}})
* [SPU-Groups CLI]({{< relref "spu-groups" >}})
* [Topics CLI]({{< relref "topics" >}})
{{< /links >}}