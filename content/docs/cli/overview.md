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

The fluvio CLI is user friendly and hierarchical. To view a list of available options at any level use {{< pre >}}-h{{< /pre >}} or {{< pre >}}--help{{< /pre >}}.  At top level, just run __fluvio__ with no arguments:

{{< cli yaml >}}
$ fluvio
Fluvio Command Line Interface

USAGE:
    fluvio <SUBCOMMAND>

FLAGS:
    -h, --help    Prints help information

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

* __spu__
* __spu-group__
* __custom-spu__
* __topic__

There are two exceptions to this rule:

* __consume/produce__
* __advanced__

"Consume/Produce" subcommands are frequently utilized operations and "Advanced" is an aggregate of system-wide operations and don't belong to a particular module.
