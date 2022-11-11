---
title: "This Week in Fluvio #8"
date: 2021-09-30
weight: 20
---

Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## New Release - Fluvio v0.9.9

### Fluvio Consumer can now stream from all partitions in a topic

As of this release, you can now use the Fluvio Consumer CLI and Rust API to stream
records from _all_ partitions in a topic, rather than just one at a time. To use
this functionality on the CLI, use the new `--all-partitions` flag.

To see this in action, let's produce some records to a topic with 2 partitions.
First, let's create the topic:

%copy first-line%
```bash
$ fluvio topic create fruits -p2
topic "fruits" created
```

Let's produce some records to our topic. By default, these records will be distributed
to the partitions in a round-robin fashion.

%copy first-line%
```bash
$ fluvio produce fruits
> apple
Ok!
> banana
Ok!
> cranberry
Ok!
> date
Ok!
> ^C
```

Before Fluvio `0.9.9`, you would run the consumer and only see the records from one
partition at a time! If you don't specify a partition, it simply chooses partition 0.

%copy first-line%
```bash
$ fluvio consume fruits -B
Consuming records from the beginning of topic 'fruits'
apple
cranberry
```

With the `--all-partitions` flag, you'll see the records from all the partitions!

%copy first-line%
```bash
$ fluvio consume fruits -B --all-partitions
Consuming records from the beginning of topic 'fruits'
apple
cranberry
banana
date
```

Notice that the records don't appear in the exact order that we produced them in!
That's because ordering is only guaranteed for the records in the _same partition_.
In this case, this means that `apple` will always come before `cranberry`, and
`banana` will always come before `date`. To learn more about partitioning,
[check out the docs on key/value records and partitions]({{<ref "/cli/commands/produce#example-3-produce-keyvalue-records-to-multiple-partitions">}}).

### Addition of `{{partition}}` to the Fluvio Consumer `--format` string

With the arrival of the multi-partition consumer, we also need a way determine
which records came from which partitions! You can now include `{{partition}}` in
the consumer's format string to print out the partition of each record. From the
example above, if we wanted to know what partition each record belongs to, we
can run the following:

%copy first-line%
```bash
$ fluvio consume fruits -B --all-partitions --format="Partition({{partition}}): {{key}}={{value}}"
Consuming records from the beginning of topic 'fruits'
Partition(0): null=apple
Partition(1): null=banana
Partition(0): null=cranberry
Partition(1): null=date
```

## Conclusion

For the full list of changes this week, be sure to check out [our CHANGELOG]. If you have any
questions or are interested in contributing, be sure to [join our Discord channel] and
come say hello!

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[join our Discord channel]: https://discordapp.com/invite/bBG2dTz
