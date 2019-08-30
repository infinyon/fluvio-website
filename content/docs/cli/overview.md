---
title: Fluvio CLI
menu: Overview
weight: 10
---

Fluvio Command Line Interface (CLI) is a powerful tool to deploy and manage your Fluvio services. One command line tool is all you need to control multiple Fluvio deployments and __automate__ them through scripts.  

Fluvio CLI has built-in Kafka compatibility which allows you to chain  __Fluvio__ and __Kafka__ services from a unified command-line interface.

## Download and Install

Fluvio command-line tool is available in __Mac__ and __Linux__ distributions and it can be used from the command prompt in your favorite terminal program.  

Binaries are available for download at:

* [Mac binary](http://github.com/infinyon)
* [Linux binary](http://github.com/infinyon)


## CLI Overview

Fluvio CLI is user friendly and hierarchical. The syntax follows a well-defined pattern: __fluvio__, succeeded by __module__, __operation__ and a series of _options_ and _flags_. There are a couple of exceptions to this rule which are describe later on.


{{< text >}}
<strong>fluvio module operation</strong> [FLAGS] [OPTIONS]
{{< /text >}}

Most options and flags are optional but there are some that are mandatory. Mandatory options are shown in the CLI usage line right below the title. For example:

{{< text >}}
Create a topic

<strong>fluvio topic create</strong> [FLAGS] [OPTIONS] --partitions &lt;integer&gt; --replication &lt;integer&gt; --topic &lt;string&gt;
{{< /text >}}

In create topic command {{< pre >}}--topic{{< /pre >}}, {{< pre >}}--partitions{{< /pre >}}, and {{< pre >}}--replication{{< /pre >}}, are mandatory.

### Fluvio

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

Top level fluvio CLI is organized by modules (aside from a couple of exceptions):

* spu
* spu-group
* custom-spu
* topic

The exceptions are as follows:

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

The modules are followed by options and flags. Options are composed of a unique attribute, such as: {{< pre >}}-t, --topic{{< /pre >}} and are followed modifiers. Flags are just attributes without any value.

Mandatory options are shown in the syntax definition. All other flags and options are optional.

{{< cli yaml >}}
$ fluvio topic create -h
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

A small subset of the options, {{< pre >}}--kf, --sc,{{< /pre >}} and {{< pre >}}--profile{{< /pre >}}, are applied to every command. The purpose of these options is to help __fluvio__ identify the server where to send the command.

### Profiles

Fluvio CLI can to simultaneously manage multiple Fluvio and Kafka deployment. Switching from one deployment to another is simple, just provision a different __profile__. 

The __profile__ is a .toml configuration file that stores location of the servers. The syntax is as follows:

{{< code toml >}}
version = <profiles-version>

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

* __version__ should match the cli version, otherwise the profile is rejected.
* __hostname/ip__ is the location of the serve, it may be a domain name or an IP address.
* __port__ is the listening port of the server.

{{< caution >}}
While it is possible to configure all three servers, it is not a useful configuration. Servers with lower priority are shadowed by the servers with higher priority. The lookup order is: SC => SPU => KF
{{< /caution >}}

The most common configuration is _one server server per profile_.

{{< code toml >}}
version = "1.0"

[sc]
host = "0.0.0.0"
port = 9003
{{< /code >}}

#### Default Profile

Fluvio CLI may be utilized with one __default__ profile and an unlimited number of __user-defined__ profiles. The CLI will look-up the __default__ profile if both conditions are met:

* no server ({{< pre >}}--sc, --spu, --kf{{< /pre >}}) is provisioned,
* no user-defined profile ({{< pre >}}--profile{{< /pre >}}) is been provided.

Conversely, if both parameters are provisioned the server configuration takes precedence.  

The CLI searches for the __default.toml__ profile file as follows:

* if $FLUVIO_HOME environment variable is, look-up:
    {{< text >}}
    $FLUVIO_HOME/.fluvio/profiles/default.toml
    {{< /text >}}
* if no environment variable is set, look-up:
    {{< text >}}
    $HOME/.fluvio/profiles/default.toml 
    {{< /text >}}

Note, the directory hierarchy  __/.fluvio/profiles/__ should be preserved whether $FLUVIO_HOME is provisioned or not.

### Target Servers

