---
title: Fluvio CLI reference
menu: Fluvio CLI reference
toc: true
weight: 50
---

This page is the detailed reference for all the commands in the Fluvio CLI.
Most of the information given here can also be found in the help menu for
Fluvio, `fluvio help`, but here we will also explain when you may want to
use certain commands and show some examples.

Here is a list of all Fluvio commands that are available by default:

| Command                           | Description                                                        |
|-----------------------------------|--------------------------------------------------------------------|
| [fluvio topic create]             | Create a topic with the given name                                 |
| [fluvio topic list]               | List all of the Topics in the cluster                              |
| [fluvio topic describe]           | Print detailed information about a Topic                           |
| [fluvio topic delete]             | Delete a topic with the given name                                 |
| [fluvio partition list]           | List all of the Partitions in the cluster                          |
| [fluvio produce]                  | Write messages to a topic/partition                                |
| [fluvio consume]                  | Read messages from a topic/partition                               |
| [fluvio profile current]          | Print the name of the current context                              |
| [fluvio profile delete]           | Delete the named profile                                           |
| [fluvio profile delete-cluster]   | Delete the named cluster                                           |
| [fluvio profile switch]           | Switch to the named profile                                        |
| [fluvio profile view]             | Display the entire configuration                                   |
| [fluvio cluster check]            | Check that all the requirements for cluster installation are met   |
| [fluvio cluster start]            | Install a Fluvio cluster, locally or on Minikube                   |
| [fluvio cluster delete]           | Uninstall a Fluvio cluster from the local machine or Minikube      |
| [fluvio cluster releases list]    | Show a list of Fluvio release versions                             |
| [fluvio cluster spu register]     | Register a new custom SPU with the cluster                         |
| [fluvio cluster spu unregister]   | Unregister a custom SPU from the cluster                           |
| [fluvio cluster spu list]         | List all SPUs known by this cluster (managed AND custom)           |
| [fluvio cluster spg create]       | Create a new managed SPU Group                                     |
| [fluvio cluster spg delete]       | Delete a managed SPU Group                                         |
| [fluvio cluster spg list]         | List all SPU Groups                                                |
| [fluvio cluster run sc]           | Run a new Streaming Controller (SC)                                |
| [fluvio cluster run spu]          | Run a new Streaming Processing Unit (SPU)                          |
| [fluvio install]                  | Install Fluvio plugins                                             |
| [fluvio update]                   | Update the Fluvio CLI                                              |
| [fluvio version]                  | Print Fluvio version information                                   |

[fluvio topic create]: #fluvio-topic-create
[fluvio topic list]: #fluvio-topic-list
[fluvio topic describe]: #fluvio-topic-describe
[fluvio topic delete]: #fluvio-topic-delete
[fluvio partition list]: #fluvio-partition-list
[fluvio produce]: #fluvio-produce
[fluvio consume]: #fluvio-consume
[fluvio profile current]: #fluvio-profile-current
[fluvio profile delete]: #fluvio-profile-delete
[fluvio profile delete-cluster]: #fluvio-profile-delete-cluster
[fluvio profile switch]: #fluvio-profile-switch
[fluvio profile view]: #fluvio-profile-view
[fluvio cluster check]: #fluvio-cluster-check
[fluvio cluster start]: #fluvio-cluster-start
[fluvio cluster delete]: #fluvio-cluster-delete
[fluvio cluster releases list]: #fluvio-cluster-releases-list
[fluvio cluster spu register]: #fluvio-cluster-spu-register
[fluvio cluster spu list]: #fluvio-cluster-spu-list
[fluvio cluster spu unregister]: #fluvio-cluster-spu-unregister
[fluvio cluster spg create]: #fluvio-cluster-spg-create
[fluvio cluster spg list]: #fluvio-cluster-spg-list
[fluvio cluster spg delete]: #fluvio-cluster-spg-delete
[fluvio cluster run sc]: #fluvio-cluster-run-sc
[fluvio cluster run spu]: #fluvio-cluster-run-spu
[fluvio install]: #fluvio-install
[fluvio update]: #fluvio-update
[fluvio version]: #fluvio-version

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

ARGS:
    <name>    The name of the Topic to create
```

Example usage:

```
$ fluvio topic create greeting
topic "greeting" created
```

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

```
$ fluvio topic list
 NAME      TYPE      PARTITIONS  REPLICAS  IGNORE-RACK  STATUS                   REASON 
 greeting  computed      1          1                   resolution::provisioned   
```

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

```
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

```
$ fluvio topic delete greeting
topic "greeting" deleted
```

## `fluvio partition list`

Prints basic information about each partition in the cluster, such as
which topic it belongs to, which SPU is leading the partition, and the
various offsets the partition keeps track of.

```
List all of the Partitions in this cluster

fluvio partition list [OPTIONS]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -O, --output <type>    Output [default: table]  [possible values: table,
                           yaml, json]
```

Example usage:

```
$ fluvio partition list
 TOPIC     PARTITION  LEADER  REPLICAS  RESOLUTION  HW  LEO  LSR  FOLLOWER OFFSETS 
 greeting  0          5001    []        Online      1   1    0    [] 
```

## `fluvio produce`

The `fluvio produce` command is a way to send messages to a particular topic and partition.
This can be useful for testing your applications by manually sending specific messages.

```
fluvio-produce 0.4.0
Write messages to a topic/partition

By default, this reads a single line from stdin to use as the message. If you
run this with no file options, the command will hang until you type a line in
the terminal. Alternatively, you can pipe a message into the command like this:

$ echo "Hello, world" | fluvio produce greetings

USAGE:
    fluvio produce [FLAGS] [OPTIONS] <topic>

FLAGS:
    -C, --continuous    Send messages in an infinite loop
    -h, --help          Prints help information

OPTIONS:
    -p, --partition <integer>
            The ID of the Partition to produce to [default: 0]

    -l, --record-per-line <filename>
            Send each line of the file as its own Record

    -r, --record-file <filename>...     Send an entire file as a single Record

ARGS:
    <topic>    The name of the Topic to produce to
```

By default, the `fluvio produce` command will read a single line from `stdin` and send it
as the payload of the message. However, you can also specify files to read messages from.
With the `--record-per-line` flag, you can send many messages from a single file, or with
the `--record-file` flag you can send the entire contents of a file (including newlines)
as a single record.

Example usage:

```
$ cat records.txt
{"user":"Bob","message":"Hello, Alice!"}
{"user":"Alice","message":"Hello, Bob!"}

$ fluvio produce my-topic --record-per-line records.txt
{"user":"Bob","message":"Hello, Alice!"}
{"user":"Alice","message":"Hello, Bob!"}
Ok!
```

You can then confirm that the records were sent by reading them back with `fluvio consume`:

```
$ fluvio consume my-topic -B -d
{"user":"Bob","message":"Hello, Alice!"}
{"user":"Alice","message":"Hello, Bob!"}
```

## `fluvio consume`

The `fluvio consume` command is a way to read the contents of messages in a Fluvio topic
from a command-line environment. This can be useful if you are developing an application
with Fluvio and want real-time visibility into what is streaming through your topics.
It can also be handy for writing shell scripts that can read and react to messages in a
topic.

If your topic has more than one partition, the `consume` command will only read from one
of those partitions, defaulting to the first one (index zero). You can specify which
partition you want to read messages from using the `-p` option.

Arguments:

```
fluvio-consume 0.4.0
Read messages from a topic/partition

USAGE:
    fluvio consume [FLAGS] [OPTIONS] <string>

FLAGS:
    -B, --from-beginning        Start reading from beginning
    -d, --disable-continuous    disable continuous processing of messages
    -s, --suppress-unknown      Suppress items items that have an unknown output
                                type
    -h, --help                  Prints help information

OPTIONS:
    -p, --partition <integer>    Partition id [default: 0]
    -o, --offset <integer>
            Offsets can be positive or negative. (Syntax for negative offset:
            --offset="-1")
    -b, --maxbytes <integer>     Maximum number of bytes to be retrieved
    -O, --output <type>
            Output [default: dynamic]  [possible values: dynamic, text, binary,
            json, raw]

ARGS:
    <string>    Topic name
```

Example usage:

Let's say you want to read all the messages that have ever been produced for a particular
topic and partition, then stop when they have all been consumed. If our topic is named
"my-topic", and we want to consume from partition 1, we can run:

```
fluvio consume my-topic -B -d -p 1 
```

If you would like the consumer to continue running and print new messages as they arrive,
simply remove the `-d` flag:

```
fluvio consume my-topic -B -p 1
```

## `fluvio profile current`

Prints out the name of the active Fluvio profile. Profiles are used to communicate with
different Fluvio clusters. For example, let's say you have an application that uses
Fluvio Cloud, but you have a local Fluvio cluster installed that you use for development.
Each of those clusters would be represented by a profile, and whichever profile is "current"
is the one that the Fluvio CLI will interact with.

For example, if your Fluvio Cloud profile is called "cloud", and your local profile is called
"local", then if your current profile is "local", then `fluvio topic create greeting` will
create a "greeting" topic on your local cluster.

```
fluvio-profile-current 0.4.0
Print the name of the current context

USAGE:
fluvio profile current

FLAGS:
-h, --help    Prints help information
```

Example usage:

```
$ fluvio profile current
local
```

## `fluvio profile delete`

Deletes a profile from your Fluvio configuration (`~/.fluvio/config`). This will
not delete a cluster (for that, see [fluvio cluster delete]), but it will cause
the Fluvio CLI to no longer be able to interact with that cluster.

```
fluvio-profile-delete 0.4.0
Delete the named profile

USAGE:
    fluvio profile delete <profile name>

FLAGS:
    -h, --help    Prints help information

ARGS:
    <profile name>
```

Example usage:

```
$ fluvio profile delete local
profile local deleted
```

If you try to delete the current profile, you will get a warning, because other CLI
commands depend on having a current profile being set.

```
$ fluvio profile delete local
profile local deleted
warning: this removed your current profile, use 'fluvio profile switch' to select a different one
```

## `fluvio profile delete-cluster`

Deletes cluster connection information from your Fluvio configuration. This will not delete
a cluster itself (for that see [fluvio cluster delete]), but it will cause the Fluvio CLI to
no longer be able to connect to that cluster.

```
fluvio-profile-delete-cluster 0.4.0
Delete the named cluster

USAGE:
    fluvio profile delete-cluster [FLAGS] <cluster name>

FLAGS:
    -f, --force    Deletes a cluster even if its active
    -h, --help     Prints help information

ARGS:
    <cluster name>    The name of a cluster connection to delete
```

Example usage:

```
$ fluvio profile delete-cluster local
Cluster local deleted
```

## `fluvio profile switch`

This switches the "current" profile. After switching current profiles, all
subsequent Fluvio CLI commands that interact with a cluster will target the
cluster of the new "current" profile.

```
fluvio-profile-switch 0.4.0
Switch to the named profile

USAGE:
    fluvio profile switch <profile name>

FLAGS:
    -h, --help    Prints help information

ARGS:
    <profile name>    
```

Example usage:

```
$ fluvio profile switch fluvio-cloud
```

## `fluvio profile view`

This prints the entire Fluvio configuration (from `~/.fluvio/config`).

```
fluvio-profile-view 0.4.0
Display the entire Fluvio configuration

USAGE:
    fluvio profile view

FLAGS:
    -h, --help    Prints help information
```

Example usage:

```
$ fluvio profile view
Config {
    version: "2.0",
    current_profile: Some(
        "minikube",
    ),
    profile: {
        "minikube": Profile {
            cluster: "minikube",
            topic: None,
            partition: None,
        },
    },
    cluster: {
        "minikube": FluvioConfig {
            addr: "10.111.134.218:9003",
            tls: Disabled,
        },
    },
    client_id: None,
}
```

## `fluvio cluster check`

The cluster commands are used to install your own Fluvio cluster. This command
is used to check whether you have all the required dependencies set up on your
system and whether they are running correctly.

```
fluvio-cluster-check 0.4.2
Check that all requirements for cluster startup are met

USAGE:
    fluvio cluster check

FLAGS:
    -h, --help    Prints help information
```

Example usage:

```
$ fluvio cluster check
Running pre-startup checks...
✅ ok: Kubernetes config is loadable
✅ ok: Supported kubernetes version is installed
✅ ok: Supported helm version is installed
✅ ok: Fluvio system charts are installed
✅ ok: Can create service
✅ ok: Can create customresourcedefinitions
✅ ok: Can create secret
✅ ok: Load balancer is up
All checks passed!
You may proceed with cluster startup
next: run `fluvio cluster start`
```

## `fluvio cluster start`

This command is used to start your own Fluvio cluster, with all the
machinery needed to receive, process, and serve streaming messages.

There are two main variations of this command. The default variation
is invoked simply by `fluvio cluster start`. This will install Fluvio
to a Kubernetes cluster, typically Minikube. The other variation is
`fluvio cluster start --local`, which will start the cluster components
as plain processes on your local machine.

```
fluvio-cluster-start 0.4.2
Start a Fluvio cluster, locally or on Minikube

USAGE:
    fluvio cluster start [FLAGS] [OPTIONS]

FLAGS:
        --develop                  use local image
        --skip-profile-creation    
        --sys                      installing sys
        --local                    install local spu/sc(custom)
        --tls                      Whether to use TLS
        --skip-checks
            Whether to skip pre-install checks, defaults to false

        --setup
            Tries to setup necessary environment for cluster startup

    -h, --help                     Prints help information

OPTIONS:
        --chart-version <chart-version>
            k8: use specific chart version [default: 0.6.0-beta.1]

        --image-version <image-version>
            k8: use specific image version

        --registry <registry>
            k8: use custom docker registry

        --namespace <namespace>
            k8 [default: default]

        --group-name <group-name>
            k8 [default: main]

        --install-name <install-name>
            helm chart installation name [default: fluvio]

        --chart-location <chart-location>
            Local path to a helm chart to install

        --cloud <cloud>
            k8 [default: minikube]

        --spu <spu>
            number of SPU [default: 1]

        --rust-log <rust-log>
            RUST_LOG options

        --log-dir <log-dir>
            log dir [default: /usr/local/var/log/fluvio]

        --domain <domain>                                        TLS: domain
        --ca-cert <ca-cert>                                      TLS: ca cert
        --client-cert <client-cert>
            TLS: client cert

        --client-key <client-key>                                TLS: client key
        --server-cert <server-cert>
            TLS: path to server certificate

        --server-key <server-key>
            TLS: path to server private key

        --authorization-config-map <authorization-config-map>    
```

Example usage:

To start a cluster on Minikube:

```
$ fluvio cluster start
"fluvio" has been added to your repositories
Hang tight while we grab the latest from your chart repositories...
...Successfully got an update from the "jaegertracing" chart repository
...Successfully got an update from the "prometheus-community" chart repository
...Successfully got an update from the "fluvio" chart repository
Update Complete. ⎈Happy Helming!⎈
NAME: fluvio
LAST DEPLOYED: Mon Dec 28 13:12:27 2020
NAMESPACE: default
STATUS: deployed
REVISION: 1
TEST SUITE: None
✅ ok: Kubernetes config is loadable
✅ ok: Supported helm version is installed
✅ ok: Fluvio system charts are installed
✅ ok: Previous fluvio installation not found
✅ ok: Load balancer is up
```

To start a cluster locally (as processes on your machine):

```
$ fluvio cluster start --local
Performing pre-flight checks
✅ ok: Supported helm version is installed
✅ ok: Supported kubernetes version is installed
✅ ok: Kubernetes config is loadable
✅ ok: Fluvio system charts are installed
```

## `fluvio cluster delete`

Deletes a Fluvio cluster and all data associated with it. Be careful, this
cannot be undone.

```
fluvio-cluster-delete 0.4.2
Delete a Fluvio cluster from the local machine or Minikube

USAGE:
    fluvio cluster delete [FLAGS] [OPTIONS]

FLAGS:
        --no-wait    don't wait for clean up
        --local      Remove local spu/sc(custom) fluvio installation
        --sys        Remove fluvio system chart
    -h, --help       Prints help information

OPTIONS:
        --namespace <namespace>     [default: default]
        --name <name>               [default: fluvio]
```

Example usage:

To uninstall Fluvio from Kubernetes (e.g. Minikube):

```
$ fluvio cluster delete
```

To uninstall Fluvio from your local machine:

```
$ fluvio cluster delete --local
```

## `fluvio cluster releases list`

Prints all the published versions of Fluvio that are candidates to
install on Kubernetes with `fluvio cluster start`.

The Fluvio cluster components are distributed as Helm charts which allow
them to be installed easily on Kubernetes. This prints all the releases
that are available.

```
fluvio-cluster-releases-list 0.4.2
Show a list of Fluvio release versions

USAGE:
    fluvio cluster releases list

FLAGS:
    -h, --help    Prints help information
```

Example usage:

```
$ fluvio cluster releases list
VERSION
0.6.0-latest
0.6.0-beta.1-latest
0.6.0-beta.1
0.6.0-alpha.8-latest
0.6.0-alpha.7-latest
...
```

## `fluvio cluster spu register`

TODO

## `fluvio cluster spu list`

TODO

## `fluvio cluster spu unregister`

TODO

## `fluvio cluster spg create`

TODO

## `fluvio cluster spg list`

TODO

## `fluvio cluster spg delete`

TODO

## `fluvio cluster run sc`

Directly invokes a Streaming Controller (SC) from the command line. This is a low-level
command and is not typically what you want to be using. If you want to run Fluvio
locally, check out the [fluvio cluster start] command and read about the `--local`
flag.

A Streaming Controller hosts a public server for interacting with clients, and a
private server for interacting with SPUs and other cluster components. Most of
the CLI options for the SC involve configuring these servers. See the
[SC Architecture] for more details.

[SC Architecture]: /docs/architecture/sc/

```
fluvio-cluster-run-sc 0.2.0
Run a new Streaming Controller (SC)

USAGE:
    fluvio cluster run sc [FLAGS] [OPTIONS]

FLAGS:
        --tls                   enable tls
        --enable-client-cert    TLS: enable client cert
    -h, --help                  Prints help information

OPTIONS:
        --bind-public <bind-public>
            Address for external service

        --bind-private <bind-private>
            Address for internal service

    -n, --namespace <namespace>                               
        --server-cert <server-cert>
            TLS: path to server certificate

        --server-key <server-key>
            TLS: path to server private key

        --ca-cert <ca-cert>
            TLS: path to ca cert, required when client cert is enabled

        --bind-non-tls-public <bind-non-tls-public>
            TLS: address of non tls public service, required

        --authorization-scopes <authorization scopes path>
             [env: X509_AUTH_SCOPES=]

        --authorization-policy <authorization policy path>
             [env: AUTH_POLICY=]
```

## `fluvio cluster run spu`

Directly invokes a Streaming Processing Unit (SPU) from the command line. This is a
low-level command and is not typically what you want to be using. If you want to run
Fluvio locally, check out the [fluvio cluster start] command and read about the `--local`
flag.

A SPU has a public server which is used to stream data to and from Fluvio Clients, and
a private server which is used to communicate with an SC. Most of the CLI options for
the SPU involve configuring these servers. See the [SPU Architecture] for more details.

[SPU Architecture]: /docs/architecture/spu/

```
fluvio-cluster-run-spu 0.2.0
Run a new Streaming Processing Unit (SPU)

USAGE:
    fluvio cluster run spu [FLAGS] [OPTIONS]

FLAGS:
        --tls                   enable tls
        --enable-client-cert    TLS: enable client cert
    -h, --help                  Prints help information

OPTIONS:
    -i, --id <integer>                                 SPU unique identifier
    -p, --public-server <host:port>
            Spu server for external communication

    -v, --private-server <host:port>
            Spu server for internal cluster communication

        --sc-addr <host:port>
            Address of the SC Server [env: FLV_SC_PRIVATE_HOST=]

        --log-base-dir <dir>                            [env: FLV_LOG_BASE_DIR=]
        --log-size <log size>                           [env: FLV_LOG_SIZE=]
        --index-max-bytes <integer>
             [env: FLV_LOG_INDEX_MAX_BYTES=]

        --index-max-interval-bytes <integer>
             [env: FLV_LOG_INDEX_MAX_INTERVAL_BYTES=]

        --peer-max-bytes <integer>
            max bytes to transfer between leader and follower [env:
            FLV_PEER_MAX_BYTES=]  [default: 1000000]
        --server-cert <server-cert>
            TLS: path to server certificate

        --server-key <server-key>
            TLS: path to server private key

        --ca-cert <ca-cert>
            TLS: path to ca cert, required when client cert is enabled

        --bind-non-tls-public <bind-non-tls-public>
            TLS: address of non tls public service, required
```

## `fluvio install`

TODO

## `fluvio update`

TODO

## `fluvio version`

TODO

#### Next Steps
----------------

*
