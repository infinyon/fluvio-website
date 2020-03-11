---
title: Fluvio CLI
menu: Overview
weight: 10
---

Fluvio Command Line Interface (CLI) is a powerful tool to deploy and manage your Fluvio services. One command line tool is all you need to control multiple Fluvio deployments and __automate__ them through scripts.  

Fluvio CLI has built-in Kafka compatibility which allows you to chain  __Fluvio__ and __Kafka__ services from a unified command-line interface.

## Download and Install

Fluvio command-line tool is available for __Mac__ and __Linux__ distributions and it can be used from the command prompt in your favorite terminal program.  

Binaries are available for download at:

* [Mac binary](https://github.com/infinyon/fluvio/releases)
* [Linux binary](https://github.com/infinyon/fluvio/releases)


## CLI Overview

Fluvio CLI is user friendly and hierarchical. The syntax follows a well-defined pattern: __fluvio__, succeeded by __module__, __operation__ and a series of _options_ and _flags_. There are a couple of exceptions to this rule which are described later on.


{{< cmd >}}
fluvio module operation [FLAGS] [OPTIONS]
{{< /cmd >}}

Most options and flags are optional but there are some that are mandatory. Mandatory options are shown in the CLI usage line. For example, the syntax for __Create a Topic__ is:

{{< cmd >}}
fluvio topic create --partitions <integer> --replication <integer> --topic <string>
{{< /cmd >}}

It shows that {{< pre >}}--topic{{< /pre >}}, {{< pre >}}--partitions{{< /pre >}}, and {{< pre >}}--replication{{< /pre >}}, are mandatory.

### Modules

Command line help is available at any level by appending {{< pre >}}-h{{< /pre >}} or {{< pre >}}--help{{< /pre >}} to the command. At top level, you can run __fluvio__ with without arguments to get a list of available options.

{{< cli yaml >}}
$ fluvio 
Fluvio Command Line Interface

fluvio <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    consume       Read messages from a topic/partition
    produce       Write log records to a topic/partition
    spu           SPU Operations
    spu-group     SPU Group Operations
    custom-spu    Custom SPU Operations
    topic         Topic operations
    advanced      Advanced operations
    help          Prints this message or the help of the given subcommand(s)
{{< /cli >}}

Top level fluvio CLI is organized by modules:

* spu
* spu-group
* custom-spu
* topic

However, there are a few exceptions:

* consume/produce
* advanced

"Consume/Produce" are kept at top level as they frequently used operations and we chose convenience over convention. "Advanced" is an aggregate of system-wide operations that don't belong to any particular module.

### Operations

Each module is followed by a series of operations that describe their capabilities. For example, __topic__ module has the ability to create, list, describe, or delete topics:

{{< cli yaml >}}
$ fluvio topic
Topic operations

fluvio topic <SUBCOMMAND>

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    create      Create a topic
    delete      Delete a topic
    describe    Show details of a topic
    list        Show all topics
    help        Prints this message or the help of the given subcommand(s)
{{< /cli >}}

Other modules, such as __spu__ have different options, hence different capabilities.

### Options / Flags

The modules are followed by options and flags. Options are composed of unique attributes, such as: {{< pre >}}-t, --topic{{< /pre >}}, followed by modifiers. Flags are attributes without value.

Mandatory options are shown in the syntax definition. All other flags and options are optional.

{{< cli yaml >}}
$ fluvio topic create --help
Create a topic

fluvio topic create [FLAGS] [OPTIONS] --partitions <integer> --replication <integer> --topic <string>

FLAGS:
    -i, --ignore-rack-assignment    Ignore racks while computing replica assignment
    -v, --validate-only             Validates configuration, does not provision
    -h, --help                      Prints help information

OPTIONS:
    -t, --topic <string>                    Topic name
    -p, --partitions <integer>              Number of partitions
    -r, --replication <integer>             Replication factor per partition
    -f, --replica-assignment <file.json>    Replica assignment file
    -c, --sc <host:port>                    Address of Streaming Controller
    -k, --kf <host:port>                    Address of Kafka Controller
    -P, --profile <profile>                 Profile name
{{< /cli >}}

A small subset of the options, {{< pre >}}--kf, --sc,{{< /pre >}} and {{< pre >}}--profile{{< /pre >}}, are applied to every command. The purpose of these options is to help the CLI identify the location of the services where to send the command.

### Target Service

The Fluvio CLI is an independent binary that generates commands and sends them to a __target service__. Fluvio CLI can simultaneously manage multiple Fluvio and Kafka deployments.  

The __target service__  be specified through command line or profiles. The command line has higher precedence than profiles.

### Profiles


The __profiles__ makes managing multiple deployments simple. A __profile__ is a .toml configuration file that stores the location of the services. The syntax is as follows:

{{< code toml >}}
version = <profile-version>

[sc]
host = <hostname/ip>
port = <port>

[spu]
host = <hostname/ip>
port = <port>

[kf]
host = <hostname/ip>
port = <port>
{{< /code >}}

The parameters are as follows:

* __version__ is currently set to "1.0".
* __hostname/ip__ is the location of the service, it may be a domain name or an IP address.
* __port__ is the listening port of the service.

{{< caution >}}
While it is possible to configure all three services, it is not a useful configuration. Services with lower priority are shadowed by the services with higher priority. The lookup order is: SC => SPU => KF
{{< /caution >}}

The most common configuration is _one service per profile_.

{{< code toml >}}
version = "1.0"

[sc]
host = "sc.fluvio.dev.acme.com"
port = 9003
{{< /code >}}

#### Default Profile

Fluvio CLI has one __default__ profile and an unlimited number of __user-defined__ profiles. The __default__ profile has the lowest precedence and it is looked-up in the following order:

* command line parameter __service__ ({{< pre >}}--sc, --spu, --kf{{< /pre >}}),
* command line parameter __user-defined profile__ ({{< pre >}}--profile{{< /pre >}}).
* __default profile__

The CLI searches for the __default.toml__ profile file in the following order: 

* if $FLUVIO_HOME environment variable is set, look-up:
    {{< text >}}
    $FLUVIO_HOME/.fluvio/profiles/default.toml
    {{< /text >}}
* if no environment variable is set, look-up:
    {{< text >}}
    $HOME/.fluvio/profiles/default.toml 
    {{< /text >}}

{{< idea >}}
The directory hierarchy  __/.fluvio/profiles/__ is preserved whether $FLUVIO_HOME is provisioned or not.
{{< /idea >}}


{{< links "Related Topics" >}}
* [Produce CLI]({{< relref "produce" >}})
* [Consume CLI]({{< relref "consume" >}})
* [SPUs CLI]({{< relref "spus" >}})
* [Custom SPU CLI]({{< relref "custom-spus" >}})
* [SPU-Groups CLI]({{< relref "spu-groups" >}})
* [Topics CLI]({{< relref "custom-spus" >}})
{{< /links >}}
