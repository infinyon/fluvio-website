---
title: Aggregate
weight: 60
toc: true
---

-> **Note**: Currently, `aggregate` SmartModules are not supported in connectors.

SmartModule Aggregates are functions that define how to combine each record
in a stream with some accumulated value. In the functional programming world,
this type of operation is also known as `folding`, since the function "folds"
each new value into the accumulator.

<img src="/smartmodules/images/smartmodule-aggregate.svg" alt="SmartModule Aggregate" justify="center" height="230">

Let's set up a new SmartModule project so that we can look at some code while
introducing aggregators. 

##### Prerequisites

This section assumes that SMDK is [installed].

## Generic Example: Aggregate Numbers

A common use cases is to aggregate a series of number and apply a particular operation, in this case it will be `sum`.

Let's dive in and see how to this up in Fluvio.

### Create a SmartModule Project

Run `smdk generate` with the name of the aggregate and choose the  "aggregate" options:

%copy first-line%
```bash
$ smdk generate example-aggregate
project-group => 'john'
fluvio-smartmodule-cargo-dependency => '"0.3.0"'
🔧   Destination: ~/smdk/example-aggregate ...
🔧   Generating template ...
✔ 🤷   Which type of SmartModule would you like? · aggregate
✔ 🤷   Will your SmartModule use init parameters? · false
Ignoring: /var/folders/5q/jwc86771549058kmbkbqjcdc0000gn/T/.tmp0ZJHbN/cargo-generate.toml
[1/5]   Done: Cargo.toml
[2/5]   Done: README.md
[3/5]   Done: SmartModule.toml
[4/5]   Done: src/lib.rs
[5/5]   Done: src
🔧   Moving generated files into: `~/smdk/example-aggregate`...
💡   Initializing a fresh Git repository
✨   Done! New project created~/smdk/example-aggregate
```

### Code Generator for Aggregate

Let's take a look at the starter code from the template, located in `src/lib.rs`:

%copy first-line%
```bash
$ cd example-aggregate && cat ./src/lib.rs
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
return the new value, we convert the sum to a String and return it, using `.into()` to convert the String to a `RecordData`.

### Build the SmartModule

Let's make sure our code compiles. If eveything works as expected, there is a `.wasm` file generated in the target directory.

%copy first-line%
```bash
$ smdk build
...
Compiling example-aggregate v0.1.0 (~/smdk/example-aggregate)
Finished release-lto [optimized] target(s) in 12.20s
```

Your SmartModule WASM binary is now ready for use.

### Test on Cluster

Let's create a new Fluvio topic to produce the sample records we want to consume with our SmartModule:

%copy first-line%
```bash
$ fluvio topic create aggregate-ints
topic "aggregate-ints" created
```

Next, we produce some data to the topic. Remember, our goal here is to sum up
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

Let's double check it's all there.

%copy first-line%
```bash
$ fluvio consume aggregate-ints -dB
Consuming records from the beginning of topic 'aggregate-ints'
1
1
1
1
1
10
```

### Load SmartModule to Fluvio

The SmartModule can be loaded to local Fluvio Cluster or [InfinyOn Cloud], as determined by the [`current profile`]. In this example, the profile points to InfinyOn Cloud.

%copy first-line%
```bash
$ smdk load
Loading package at: ~/smdk/example-aggregate
Found SmartModule package: example-aggregate
loading module at: ~/smdk/example-aggregate/target/wasm32-unknown-unknown/release-lto/example_aggregate.wasm
Trying connection to fluvio router.infinyon.cloud:9003
Creating SmartModule: example-aggregate
```

%copy first-line%
```bash
$ fluvio smartmodule list
  SMARTMODULE                   SIZE     
  john/example-aggregate@0.1.0   91.5 KB
```

SmartModules that have been uploaded on the cluster can be used by other areas of the system (consumers, producers, connectors, etc):


#### Run Aggregates with default initial value

The default inial value for aggregagates is an "empty record", let's see it in action:

%copy first-line%
```bash
$ fluvio consume aggregate-ints -dB --smartmodule=john/example-aggregate@0.1.0
Consuming records from the beginning of topic 'aggregate-ints'
1
2
3
4
5
15
```

#### Run Aggregate with pre-defined initial value

If we want to specify an initial value other than "empty record", we can use the `--aggregate-initial` flag in the Fluvio CLI to specify a value, let's use `100`:

%copy first-line%
```bash
$ fluvio consume aggregate-ints -dB --aggregate-initial="100" --smartmodule=john/example-aggregate@0.1.0
101
102
103
104
105
115
```

Congratulations! :tada: Eveything worked as expected!


## Publish to SmartModule Hub

Let's [publish] this SmartModule to [SmartModule Hub] to make accessible to others.

%copy first-line%
```bash
$ smdk publish
Creating package john/example-aggregate@0.1.0
.. fill out info in hub/package-meta.yaml
Package hub/example-aggregate-0.1.0.ipkg created
Package uploaded!
```

Let's double check that the SmartModule is available for download:

%copy first-line%
```bash
$ fluvio hub list
  SMARTMODULE                    
  john/example-aggregate@0.1.0    
```

Congratulations! :tada: Your SmartModule is now available for download in the SmartModule Hub.


## Read next

- [Explore aggregate use-cases](https://www.infinyon.com/blog/2021/08/smartstream-aggregates/)
- [Writing a JSON filter]({{< ref "/smartmodules/transforms/filter" >}})
- [Writing a map to transform records]({{< ref "/smartmodules/transforms/map" >}})


[installed]: {{< ref "smartmodules/smdk/install" >}}
[publish]: {{< ref "smartmodules/smdk/publish" >}}
[InfinyOn Cloud]: https://infinyon.cloud
[`current profile`]: {{< ref "cli/client/profile" >}}
[SmartModule Hub]: {{< ref "smartmodules/hub/overview" >}}
