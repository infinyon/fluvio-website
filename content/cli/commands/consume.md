---
title: Consume
weight: 20
---

## `fluvio consume`
The `fluvio consume` command is a way to read the contents of records in a Fluvio topic
from a command-line environment.

The `consume` command will only read from one of those partitions, defaulting to partition `0`.

{{% inline-embed file="embeds/cli/help/fluvio-consume.md" %}}

The following `fluvio consume` examples come after the [`fluvio produce` examples]({{< ref "/cli/commands/produce" >}}).

## Examples
### Consume all records

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

### Consume key/value records

If we want to see both the keys _and_ values of the records in the topic, you can use
the `--key-value` flag:

%copy first-line%
```bash
$ fluvio consume my-topic -dB --key-value
[null] This is my first record ever
[null] This is my second record ever
[alice] Alice In Wonderland
[batman] Bruce Wayne
[santa] Santa Claus
```

Records that were not given a key are printed with `[null]`.

### Consume using a SmartModule

Fluvio SmartModules are WASM modules that can edit the contents of a stream
inline, before the records of that stream are delivered to a consumer. One way to use
SmartModules is to supply the WASM module with the `--smartmodule-path` flag to
the `fluvio consume` command.

The simplest SmartModule is the [filter example], which
filters records from the stream based on whether they contain the letter `a`
or not. You can find the full example code [in our GitHub repo] and compile
it to test out yourself.

[filter example]: {{< ref "/smartmodules/transform/filter" >}}
[in our GitHub repo]:https://github.com/infinyon/fluvio/blob/d63e3e2569e4d64a098e5c2189ac68e6e9cd2670/crates/fluvio-smartmodule/examples/filter_json

Once you have compiled your SmartModule Filter and have a `.wasm` file for it, you
can apply it by sending the binary to the cluster when you start your CLI consumer:

%copy first-line%
```bash
$ fluvio consume my-topic -B --smartmodule-path="fluvio_wasm_filter.wasm"
```

Alternatively, to avoid sending the SmartModule binary to the cluster with each
`fluvio consume` session, you can have the cluster store it for you:

%copy first-line%
```bash
$ fluvio smartmodule create --wasm-file="fluvio_wasm_filter.wasm" my_filter
```

Then you can apply the SmartModule by name:

%copy first-line%
```bash
$ fluvio consume my-topic -B --smartmodule="my_filter"
```

### Consume from a topic with multiple partitions

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

### Consume from all partitions

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


### Print consumed records with custom formatting

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
