---
title: Produce
weight: 10
---

## `fluvio produce`
The `fluvio produce` command is a way to send records to the leader of a partition.
Produce records by specifying the destination Topic

{{% inline-embed file="embeds/cli/help/fluvio-produce.md" %}}

## Examples
### Produce records from stdin

The quickest way to send a record using the producer is to just type your record
into standard input:

%copy first-line%
```bash
$ fluvio produce my-topic
> This is my first record ever
Ok!
> This is my second record ever
Ok!
> ^C
```

-> In order to stop the producer, we need to press `Ctrl-C` (shown above as `^C`)

As the message says, each line that you type will be sent a new record to the topic.

The `Ok!` was printed by the producer after each record, to let us know the record
was sent successfully.

### Produce key/value records from stdin

Fluvio supports key/value records out-of-the-box. In a key/value record, the key is used
to decide which partition the record is sent to. Let's try sending some simple key/value records:

%copy first-line%
```bash
$ fluvio produce my-topic --key-separator=":"
> alice:Alice In Wonderland
Ok!
> batman:Bruce Wayne
Ok!
> ^C
```

So our records are being sent, but how do we know that the producer recognized each key properly?
We can use the `--verbose` (`-v`) flag to tell the producer to print back the keys and values it
recognizes. That way, we can be confident that our records are being sent the way we want them to be.

%copy first-line%
```bash
$ fluvio produce my-topic -v --key-separator=":"
> santa:Santa Claus
[santa] Santa Claus
Ok!
> ^C
```

The producer splits the key from the value and prints it in a `[key] value` format.

### Produce key/value records to multiple partitions

When producing to a topic with multiple partitions, the producer will send
all records with the same key to the same partition. Let's test this out by making
a multi-partition topic, then sending some key/value records.

First, we'll use [`fluvio topic create`] to create a topic called `multi-keys` with 5 partitions:

[`fluvio topic create`]: {{< ref "/cli/commands/topic#fluvio-topic-create" >}}

%copy first-line%
```bash
$ fluvio topic create multi-keys -p 5
```

Next, let's create a text file with the records we want to send:

```bash
# Put the following records into a text file using your favorite editor
$ cat records.txt
rafael:create account
rafael:validate account
samuel:create account
tabitha:create account
rafael:add item 1234 to cart
tabitha:validate account
samuel:validate account
rafael:add item 2345 to cart
tabitha:add item 9876 to cart
rafael:complete purchase
```

Then, we'll send the key/value records from the file, using the `--key-separator` flag to separate
our keys from our values. In this example, the keys are unique username.

%copy first-line%

```bash
$ fluvio produce multi-keys --key-separator=":" -f records.txt
```

Looking at this sample input, we can see that `rafael` generated 5 events, `samuel`
generated 2 events, and `tabitha` generated 3 events. When we look at the partitions,
we should see the records distributed in groups of 5, 2, and 3. We can use the
[`fluvio partition list`] command to view the distribution of records in our partitions:

[`fluvio partition list`]: {{< ref "/cli/commands/partition#fluvio-partition-list" >}}

%copy first-line%

```bash
$ fluvio partition list
 TOPIC       PARTITION  LEADER  REPLICAS  RESOLUTION  SIZE   HW  LEO  LRS  FOLLOWER OFFSETS 
 multi-keys  0          0       []        Online      0 B    0   0    0    [] 
 multi-keys  1          0       []        Online      0 B    0   0    0    [] 
 multi-keys  2          0       []        Online      157 B  3   3    0    [] 
 multi-keys  3          0       []        Online      220 B  5   5    0    [] 
 multi-keys  4          0       []        Online      119 B  2   2    0    [] 
```

By looking at the high watermark (HW) and log-end-offset (LEO) of our partitions,
we can see how many records have landed in each partition. As we expected, they
were distributed in groups of 5, 3, and 2. Let's dig a little further, we know that
`rafael` was the key used by the group of 5 records, so we should be able to see those
records by using [`fluvio consume`] to consume from partition 3.

[`fluvio consume`]: {{< ref "/cli/commands/consume" >}}

%copy first-line%

```bash
$ fluvio consume multi-keys -B -p3 --key-value
[rafael] create account
[rafael] validate account
[rafael] add item 1234 to cart
[rafael] add item 2345 to cart
[rafael] complete purchase
```

### Producing to multiple partitions using Round-Robin

When we produce to a topic with multiple partitions, records that have no key
are assigned to partitions in a round-robin fashion. This ensures an even load
distribution among the partitions.

To see this in action, let's create a topic with multiple partitions using
[`fluvio topic create`].

%copy first-line%
```bash
$ fluvio topic create multi-no-keys -p 5
```

Let's produce some data to our topic. We'll use the same data from [Example 3],
but this time we won't tell the Producer to interpret our input as key-value records
(we'll do this by omitting the `--key-separator` flag).

[Example 3]: {{< ref "/cli/commands/produce#example-3-produce-keyvalue-records-to-multiple-partitions" >}}

```bash
# Put the following records into a text file using your favorite editor
$ cat records.txt
rafael:create account
rafael:validate account
samuel:create account
tabitha:create account
rafael:add item 1234 to cart
tabitha:validate account
samuel:validate account
rafael:add item 2345 to cart
tabitha:add item 9876 to cart
rafael:complete purchase
```

Next, we'll produce the records into the `multi-no-keys` stream.

%copy first-line%

```bash
$ fluvio produce multi-no-keys -f records.txt
```

Since records with no keys use round-robin partitioning, we should expect to see
the records be evenly distributed among the partitions. This differs from Example 3
in that the records are not grouped by any kind of key. Let's take a look at our
partitions using [`fluvio partition list`].

%copy first-line%
```bash
$ fluvio partition list
 TOPIC          PARTITION  LEADER  REPLICAS  RESOLUTION  SIZE   HW  LEO  LRS  FOLLOWER OFFSETS 
 multi-no-keys  0          0       []        Online      120 B  2   2    0    [] 
 multi-no-keys  1          0       []        Online      121 B  2   2    0    [] 
 multi-no-keys  2          0       []        Online      124 B  2   2    0    [] 
 multi-no-keys  3          0       []        Online      126 B  2   2    0    [] 
 multi-no-keys  4          0       []        Online      127 B  2   2    0    [] 
```

Notice how the high watermark (HW) and log-end-offset (LEO) tell us that there are
exactly 2 records in each partition. Our ten records have been evenly distributed!

### Producing using a compression algorithm (GZIP)

Fluvio support different types of compression algorithms to send records. 
Compression, in general, improves throughput in exchange of some CPU cost to compress/uncompress the data.

Let's try to use `gzip` algorithm in the CLI.

First, we'll use [`fluvio topic create`] to create a topic called `compressed` and other topic called `uncompressed`:

[`fluvio topic create`]: {{< ref "/cli/commands/topic#fluvio-topic-create" >}}

%copy first-line%
```bash
$ fluvio topic create compressed
```

%copy first-line%
```bash
$ fluvio topic create uncompressed
```

Next, let's create a text file called `records.txt` with the following contents:

%copy%
```bash
{"ts":"2020-06-18T10:44:12","started":{"pid":45678}}
{"ts":"2020-06-18T10:44:13","logged_in":{"username":"foo"},"connection":{"addr":"1.2.3.4","port":5678}}
{"ts":"2020-06-18T10:44:15","registered":{"username":"bar","email":"bar@example.com"},"connection":{"addr":"2.3.4.5","port":6789}}
{"ts":"2020-06-18T10:44:16","logged_out":{"username":"foo"},"connection":{"addr":"1.2.3.4","port":5678}}
{"ts":"2020-06-18T10:49:29","logged_in":{"username":"foo"},"connection":{"addr":"1.2.3.4","port":5678}}
{"ts":"2020-06-18T10:50:13","logged_in":{"username":"bar"},"connection":{"addr":"2.3.4.5","port":6789}}
{"ts":"2020-06-18T10:51:13","logged_out":{"username":"bar"},"connection":{"addr":"2.3.4.5","port":6789}}
```

Next, we'll produce the records from that file into the `compressed` stream using the `--compression gzip` option.

%copy first-line%

```bash
$ fluvio produce compressed -f records.txt --compression gzip
```

Let's produce also the same to the `uncompressed` stream using no compression.

%copy first-line%

```bash
$ fluvio produce uncompressed -f records.txt # when no --compression flag is passed, it used `none` as compression algorithm
```

Since records are compressed in the producer before are sent to the SPU, their disk usage on the SPU should be lower than without compression. Let's take a look at the disk usage by the partitions using [`fluvio partition list`].

%copy first-line%
```bash
$ fluvio partition list
 TOPIC          PARTITION  LEADER  REPLICAS  RESOLUTION  SIZE   HW  LEO  LRS  FOLLOWER OFFSETS 
 compressed     0          0       []        Online      328 B  7   7    0    [] 
 uncompressed   0          0       []        Online      821 B  7   7    0    [] 
```

Notice how the SIZE field tell us that the `compressed` topic is using less disk space than the `uncompressed` topic for the same amount of records.

Also note that, [`consuming`] from topics is done at the same way for both compressed and uncompressed data.

[`consuming`]: {{< ref "/cli/commands/consume" >}}