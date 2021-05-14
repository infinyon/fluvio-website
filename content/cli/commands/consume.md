---
title: Consume
weight: 20
---

The `fluvio consume` command is a way to read the contents of records in a Fluvio topic
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

USAGE:
    fluvio consume [FLAGS] [OPTIONS] <topic>

FLAGS:
    -B, --from-beginning        Start reading from beginning
    -d, --disable-continuous    disable continuous processing of messages
    -k, --key-value             Print records in "[key] value" format, with
                                "[null]" for no key
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

For our consumer examples, we are going to read back the records we sent from the
[produce command examples].

[produce command examples]: /cli/commands/produce

## Example 1: Consume all records

When consuming, we need to specify a starting offset from which to begin reading.
We can use the `--from-beginning` (`-B`) flag in order to read everything from the very
beginning. Here we'll also use the `--disable-continuous` (`-d`) flag in order to exit
after all the records have been read:

```bash
$ fluvio consume my-topic -B -d
This is my first record ever
This is my second record ever
Alice In Wonderland
Bruce Wayne
Santa Claus
```

Notice that all the records are printed by value only: the records with keys have not
had their keys printed! This is the default behavior of the consumer. To see how to print
the keys of key/value records, see the next example!

## Example 2: Consume key/value records

If we want to see both the keys _and_ values of the records in the topic, you can use
the `--key-value` flag:

```bash
$ fluvio consume my-topic -B -d --key-value
[null] This is my first record ever
[null] This is my second record ever
[alice] Alice In Wonderland
[batman] Bruce Wayne
[santa] Santa Claus
```

Records that were not given a key are printed with `[null]`.