---
title: Install Fluvio Command Line Interface (CLI)
menu: Install CLI
weight: 9
---

### Download CLI

Fluvio Command Line Interface binaries are available in macOS and Linux versions:

* Download binaries for your environment from [github release](https://github.com/infinyon/fluvio/releases).  
* Copy binary to your bin path and make it executable.

### Test CLI

Run following command to ensure Fluvio CLI is working correctly:

{{< cli yaml>}}
$ fluvio --help
Fluvio Command Line Interface

fluvio <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    consume       Read messages from a topic/partition
    produce       Write messages to a topic/partition
    spu           SPU Operations
    spu-group     SPU Group Operations
    custom-spu    Custom SPU Operations
    topic         Topic operations
    advanced      Advanced operations
    help          Prints this message or the help of the given subcommand(s)
{{< /cli>}}

Congratulation! You have successfully deployed Fluvio.

{{< links "Next Steps" >}}
* [Send your First Message]({{< relref "example" >}})
{{< /links >}}