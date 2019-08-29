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

The fluvio CLI is user friendly and hierarchical. Use {{< pre >}}-h{{< /pre >}} or {{< pre >}}--help{{< /pre >}} at any level to list all available options or subcommands. At top level, you can run __fluvio__ with no arguments:

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

This CLI is organized in sections, where the first subcommand is the module:

* spu
* spu-group
* custom-spu
* topic

There are two exceptions to this rule:

* consume/produce
* advanced

"Consume/Produce" subcommands are frequently utilized operations hence kept at top level.  
"Advanced" is an aggregate of system-wide operations and don't belong to any particular module.

### Operations

The top level subcommands are followed by the an operation such as create, list or delete.

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

### Options

Operations are followed by parameter options

{{< cli yaml >}}
$topic create -h
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