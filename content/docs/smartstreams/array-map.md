---
title: ArrayMap
weight: 30
toc: false
---

SmartStream ArrayMaps are used to break apart Records into smaller pieces.
We say that an ArrayMap "flattens" each input record and produces zero to
many output records from it. For example, with a ArrayMap, you could
convert the following input stream:

```bash
["a", "b"]
["c", "d"]
["e", "f"]
```

Into this output stream:

```bash
"a"
"b"
"c"
"d"
"e"
"f"
```

An ArrayMap can be very useful when each of your records contains
a _collection_ of data points that each make sense independently. A good example
of this might be with API pagination: if each of your records contains a page
of results from an API, it may make sense to flatten those pages into a stream
of the individual elements from the API.

Let's take a look at an example ArrayMap and walk through how it works and
what some sample input and output data might look like.

### Create a new Project

We can use the `cargo-generate` tool to create a new SmartStreams project that
is ready to go. If you don't already have it, you can install `cargo-generate`
using this command:

```bash
$ cargo install cargo-generate
```

Then, use the following command to create a new SmartStreams ArrayMap project.

```bash
$ cargo generate --git="https://github.com/infinyon/fluvio-smartstream-template"
⚠️   Unable to load config file: ~/.cargo/cargo-generate.toml
🤷   Project Name : array-map-array
🔧   Generating template ...
✔ 🤷   Which type of SmartStream would you like? · array-map
[1/7]   Done: .cargo/config.toml
[2/7]   Done: .cargo
[3/7]   Done: .gitignore
[4/7]   Done: Cargo.toml
[5/7]   Done: README.md
[6/7]   Done: src/lib.rs
[7/7]   Done: src
🔧   Moving generated files into: `array-map-array`...
✨   Done! New project created array-map-array
```

The code in this generated project takes JSON arrays as input records and
returns the _elements_ of those arrays as output records. Let's take a look
at the full source, then we'll cover it piece by piece.

```rust
use fluvio_smartstream::{smartstream, Record, RecordData, Result};

#[smartstream(array_map)]
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

1) Deserialize a JSON array as input and store it in a `Vec<Value>`.

The input 


















### Read next

- [Explore map use-cases](https://www.infinyon.com/blog/2021/08/smartstream-map-use-cases/)
- [Writing a JSON filter]({{< ref "/docs/smartstreams/filter" >}})
- [Writing an aggregate to sum numbers]({{< ref "/docs/smartstreams/aggregate" >}})
