---
title: Map API
menu: Map
weight: 30
toc: true
---

SmartModule Maps are used to transform or edit each Record in a stream. We say that these SmartModules "map" each input record into a new output record by applying a function to the input data. This type of SmartModule may be used for many use-cases, such as:

- Narrowing large records into a smaller subset of important fields
- Scrubbing sensitive fields of data to be invisible to downstream consumers
- Computing rich, derived fields from simple raw data

Let's create a brand-new SmartModule Map to see what a minimal working
example looks like.

<img src="/smartmodules/images/smartmodule-map.svg" alt="SmartModule Map" justify="center" height="190">


##### Prerequisites

This section assumes that SMDK is [installed].


### Create a SmartModule Project

Run `smdk generate` with the name of the map and choose the "map" options:

%copy first-line%
```bash
$ smdk generate map-example
Generating new SmartModule project: map-example
project-group => 'john'
fluvio-smartmodule-cargo-dependency => '"0.2.5"'
🔧   Destination: ~/smdk/map-example ...
🔧   Generating template ...
✔ 🤷   Will your SmartModule use init parameters? · false
✔ 🤷   Which type of SmartModule would you like? · map
Ignoring: /var/folders/5q/jwc86771549058kmbkbqjcdc0000gn/T/.tmpNFObJj/cargo-generate.toml
[1/5]   Done: Cargo.toml
[2/5]   Done: README.md
[3/5]   Done: SmartModule.toml
[4/5]   Done: src/lib.rs
[5/5]   Done: src
🔧   Moving generated files into: `~/smdk/map-example`...
💡   Initializing a fresh Git repository
✨   Done! New project created ~/smdk/map-example
```

We should see a new folder has been created for our project `map-example`. Let's navigate inside and take a look at the sample Map generated for us by the template:

%copy first-line%
```bash
$ cd map-example && cat ./src/lib.rs
```

%copy%
```rust
use fluvio_smartmodule::{smartmodule, Result, Record, RecordData};

#[smartmodule(map)]
pub fn map(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let key = record.key.clone();

    let string = std::str::from_utf8(record.value.as_ref())?;
    let int = string.parse::<i32>()?;
    let value = (int * 2).to_string();

    Ok((key, value.into()))
}
```

Let's break down what's happening here:

- Firstly, `#[smartmodule(map)]` marks the entry point for this SmartModule Map.
  There may only be one of these in the project, and it is called once for each
  record in the data stream.
- The annotated function `fn map` may be named anything, but it must take a
  single `&Record` argument. This variable contains the contents of one record
  in the stream, and you may read the Key and Value of this record as bytes.
- The `fn map` function must return a new Key and Value for the output record.
  The Key is the `Option<RecordData>` and the Value is the `RecordData` in the
  return type. `RecordData` is a helper type that may be constructed from any
  type that has `impl Into<Vec<u8>>` such as `String`, by using `.into()`.
- At any point in the SmartModule, errors may be returned using `?` or via
  `Err(e.into())`. This works for any error type that has `impl std::error::Error`.

This template SmartModule will parse each record as an `i32` integer, then multiply that value by 2. 

Let's make sure our code compiles. If eveything works as expected, there is a `.wasm` file generated in the target directory.

%copy first-line%
```bash
$ smdk build
...
Compiling map-example v0.1.0 (~/smdk/map-example)
Finished release-lto [optimized] target(s) in1 12.83s
```

Your WASM binary is now ready for use.

### Test with SMDK

Now that we've written our map, let's test using the command line. 

%copy first-line%
```bash
$ smdk test --text=6
loading module at: ~/smdk/map-example/target/wasm32-unknown-unknown/release-lto/map_example.wasm
1 records outputed
12
```

Good news! :tada: it works as expected!


### Test on Cluster

Let's create a new topic to produce our source data:

%copy first-line%
```bash
$ fluvio topic create map-double
topic "map-double" created
```

In a new terminal, producde the following entries:

%copy first-line%
```bash
$ fluvio produce map-double   
> 1
Ok!
> 2
Ok!
> 3
Ok!
> 4
Ok!
> 5
Ok!
> ^C
```

Let's double check it's all there.

%copy first-line%
```bash
$ fluvio consume map-double -B -d
Consuming records from the beginning of topic 'map-double'
1
2
3
4
5
```

#### Load SmartModule to Fluvio

The SmartModule can be loaded to local Fluvio Cluster or [InfinyOn Cloud], as determined by the [`current profile`]. In this example, the profile points to InfinyOn Cloud.

%copy first-line%
```bash
$ smdk load 
Loading package at: ~/smdk/map-example
Found SmartModule package: map-example
loading module at: ~/smdk/map-example/target/wasm32-unknown-unknown/release-lto/map_example.wasm
Trying connection to fluvio router.infinyon.cloud:9003
Creating SmartModule: map-example
```

Rust `fluvio smartmodule list` to ensure your SmartModule has been uploaded:

%copy first-line%
```bash
$ fluvio smartmodule list
 SMARTMODULE                    SIZE     
 john/map-example@0.1.0         87.5 KB 
```

SmartModule that have been uploaded on the cluster can be used by other areas of the system (consumers, producers, connectors, etc):

%copy first-line%
```bash
$ fluvio consume map-double -B --smartmodule=john/map-example@0.1.0
Consuming records from the beginning of topic 'map-double'
2
4
6
8
10
```

Congratulations! :tada: Eveything worked as expected!


### Publish to SmartModule Hub

As bonus, let's [publish] this SmartModule to [SmartMoudle Hub].

%copy first-line%
```bash
$ smdk publish
Creating package john/map-example@0.1.0
.. fill out info in hub/package-meta.yaml
Package hub/map-example-0.1.0.ipkg created
Package uploaded!
```

Let's double check that the SmartModule is available for download:

%copy first-line%
```bash
$ fluvio hub list
  SMARTMODULE                    
  john/map-example@0.1.0        
```

Congratulations! :tada: Your SmartModule is now available for download in the SmartModule Hub.


### Read next

- [Explore map use-cases](https://www.infinyon.com/blog/2021/08/smartstream-map-use-cases/)
- [Writing a JSON filter]({{< ref "/smartmodules/apis/filter" >}})
- [Writing an aggregate to sum numbers]({{< ref "/smartmodules/apis/aggregate" >}})


[installed]: {{< ref "smartmodules/smdk/install" >}}
[publish]: {{< ref "smartmodules/smdk/publish" >}}
[InfinyOn Cloud]: https://infinyon.cloud
[`current profile`]: {{< ref "cli/client/profile" >}}
[SmartMoudle Hub]: {{< ref "smartmodules/hub/overview" >}}