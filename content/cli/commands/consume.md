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

By default, consume operates in "streaming" mode, where the command will remain active and wait for
new messages, printing them as they arrive. You can use the '-d' flag to exit after consuming all
available messages.

USAGE:
    fluvio consume [FLAGS] [OPTIONS] <topic>

FLAGS:
    -A, --all-partitions         Consume records from all partitions
    -d, --disable-continuous     disable continuous processing of messages
        --disable-progressbar    disable the progress bar and wait spinner
    -k, --key-value              Print records in "[key] value" format, with "[null]" for no key
        --suppress-unknown       Suppress items items that have an unknown output type
    -h, --help                   Prints help information

OPTIONS:
    -p, --partition <integer>                Partition id [default: 0]
    -F, --format <format>
            Provide a template string to print records with a custom format. See --help for details

        --table-format <table-format>
            Consume records using the formatting rules defined by TableFormat name

    -B <integer>
            Consume records starting X from the beginning of the log (default: 0)

    -o, --offset <integer>                   The offset of the first record to begin consuming from
    -T, --tail <integer>
            Consume records starting X from the end of the log (default: 10)

        --end-offset <integer>               Consume records until end offset
    -b, --maxbytes <integer>                 Maximum number of bytes to be retrieved
    -O, --output <type>
            Output [possible values: dynamic, text, binary, json, raw, table, full-table]

        --derived-stream <derived-stream>    Name of DerivedStream
        --filter <filter>                    Path to a SmartModule filter wasm file
        --map <map>                          Path to a SmartModule map wasm file
        --filter-map <filter-map>            Path to a SmartModule filter_map wasm file
        --array-map <array-map>              Path to a SmartModule array_map wasm file
        --join <join>                        Path to a SmartModule join wasm filee
        --aggregate <aggregate>              Path to a WASM file for aggregation
        --join-topic <join-topic>            
        --initial <initial>
            (Optional) Path to a file to use as an initial accumulator value with --aggregate

    -e, --extra-params <extra-params>...
            (Optional) Extra input parameters passed to the smartmodule module. They should be
            passed using key=value format Eg. fluvio consume topic-name --filter filter.wasm -e
            foo=bar -e key=value -e one=1

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

## Example 3: Consume using a SmartModule

Fluvio SmartModules are WASM modules that can edit the contents of a stream
inline, before the records of that stream are delivered to a consumer. In order
to use SmartModules, you must supply the WASM module to the `fluvio consume`
command using the SmartModule options: `--filter`, `--map`, `--aggregate`.

The simplest SmartModule is the [filter example], which
filters records from the stream based on whether they contain the letter `a`
or not. You can find the full example code [in our GitHub repo] and compile
it to test out yourself.

[filter example]: {{< ref "/docs/smartmodules/filter" >}}
[in our GitHub repo]: https://github.com/infinyon/fluvio/tree/master/crates/fluvio-smartmodule/examples/filter_json

Once you have compiled your SmartModule Filter and have a `.wasm` file for it, you
can apply it to the consumer as follows:

%copy first-line%
```bash
$ fluvio consume my-topic -B --filter="fluvio_wasm_filter.wasm"
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
$ fluvio consume "consume-multi" -B --partition 1
two
five
eight
```

And from partition 2:

%copy first-line%
```bash
$ fluvio consume "consume-multi" -B --partition 2
three
six
nine
```

## Example 5: Consume from all partitions

At times, it is useful to see all records from all partitions from a single consumer. 
Using the example above:

%copy first-line%
```bash
$ fluvio partition list
 TOPIC          PARTITION  LEADER  REPLICAS  RESOLUTION  HW  LEO  LSR  FOLLOWER OFFSETS
 consume-multi  0          5001    []        Online      3   3    0    []
 consume-multi  1          5001    []        Online      3   3    0    []
 consume-multi  2          5001    []        Online      3   3    0    []
```

Each partition has 3 records. Now let's consume from all partitions:

%copy first-line%
```bash
$ fluvio consume "consume-multi" -B -A           
one
four
seven
two
three
five
six
eight
nine
```

-> Note: There is no order guarantee between partitions.


## Example 6: Print consumed records with custom formatting

Sometimes, the default Consumer printout might not work for your needs. As of Fluvio `0.9.6`
you can now use the `--format` string to describe how the Consumer should print your records!

The format string will replace placeholders such as `{{key}}`, `{{value}}`, `{{partition}}`(added in Fluvio `0.9.9` ), `{{offset}}` and `{{time}}` (added in Fluvio `0.9.25`)
with the actual contents for each record. One possible use for this is formatting each record
as a CSV row:

%copy first-line%
```bash
$ fluvio consume my-topic -B --format="{{time}},{{partition}},{{offset}},{{key}},{{value}}"
2022-05-04T15:35:49.244Z,0,0,null,This is my first record ever
2022-05-04T15:35:49.244Z,0,1,null,This is my second record ever
2022-05-04T15:52:19.963Z,0,2,alice,Alice In Wonderland
2022-05-04T15:52:28.875Z,0,3,batman,Bruce Wayne
2022-05-04T15:53:37.099Z,0,4,santa,Santa Claus
```
