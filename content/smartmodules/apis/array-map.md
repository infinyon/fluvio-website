---
title: ArrayMap API
menu: ArrayMap
weight: 50
toc: true
---

SmartModule ArrayMaps are used to break apart Records into smaller pieces. This can be very useful for working with your data at a fine granularity. Often, each record in a Topic may actually represent many data points, but we'd like to be able to analyze and manipulate those data points independently. ArrayMap allows us to dig in and break apart these composite records into the smaller units of data that we want to work with.

<img src="/smartmodules/images/smartmodule-arraymap.svg" alt="SmartModule ArrayMap" justify="center" height="190">

Let's take a look at an example ArrayMap and walk through how it works and what some sample input and output data might look like.

##### Prerequisites

This section assumes that SMDK is [installed].

## Generic Example: Transform JSON arrays to records

A common use case is to transform JSON arrays and produce a stream of the records of those arrays. For example, suppose that each element we receive in Fluvio is a JSON array, though we want to interact with the _elements_ of these arrays rather than the arrays themselves. Then, using an ArrayMap, we can transform composite arrays to records that look like this (where this line is a single record):

```bash
["a", "b", "c"]
```

But, we want those elements as a distinct record, like this (where each line is a distinct record):

```bash
"a"
"b"
"c"
```

> If you'd like to see a practical example of ArrayMap in action, check out our blog on [using ArrayMap to break apart paginated API requests].

Let's dive in and see how to this up in Fluvio.

### Create a SmartModule Project

Run `smdk generate` with the name of the filter and choose the  "filter" options:

%copy first-line%
```bash
$ smdk generate array-map
Generating new SmartModule project: array-map
project-group => 'john'
fluvio-smartmodule-cargo-dependency => '"0.3.0"'
🔧   Destination: ~/smdk/array-map ...
🔧   Generating template ...
✔ 🤷   Will your SmartModule use init parameters? · false
✔ 🤷   Which type of SmartModule would you like? · array-map
Ignoring: /var/folders/5q/jwc86771549058kmbkbqjcdc0000gn/T/.tmp4imt4g/cargo-generate.toml
[1/5]   Done: Cargo.toml
[2/5]   Done: README.md
[3/5]   Done: SmartModule.toml
[4/5]   Done: src/lib.rs
[5/5]   Done: src
🔧   Moving generated files into: `~/smdk/array-map`...
💡   Initializing a fresh Git repository
✨   Done! New project created ~/smdk/array-map
```

### Code Generator for ArrayMap

The code in this generated project takes JSON arrays as input records and
returns the _elements_ of those arrays as output records. Let's take a look at the full source, then we'll cover it piece by piece. Let's look at `src/lib.rs`:

%copy first-line%
```bash
$ cd array-map && cat src/lib.rs 
```

```rust
use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};

#[smartmodule(array_map)]
pub fn array_map(record: &Record) -> Result<Vec<(Option<RecordData>, RecordData)>> {
    // Deserialize a JSON array with any kind of values inside
    let array: Vec<serde_json::Value> = serde_json::from_slice(record.value.as_ref())?;

    // Convert each JSON value from the array back into a JSON string
    let strings: Vec<String> = array
        .into_iter()
        .map(|value| serde_json::to_string(&value))
        .collect::<core::result::Result<_, _>>()?;

    // Create one record from each JSON string to send
    let records: Vec<(Option<RecordData>, RecordData)> = strings
        .into_iter()
        .map(|s| (None, RecordData::from(s)))
        .collect();
    Ok(records)
}
```

This ArrayMap essentially has three steps it takes:

1) Deserialize a JSON array as input and store it in a `Vec<Value>`
2) Converts each `Value` back into a JSON string
3) Converts each JSON string into a distinct output Record

Let's take this for a test drive and see it in action.

### Build the SmartModule

Let's make sure our code compiles. If eveything works as expected, there is a `.wasm` file generated in the target directory.

%copy first-line%
```bash
$ smdk build
...
Compiling array-map v0.1.0 (~/smdk/array-map)
Finished release-lto [optimized] target(s) in 11.31s
```

Your SmartModule WASM binary is now ready for use.

### Test with SMDK

Let's test our work using the command line `test` facility.

%copy first-line%
```bash
$ smdk test --text='["a", "b", "c"]'
loading module at: ~/smdk/array-map/target/wasm32-unknown-unknown/release-lto/array_map.wasm
3 records outputed
"a"
"b"
"c"
```

Great, everything works as expected. Let's test on cluster.


### Test on Cluster

Let's create a new Fluvio topic to produce the sample records we want to consume with our SmartModule:

%copy first-line%
```bash
$ fluvio topic create array-map
topic "array-map" created
```

Now we can produce the sample data to our topic.

%copy first-line%
```bash
$ fluvio produce array-map
> ["a", "b", "c"]
Ok!
> ["d", "e", "f"]             
Ok!
> ^C
```

Let's double check it's all there.

%copy first-line%
```bash
$ fluvio consume array-map -dB
Consuming records from the beginning of topic 'array-map'
["a", "b", "c"]
["d", "e", "f"]
```

### Load SmartModule to Fluvio

The SmartModule can be loaded to local Fluvio Cluster or [InfinyOn Cloud], as determined by the [`current profile`]. In this example, the profile points to InfinyOn Cloud.

%copy first-line%
```bash
$ smdk load
Found SmartModule package: array-map
loading module at: ~/smdk/array-map/target/wasm32-unknown-unknown/release-lto/array_map.wasm
Trying connection to fluvio router.infinyon.cloud:9003
Creating SmartModule: array-map
```

Rust `fluvio smartmodule list` to ensure your SmartModule has been uploaded:

%copy first-line%
```bash
$ fluvio smartmodule list
  SMARTMODULE                   SIZE     
  john/array-map@0.1.0          157.8 KB
```

SmartModule that have been uploaded on the cluster can be used by other areas of the system (consumers, producers, connectors, etc):

%copy first-line%
```bash
$ fluvio consume array-map -dB --smartmodule=john/array-map@0.1.0
Consuming records from the beginning of topic 'array-map'
"a"
"b"
"c"
"d"
"e"
"f"
```

Congratulations! :tada: Eveything worked as expected!


## Publish to SmartModule Hub

It turns out this SmartModule was requested by other data streaming teams in the organization, so we've decided to [publish] it on [SmartMoudle Hub].

%copy first-line%
```bash
$ smdk publish
Creating package john/array-map@0.1.0
.. fill out info in hub/package-meta.yaml
Package hub/array-map-0.1.0.ipkg created
Package uploaded!
```

Let's double check that the SmartModule is available for download:

%copy first-line%
```bash
$ fluvio hub list
  SMARTMODULE                    
  john/array-map@0.1.0      
```

Congratulations! :tada: Your SmartModule is now available for download in the SmartModule Hub.


## Read next

- [Explore map use-cases](https://www.infinyon.com/blog/2021/08/smartstream-map-use-cases/)
- [Writing a JSON filter]({{< ref "/smartmodules/apis/filter" >}})
- [Writing an aggregate to sum numbers]({{< ref "/smartmodules/apis/aggregate" >}})

[installed]: {{< ref "smartmodules/smdk/install" >}}
[publish]: {{< ref "smartmodules/smdk/publish" >}}
[InfinyOn Cloud]: https://infinyon.cloud
[`current profile`]: {{< ref "cli/client/profile" >}}
[SmartMoudle Hub]: {{< ref "smartmodules/hub/overview" >}}

[downloaded the Fluvio CLI]: https://www.fluvio.io/download/
[using ArrayMap to break apart paginated API requests]: https://infinyon.com/blog/2021/10/smartstream-array-map-reddit/
[full source code for this example on GitHub]: https://github.com/infinyon/fluvio/blob/095d8f0cbbcc79ebc71cea464cd653ffde7af4e0/crates/fluvio-smartstream/examples/array_map_json_array/src/lib.rs
