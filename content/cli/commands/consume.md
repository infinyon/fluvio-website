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
        --suppress-unknown      Suppress items items that have an unknown output
                                type
    -h, --help                  Prints help information

OPTIONS:
    -p, --partition <integer>            Partition id [default: 0]
    -o, --offset <integer>
            Offsets can be positive or negative. (Syntax for negative offset:
            --offset="-1")
    -b, --maxbytes <integer>             Maximum number of bytes to be retrieved
    -O, --output <type>
            Output [default: dynamic]  [possible values: dynamic, text, binary,
            json, raw]
    -s, --smart-stream <smart-stream>    Path to a WASM binary file

ARGS:
    <topic>    Topic name
```

For our consumer examples, we are going to read back the records we sent from the
[produce command examples].

[produce command examples]: {{< ref "/cli/commands/produce" >}}

## Example 1: Consume all records

When consuming, we need to specify a starting offset from which to begin reading.
We can use the `--from-beginning` (`-B`) flag in order to read everything from the very
beginning. Here we'll also use the `--disable-continuous` (`-d`) flag in order to exit
after all the records have been read:

%copy first-line%
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

%copy first-line%
```bash
$ fluvio consume my-topic -B -d --key-value
[null] This is my first record ever
[null] This is my second record ever
[alice] Alice In Wonderland
[batman] Bruce Wayne
[santa] Santa Claus
```

Records that were not given a key are printed with `[null]`.

## Example 3: Consume using a SmartStream

Fluvio SmartStreams are WASM modules that can edit the contents of a stream
inline, before the records of that stream are delivered to a consumer. In order
to use SmartStreams, you must supply the WASM module to the `fluvio consume`
command using the `--smart-stream` option.

The simplest SmartStream is the [filter example from the quick-start], which
filters records from the stream based on whether they contain the letter `a`
or not. You can find the full example code [in our GitHub repo] and compile
it to test out yourself.

[filter example from the quick-start]: {{< ref "/docs/smartstreams/quick-start" >}}
[in our GitHub repo]: https://github.com/infinyon/fluvio/tree/master/src/smartstream/examples/filter_json 

Once you have compiled your SmartStream and have a `.wasm` file for it, you
can apply it to the consumer as follows:

%copy first-line%
```bash
$ fluvio consume my-topic -B --smart-stream="fluvio_wasm_filter.wasm"
```

## Example 4: Consume from a topic with multiple partitions

As of today, the Fluvio CLI Consumer can only consume records from a single
partition at a time. When running `fluvio consume topic-name`, the CLI will
read records from partition `0` by default. Let's look at how we can read
records from the different partitions in a topic by using the `--partition (-p)` flag.

Start out by creating a new topic with multiple partitions using [`fluvio topic create`].

[`fluvio topic create`]: {{< ref "/cli/commands/topic#fluvio-topic-create" >}}

%copy first-line%
```bash
$ fluvio topic create consume-multi -p 3
```

Let's create a text file with some records we would like to send. Each line of the
text file will be treated as one record.

```bash
# Put the following records into a text file using your favorite editor
$ cat records.txt
one
two
three
four
five
six
seven
eight
nine
```

Then, produce the test data to the topic.

%copy first-line%
```bash
$ fluvio produce "consume-multi" -f records.txt
```

After producing some data, let's take a look at how the records got distributed
among our partitions using [`fluvio partition list`].

[`fluvio partition list`]: {{< ref "/cli/commands/partition#fluvio-partition-list" >}}

%copy first-line%
```bash
$ fluvio partition list
 TOPIC          PARTITION  LEADER  REPLICAS  RESOLUTION  HW  LEO  LSR  FOLLOWER OFFSETS
 consume-multi  0          5001    []        Online      3   3    0    []
 consume-multi  1          5001    []        Online      3   3    0    []
 consume-multi  2          5001    []        Online      3   3    0    []
```

We can see by the high watermark (HW) and log-end-offset (LEO) that 3 records were
sent to each partition. Let's look at how to consume from each partition.

To consume from a specific partition, use the `--partition (-p)` flag on `fluvio consume`.

%copy first-line%
```bash
$ fluvio consume "consume-multi" -B --partition 0
one
four
seven
```

To consume from partition 1:

%copy first-line%
```bash
$ fluvio consume "consume-multi" -B --partition 0
two
five
eight
```

And from partition 3:

%copy first-line%
```bash
$ fluvio consume "consume-multi" -B --partition 0
three
six
nine
```
