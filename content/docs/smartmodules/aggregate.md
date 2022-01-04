---
title: Aggregate
weight: 40
toc: false
---

SmartModule Aggregates are functions that define how to combine each record
in a stream with some accumulated value. In the functional programming world,
this type of operation is also known as `folding`, since the function "folds"
each new value into the accumulator.

<img src="/docs/smartmodules/images/smartmodule-aggregate.svg" alt="SmartModule Aggregate" justify="center" height="250">

Let's set up a new SmartModule project so that we can look at some code while
introducing aggregators. 

We use `cargo-generate` to set up a blank SmartModule Aggregator project. If you don't have it yet, run the following command to install it:

%copy first-line%
```bash
$ cargo install cargo-generate
```

Use `cargo generate` to create a new SmartModule project, and be sure to select the "aggregate" type during setup.

%copy first-line%
```bash
$ cargo generate --git https://github.com/infinyon/fluvio-smartmodule-template
🤷   Project Name : example-aggregate
🔧   Creating project called `example-aggregate`...
✔ 🤷   Which type of SmartModule would you like? · aggregate
[1/5]   Done: .cargo/config.toml
[2/5]   Done: .gitignore
[3/5]   Done: Cargo.toml
[4/5]   Done: README.md
[5/5]   Done: src/lib.rs
✨   Done! New project created example-aggregate
```

Let's take a look at the starter code from the template, located in `src/lib.rs`:

%copy first-line%
```bash
cd example-aggregate && cat ./src/lib.rs
```

```rust
use fluvio_smartmodule::{smartmodule, Result, Record, RecordData};

#[smartmodule(aggregate)]
pub fn aggregate(accumulator: RecordData, current: &Record) -> Result<RecordData> {
    // Parse the accumulator and current record as strings
    let accumulator_string = std::str::from_utf8(accumulator.as_ref())?;
    let current_string = std::str::from_utf8(current.value.as_ref())?;

    // Parse the strings into integers
    let accumulator_int = accumulator_string.trim().parse::<i32>().unwrap_or(0);
    let current_int = current_string.trim().parse::<i32>()?;

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
aggregators can operate over arbitrary data types. This is the reason that in this example, we first need to parse the input as strings and then as integers.

Aggregate functions require us to return a buffer of data that represents
the new accumulated value. In this example, the new accumulated value is the
arithmetic sum of the old accumulator and the current record as integers. To
return the new value, we convert the sum to a String and return it, using `.into()`
to convert the String to a `RecordData`.

## Running the SmartModule Aggregator

First, let's create a topic where we'll produce and consume our data from.

%copy first-line%
```bash
$ fluvio topic create aggregate-ints
topic "aggregate-ints" created
```

Then we'll produce some data to the topic. Remember, our goal here is to sum up
integers in a stream, so we'll produce some sample input integers that we can read using the aggregator.

%copy first-line%
```bash
$ fluvio produce aggregate-ints
> 1
Ok!
> 1
Ok!
> 1
Ok!
> 1
Ok!
> 1
Ok!
> 10
Ok!
```

Let's compile the SmartModule:

%copy first-line%
```bash
$ cargo build --release
```

Now, open a consumer and use the `--aggregate` flag to point it to your WASM module:

%copy first-line%
```bash
$ fluvio consume aggregate-ints -B --aggregate=target/wasm32-unknown-unknown/release/example_aggregate.wasm
Consuming records from the beginning of topic 'aggregate-ints'
1
2
3
4
5
15
```

### Aggregate with initial value

If we want to specify an initial value other than "empty record", we can use the `--initial` flag in the Fluvio CLI to specify a file to use as the initial file. So let's say we put the value `100`
into a text file:

%copy first-line%
```bash
$ echo '100' > initial.txt
```

Then, we can re-run our consumer and give `initial.txt` as the initial value to use for our accumulator value in the stream:

%copy first-line%
```bash
$ fluvio consume aggregate-ints -B --initial=./initial.txt --aggregate=target/wasm32-unknown-unknown/release/example_aggregate.wasm
101
102
103
104
105
115
```

## Register the SmartModule with Fluvio

After building a SmartModule as a WASM binary, it may be registered with Fluvio using the `fluvio smart-module` command:

%copy first-line%
```bash
$ fluvio smart-module create aggregate-sm --wasm-file target/wasm32-unknown-unknown/release/example_aggregate.wasm
```

Use the `fluvio smart-module list` command to see all available SmartModules:

%copy first-line%
```bash
$ fluvio smart-module list
 NAME          STATUS             SIZE
 aggregate-sm  SmartModuleStatus  113999 
```

Once the SmartModule is created, it can be used by other areas of the system (consumers, producers, connectors, etc):

%copy first-line%
```bash
$ fluvio consume aggregate-ints -B --aggregate=aggregate-sm
```



### Read next

- [Explore aggregate use-cases](https://www.infinyon.com/blog/2021/08/smartstream-aggregates/)
- [Writing a JSON filter]({{< ref "/docs/smartmodules/filter" >}})
- [Writing a map to transform records]({{< ref "/docs/smartmodules/map" >}})
