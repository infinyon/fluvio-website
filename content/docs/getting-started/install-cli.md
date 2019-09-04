---
title: Install Fluvio Command Line Interface (CLI)
menu: Install CLI
weight: 50
---

### Downloading CLI

Download binaries for your environment from [github release](https://github.com/infinyon/fluvio/releases).  
Copy binary to your bin path and make it executable.  
Currently we only support macOS and Linux.

### Test CLI

Run follow snippets to check make sure you have Fluvio CLI
{{< cli yaml>}}
$ fluvio --help
Fluvio Command Line Interface

fluvio.x86_64-apple-darwin <SUBCOMMAND>

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

#### Next Steps
* [Example]({{< relref "example" >}})
