---
title: ArrayMap
weight: 35
toc: false
---

SmartStream ArrayMaps are used to break apart Records into smaller pieces.
This can be very useful for working with your data at a fine granularity.
Often, each record in a Topic may actually represent many data points, but
we'd like to be able to analyze and manipulate those data points independently.
ArrayMap allows us to dig in and break apart these composite records into
the smaller units of data that we want to work with.

For example, suppose that each element in our Topic is a JSON array. We
might want to interact with the _elements_ of these arrays rather than
the arrays themselves. Using an ArrayMap, we could transform a Topic whose
records look like this:

```bash
["a", "b"]
["c", "d"]
["e", "f"]
```

Into a topic that contains each of those elements as a distinct record,
like this:

```bash
"a"
"b"
"c"
"d"
"e"
"f"
```

A good practical example of when you might want to use ArrayMap might be with API
pagination. In this scenario, each record in our topic would represent a "page" of
data from an API, but we would like to analyze and operate on just the elements
we find in those pages. For example, if we perform an image search and receive
pages of search results, we probably only care about the search results themselves
and what order they were received in, we don't necessarily care about how many
results landed in each page. By using ArrayMap to put each search result into a
distinct record, we can simplify our data processing and analysis later in the stream.

Let's take a look at an example ArrayMap and walk through how it works and
what some sample input and output data might look like. The ArrayMap we'll look
at will simply read a topic full of JSON arrays and produce a stream of the
elements of those arrays, similar to the input and output we saw above.

### Create a new Project

We can use the `cargo-generate` tool to create a new SmartStreams project that
is ready to go. If you don't already have it, you can install `cargo-generate`
using this command:

%copy first-line%
```bash
$ cargo install cargo-generate
```

Then, use the following command to create a new SmartStreams ArrayMap project.

%copy first-line%
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

We'll want to `cd` into the project directory for the rest of the commands
to work:

```bash
$ cd array-map-array
```

The code in this generated project takes JSON arrays as input records and
returns the _elements_ of those arrays as output records. Let's take a look
at the full source, then we'll cover it piece by piece. Let's look at
`src/lib.rs`:

%copy%
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

1) Deserialize a JSON array as input and store it in a `Vec<Value>`
2) Converts each `Value` back into a JSON string
3) Converts each JSON string into a distinct output Record

Let's take this for a test drive and see it in action.

### Running the ArrayMap

Before getting started, make sure you have [downloaded the Fluvio CLI] and followed
the getting started guide to get up and running with a Fluvio cluster. Then, if you
haven't done so already, you'll need to install the `wasm32-unknown-unknown` target
for Rust using the following command:

%copy first-line%
```bash
$ rustup target add wasm32-unknown-unknown
```

Now we'll be able to compile the ArrayMap SmartStream. Let's use release mode so
we get the smallest WASM binary possible:

%copy first-line%
```bash
$ cargo build --release
```

Next, we'll need to create a new Fluvio topic to produce and consume our data using
this command:

%copy first-line%
```bash
$ fluvio topic create array-map
topic "array-map" created
```

Now we can produce some test data to our topic.

%copy first-line%
```bash
$ fluvio produce array-map
> ["a", "b"]
Ok!
> ["c", "d"]
Ok!
> ["e", "f"]
Ok!
> ^C
```

Finally, let's consume our data using our ArrayMap SmartStream and see that each
of the output records shows just one of the elements from each input array.

%copy first-line%
```bash
$ fluvio consume array-map -B --array-map=target/wasm32-unknown-unknown/release/array_map_array.wasm
"a"
"b"
"c"
"d"
"e"
"f"
```

Congratulations, you just completed your first ArrayMap example! You can find the
[full source code for this example on GitHub], along with the full sources for many
other SmartStreams examples.

### Read next

- [Explore map use-cases](https://www.infinyon.com/blog/2021/08/smartstream-map-use-cases/)
- [Writing a JSON filter]({{< ref "/docs/smartstreams/filter" >}})
- [Writing an aggregate to sum numbers]({{< ref "/docs/smartstreams/aggregate" >}})

[downloaded the Fluvio CLI]: https://www.fluvio.io/download/
[full source code for this example on GitHub]:
