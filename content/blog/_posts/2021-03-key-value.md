---
title: Key/Value records in Fluvio
author:
    name: "The Fluvio Team"
description: 
date: 2021-03-01
slug: key-value-records
url: /blog/2021/03/key-value-records
twitter-card: summary_large_image
hidden: false
---

This week, we're happy to announce the addition of a Key/Value API for
Fluvio producers and consumers! The ability to define a Key for your records
gives you more control over how your data is distributed and stored within
Fluvio. In this blog, we'll talk more about the guarantees that key/value
records give you, as well as how to use key/value records from Fluvio's
various producers and consumers.

## What are Key/Value records and why use them?

Key/Value records are all about determining which partition each record gets sent to.
The golden rule is: **records with the same key always go to the same partition**.
This is great, because we also know that all records that go to the same partition
will be well-ordered, and will be consumed in the same order they were produced.
Generally, you would pick some property of your data to use as the key, such as an
account ID or a username, so that all records belonging to the same user will be
delivered in order. This also means that records belonging to different users may be
distributed across different partitions, making the system free to spread traffic
out across multiple servers and increase throughput.

## Producing and Consuming simple Key/Value records

In this section, we'll be showing `fluvio` commands for producing and consuming
key/value records. If you want to follow along, make sure you've followed the
[getting started] guide and set up a Fluvio cluster, either locally or with a
[free Fluvio Cloud account].

[getting started]: /docs/getting-started
[free Fluvio Cloud account]: https://cloud.fluvio.io/signup

Once the cluster is set up, create a fresh topic to use for this example:

```bash
$ fluvio topic create bank-transactions
```

#### Producing key/value records

The producer and consumer built into Fluvio's CLI can send and receive key/value
records. Let's look at a quick example of producing data from a text file.

```bash
$ cat transactions.txt
alice=Deposit 100.00
bob=Withdraw 50.00
bob=Withdraw 25.00
```

Here we have a file, `transactions.txt`, with keys and values separated by a `=` and with
one record on each line of the file. We can use the following command to send
each line as a key/value record:

```bash
$ fluvio produce bank-transactions -v --key-separator "=" -f transactions.txt
[alice] Deposit 100.00
[bob] Withdraw 50.00
[bob] Withdraw 25.00
Ok!
```

Let's break down this command:

- `fluvio produce` is how we start up the producer
- `key-value-text` is the name of the topic we want to produce to
- `-v` or (`--verbose`) tells the producer to print each record after it's sent
- `--key-separator "="` tells the producer to split each line on an `=`, using the
left side as the key and the right side as the value
- `-f transactions.txt` tells the producer to read data from the `transactions.txt` file
  
We can tell that the producer recognized the keys correctly because it prints them
back out in square brackets. Next, let's look at how to use a consumer to read back
records that have been stored.

#### Consuming key/value records

Let's get right to it and consume our records:

```bash
$ fluvio consume bank-transactions -B -d
Deposit 100.00
Withdraw 50.00
Withdraw 25.00
```

By default, the consumer does not print the keys of each record. This highlights the
fact that key/value records are the same as regular records, they just happen to have keys.
We can tell the consumer to print the keys that belong to each record with `--key-value`:

```bash
$ fluvio consume bank-transactions -B -d --key-value
[alice] Deposit 100.00
[bob] Withdraw 50.00
[bob] Withdraw 25.00
```

# Summary

Don't forget to join the conversation on
[Discord](https://discordapp.com/invite/bBG2dTz), follow [the project on
github](https://github.com/infinyon/fluvio/watchers) or open an
[issue](https://github.com/infinyon/fluvio/issues) if you find a bug or have a
feature request.
