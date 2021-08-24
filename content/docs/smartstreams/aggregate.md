---
title: Aggregate
weight: 40
toc: false
---

SmartStream Aggregates are functions that define how to combine each record
in a stream with some accumulated value. In the functional programming world,
this type of operation is also known as `folding`, since the function "folds"
each new value into the accumulator.

Let's set up a new SmartStream project so that we can look at some code while
introducing aggregators. Use `cargo-generate` to create a new SmartStream project,
and be sure to select the "aggregate" type during setup.

```bash
$ cargo install cargo-generate
$ cargo generate --git https://github.com/infinyon/fluvio-smartstream-template
🤷   Project Name : example-aggregate
🔧   Creating project called `example-aggregate`...
✔ 🤷   Which type of SmartStream would you like? · aggregate
[1/5]   Done: .cargo/config.toml
[2/5]   Done: .gitignore
[3/5]   Done: Cargo.toml
[4/5]   Done: README.md
[5/5]   Done: src/lib.rs
✨   Done! New project created example-aggregate
```

Let's take a look at the starter code from the template, located in `src/lib.rs`.

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

This example shows an aggregate function that adds all the integers in a stream
together. In our `aggregate` function, we get two inputs:

- The `accumulator`, which is everything we have summed so far, and
- The `current` record, whose contents we want to add to the accumulator

The return value from our aggregate function will be the result of adding the record
to our accumulator. This value will be emitted in the output stream, and it will also
be passed as the `accumulator` argument to the next call to `aggregate`, with the subsequent record.

The input values are passed into the aggregator in a binary representation, so
aggregators can operate over arbitrary data types. This is the reason that in this example,
we first need to parse the input as strings and then as integers.

Aggregate functions require us to return a buffer of data that represents
the new accumulated value. In this example, the new accumulated value is the
arithmetic sum of the old accumulator and the current record as integers. To
return the new value, we convert the sum to a String and return it, using `.into()`
to convert the String to a `RecordData`.

### Read next

- [Writing a JSON filter]({{< ref "/docs/smartstreams/filter" >}})
- [Writing a map to transform records]({{< ref "/docs/smartstreams/map" >}})
