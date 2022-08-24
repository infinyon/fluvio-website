---
title: "This Week in Fluvio #3"
date: 2021-08-19
weight: 20
---

Welcome to the third edition of This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

[Fluvio open source]: https://github.com/infinyon/fluvio

{{< banner >}}

## New Release - Fluvio v0.9.3

### SmartStreams Aggregates

This week's release has one big feature and several quality-of-life user
experience improvements. The big feature is the arrival of our [SmartStreams Aggregate]
feature, which allows users to write functions that combine records into
a long-running accumulator state. Good examples of accumulator use-cases
are calculating [sums] or [averages] of numeric data, or
[combining structural key-value data].

To quickly illustrate what a SmartStream Aggregate looks like, let's take a look
at the simplest example of just summing numbers together:

```rust
use fluvio_smartstream::{smartstream, Result, Record, RecordData};

#[smartstream(aggregate)]
pub fn aggregate(accumulator: RecordData, current: &Record) -> Result<RecordData> {
    // Parse the accumulator and current record as strings
    let accumulator_string = std::str::from_utf8(accumulator.as_ref())?;
    let current_string = std::str::from_utf8(current.value.as_ref())?;

    // Parse the strings into integers
    let accumulator_int = accumulator_string.parse::<i32>().unwrap_or(0);
    let current_int = current_string.parse::<i32>()?;

    // Take the sum of the two integers and return it as a string
    let sum = accumulator_int + current_int;
    Ok(sum.to_string().into())
}
```

Every aggregate function has two inputs: an "accumulator" and the current record from
the stream. The function's jobs is to combine these two values and produce a new
accumulator value, which will be used when processing subsequent records in the stream.
In this example, we simply parse the accumulator and input record as integers, then
add them together.

### Client fix: Out-of-bounds relative offsets no longer cause freezing

When consuming records in Fluvio, we generally open a stream by asking for a particular
Topic and partition, and providing an Offset for the first record where we would like
our stream to begin reading. Offsets may be specified in three ways: by giving an
**absolute** index into the partition, by a **relative-from-beginning** offset, and
by a **relative-from-end** offset. When you specify a relative offset, they are
first resolved to absolute offsets and then used to make a stream request.

Prior to `0.9.3`, there was a bug in the client where relative offsets could overflow
the actual size of the stream, which would cause the consumer to simply freeze and not
yield any records or errors. An example that would cause this problem would be if you
had a topic with 10 records in it, and you asked for a stream starting "20 from the end"
of that topic. This would incorrectly resolve to an absolute offset of `10 - 20 = -10`.

This bug has been fixed in `0.9.3`, with a new behavior where relative offsets that
are too large simply "bottom out" at the start or end of the stream. That is, if you
ask for "1000 from the end" in a stream of 100 elements, you'll just start streaming
from the start, and if you ask for "1000 from the beginning" of a stream of 100 elements,
you'll just start at the end, waiting for new records to arrive.

### CLI usability: Added `fluvio consume -T/--tail` for "tailing" consumer streams

This is a nice usability improvement for Fluvio CLI users, where you may now "tail"
your streams. This acts similarly to the UNIX `tail` command, which reads the last 10
lines of a file. `fluvio consume --tail=X` will open a stream that begins `X` elements
from the end, letting you quickly see the most recent records in your stream. By
default, using `--tail` with no argument (no `X`) will give you the last 10 elements
for some easy context over the latest records (just like UNIX `tail` does).

As a quick example, this is what happens when you use `--tail` on a stream with 20
sequential integers.

```bash
$ fluvio consume ints --tail
Consuming records starting 10 from the end of topic 'ints'
11
12
13
14
15
16
17
18
19
20
```

## Conclusion

For the full list of changes, be sure to check out [our CHANGELOG]. If you have any
questions or are interested in contributing, be sure to [join our Discord channel] and
come say hello!

Until next week!

[SmartStreams Aggregate]: {{<ref "/smartmodules/analytics/aggregate" >}}
[sums]: https://github.com/infinyon/fluvio/blob/master/crates/fluvio-smartmodule/examples/aggregate-sum/src/lib.rs
[averages]: https://github.com/infinyon/fluvio/blob/master/crates/fluvio-smartmodule/examples/aggregate-average/src/lib.rs
[combining structural key-value data]: https://github.com/infinyon/fluvio/blob/master/crates/fluvio-smartmodule/examples/aggregate-json/src/lib.rs
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[join our Discord channel]: https://discordapp.com/invite/bBG2dTz
