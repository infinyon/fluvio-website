---
title: Produce
weight: 10
---

The `fluvio produce` command is a way to send messages to a particular topic and partition.
This can be useful for testing your applications by manually sending specific messages.

```
fluvio-produce
Write messages to a topic/partition

When no '--file' is provided, the producer will read from 'stdin' and send each
line of input as one record.

If a file is given with '--file', the file is sent as one entire record.

If '--key-separator' is used, records are sent as key/value pairs, and the keys
are used to determine which partition the records are sent to.

USAGE:
    fluvio produce [FLAGS] [OPTIONS] <topic>

FLAGS:
    -v, --verbose    Print progress output when sending records
    -h, --help       Prints help information

OPTIONS:
        --key-separator <key-separator>
            Sends key/value records split on the first instance of the separator

    -f, --file <file>
            Path to a file to produce to the topic. If absent, producer will
            read stdin

ARGS:
    <topic>    The name of the Topic to produce to
```

## Example 1: Produce records from stdin

The quickest way to send a record using the producer is to just type your record
into standard input:

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

## Example 2: Produce key/value records from stdin

Fluvio supports key/value records out-of-the-box. In a key/value record, the key is used
to decide which partition the record is sent to. Let's try sending some simple key/value records:

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

```bash
$ fluvio produce my-topic -v --key-separator=":"
> santa:Santa Claus
[santa] Santa Claus
Ok!
> ^C
```

The producer splits the key from the value and prints it in a `[key] value` format.
