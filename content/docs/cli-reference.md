---
title: Fluvio CLI Reference
menu: CLI Reference
toc: true
weight: 300
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
| [fluvio cluster run sc]           | Run a new Streaming Controller (SC)                                |
| [fluvio cluster run spu]          | Run a new Streaming Processing Unit (SPU)                          |
| [fluvio install]                  | Install Fluvio plugins                                             |
| [fluvio update]                   | Update the Fluvio CLI                                              |
| [fluvio version]                  | Print Fluvio version information                                   |

[fluvio topics]: #fluvio-topic
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
[fluvio cluster run sc]: #fluvio-cluster-run-sc
[fluvio cluster run spu]: #fluvio-cluster-run-spu
[fluvio install]: #fluvio-install
[fluvio update]: #fluvio-update
[fluvio version]: #fluvio-version

## Topics

Commands for topic management.

### `fluvio topic create`

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

### `fluvio topic list`

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

### `fluvio topic describe`

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

### `fluvio topic delete`

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

## Partitions

Commands for partition management.

### `fluvio partition list`

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

## Producer/Consumer

Commands for producer and consumer.

### `fluvio produce`

The `fluvio produce` command is a way to send messages to a particular topic and partition.
This can be useful for testing your applications by manually sending specific messages.

```
fluvio-produce
Write messages to a topic/partition

When no '--files' are provided, the producer will read from 'stdin' and send
each line of input as one record.

If one or more files are given with '--files', each file is sent as one entire
record.

If '--key-separator' or '--json-path' are used, records are sent as key/value
pairs, and the keys are used to determine which partition the records are sent
to.

USAGE:
    fluvio produce [FLAGS] [OPTIONS] <topic>

FLAGS:
    -v, --verbose    Print progress output when sending records
    -h, --help       Prints help information

OPTIONS:
        --key-separator <key-separator>
            Sends key/value records split on the first instance of the separator

        --json-path <json-path>
            Sends key/value JSON records where the key is selected using this
            JSON path
    -f, --files <files>...
            Paths to files to produce to the topic. If absent, producer will
            read stdin

ARGS:
    <topic>    The name of the Topic to produce to
```

#### Example 1: Produce records from stdin

The quickest way to send a record using the producer is to just type your record
into standard input:

```
$ fluvio produce my-topic
Reading one record per line from stdin:
```

As the message says, each line that you type will be sent a new record to the topic.

```
$ fluvio produce my-topic
Reading one record per line from stdin:
This is my first record ever
Ok!
This is my second record ever
Ok!
^C
```

-> In order to stop the producer, we need to press `ctrl-C` (shown above as `^C`)

The `Ok!` was printed by the producer after each record, to let us know the record
was sent successfully.

#### Example 2: Produce key/value records from stdin

Fluvio supports key/value records out-of-the-box. In a key/value record, the key is used
to decide which partition the record is sent to. Let's try sending some simple key/value records:

```
$ fluvio produce my-topic --key-separator=":"
Reading one record per line from stdin:
alice:Alice In Wonderland
Ok!
batman:Bruce Wayne
Ok!
^C
```

So our records are being sent, but how do we know that the producer recognized each key properly?
We can use the `--verbose` (`-v`) flag to tell the producer to print back the keys and values it
recognizes. That way, we can be confident that our records are being sent the way we want them to be.

```
$ fluvio produce my-topic -v --key-separator=":"
Reading one record per line from stdin:
santa:Santa Claus
[santa] Santa Claus
Ok!
^C
```

The producer splits the key from the value and prints it in a `[key] value` format.

#### Example 3: Produce key/value records from JSON files

Sometimes, we may have multiple files that each contain one record we want to send. We
can use the `--files` (`-f`) argument to specify one or more files to send all at once!
Suppose we have the following JSON files:

Suppose we want to send some JSON data as our records. We have two files, `tom.json`
and `jerry.json`, and we want to use the `name` field as the key for these records.

```
$ cat tom.json
{
  "name": "Tom",
  "animal": "Cat"
}

$ cat jerry.json
{
  "name": "Jerry",
  "animal": "Mouse"
}
```

We can use the `--files` (`-f`) argument to select these files, and we can use the
`--json-path` argument to describe which data we should use as the record keys.

To choose the `name` field as our key, we'll use `--json-path="$.name"`:

```
$ fluvio produce my-topic -v --json-path="$.name" -f tom.json jerry.json
[Tom] {
  "name":"Tom",
  "animal":"Cat"
}

Ok!
[Jerry] {
  "name":"Jerry",
  "animal":"Mouse"
}

Ok!
^C
```

Notice that `-f` can take multiple arguments at once! That way, we can read all
the files during a single run of the producer, rather than re-running it for each
file we want to give. This makes the producer efficient, since it keeps the same network
connection open while producing all the files.

### `fluvio consume`

The `fluvio consume` command is a way to read the contents of messages in a Fluvio topic
from a command-line environment. This can be useful if you are developing an application
with Fluvio and want real-time visibility into what is streaming through your topics.
It can also be handy for writing shell scripts that can read and react to messages in a
topic.

If your topic has more than one partition, the `consume` command will only read from one
of those partitions, defaulting to the first one (index zero). You can specify which
partition you want to read messages from using the `-p` option.

```
fluvio-consume
Read messages from a topic/partition

By default, consume operates in "streaming" mode, where the command will remain
active and wait for new messages, printing them as they arrive. You can use the
'-d' flag to exit after consuming all available messages.

Records are printed in "[key]: value" format, where "null" is used for no key.

USAGE:
    fluvio consume [FLAGS] [OPTIONS] <topic>

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
    <topic>    Topic name
```

For our consumer examples, we are going to read back the records we sent from the producer
examples above.

#### Example 1: Consume all records

When consuming, we need to specify a starting offset from which to begin reading.
We can use the `--from-beginning` (`-B`) flag in order to read everything from the very
beginning. Here we'll also use the `--disable-continuous` (`-d`) flag in order to exit
after all the records have been read:

```
$ fluvio consume my-topic -B -d
[null] This is my first record ever
[null] This is my second record ever
[alice] Alice In Wonderland
[batman] Bruce Wayne
[santa] Santa Claus
[Tom] {
  "name":"Tom",
  "animal":"Cat"
}

[Jerry] {
  "name":"Jerry",
  "animal":"Mouse"
}
```

Looking at the output of the consumer, notice that _all_ of the records are actually
key/value records, even the ones that we never defined a key for! Records without keys
are printed with `[null]`, and there is no guarantee about which partition they live on.

#### Example 2: Consume the latest 5 records

Sometimes we do not want to read _all_ of the records from a partition, just some subset
of them. We can use the `--offset` (`-o`) argument to tell the consumer where to begin
reading records from.

To begin reading from the fifth-to-the-last record, we can use offset `-5`, like this: 

```
$ fluvio consume my-topic -d --offset="-5"
[alice] Alice In Wonderland
[batman] Bruce Wayne
[santa] Santa Claus
[Tom] {
  "name":"Tom",
  "animal":"Cat"
}

[Jerry] {
  "name":"Jerry",
  "animal":"Mouse"
}
```

Negative offsets count backward from the end of a partition, and positive offsets count
forward from the beginning of a partition.

## Profiles

Commands for profile management

### `fluvio profile current`

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

### `fluvio profile delete`

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

### `fluvio profile delete-cluster`

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

### `fluvio profile switch`

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

### `fluvio profile view`

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

## Clusters

Commands for cluster management.

### `fluvio cluster check`

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

### `fluvio cluster start`

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

### `fluvio cluster delete`

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

### `fluvio cluster releases list`

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

### `fluvio cluster run sc`

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

### `fluvio cluster run spu`

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

## Maintenance

Maintenance commands, cluster installation, upgrading and versioning.

### `fluvio install`

Some Fluvio CLI commands are distributed as separate executables that we call
"extensions". This command installs and updates extensions by name, placing
them in the `~/.fluvio/extensions/` directory.

```
fluvio-install 0.4.0
Install Fluvio plugins

The Fluvio CLI considers any executable with the prefix `fluvio-` to be a CLI
plugin. For example, an executable named `fluvio-foo` in your PATH may be
invoked by running `fluvio foo`.

This command allows you to install plugins from Fluvio's package registry.

USAGE:
    fluvio install <package>

FLAGS:
    -h, --help    Prints help information

OPTIONS:


ARGS:
    <package>    The ID of a package to install, e.g. "fluvio/fluvio-cloud"
```

Example usage:

```
$ fluvio install fluvio/fluvio-cloud
🎣 Fetching latest version for package: fluvio/fluvio-cloud...
⏳ Downloading package with latest version: fluvio/fluvio-cloud:0.1.0...
🔑 Downloaded and verified package file
```

### `fluvio update`

This command performs a self-update for the Fluvio CLI. It takes no arguments,
and just simply downloads the latest version of `fluvio` and overwrites itself.

Example usage:

```
$ fluvio update
🎣 Fetching latest version for fluvio/fluvio...
⏳ Downloading Fluvio CLI with latest version: fluvio/fluvio:0.6.0-beta.1...
🔑 Downloaded and verified package file
✅ Successfully installed ~/.fluvio/bin/fluvio
```

### `fluvio version`

This command prints out the version of the Fluvio CLI that you're running, as
well as some other useful information about how it was built. For example, it
includes the version of the Rust compiler used to build it as well as the exact
git commit of the Fluvio source code that was built. This is useful information
to include if you ever need to file a bug report so that the developers know
exactly what code needs fixing or if a fix is already available.

Example usage:

```
$ fluvio version
Fluvio CLI      : 0.6.0-beta.1
Fluvio Platform : 0.6.0-beta.1
Git Commit      : 68acd75c8d8c1a03ffa7512be5eb71fa6f79caa1
OS Details      : Darwin 19.6.0 x86_64
Rustc Version   : 1.48.0 (7eac88a 2020-11-16)
```
