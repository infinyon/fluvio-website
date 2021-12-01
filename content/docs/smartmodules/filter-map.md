---
title: FilterMap
weight: 33
toc: false
---

SmartModule FilterMaps are used to both transform _and_ potentially filter
records from a stream at the same time. This can be useful for a number of
scenarios including working with data with nullable fields, or working with
subsets of event data. In these cases, FilterMap allows us discard irrelevant
data - such as records with null fields or event types that we don't care about -
while also performing meaningful work with relevant data - such as reformatting
fields we've extracted or events we've gathered.

FilterMap functions work by returning an `Option` of a new record. To discard a
record from the stream, return `None`. Otherwise, transform
the record according to your needs and return it as `Some(record)`.

### Create a new Project

We can use the `cargo-generate` tool to create a new SmartModules project that
is ready to go. If you don't already have it, you can install `cargo-generate`
using this command:

%copy first-line%
```bash
$ cargo install cargo-generate
```

Then, use the following command to create a new SmartModules FilterMap project.

%copy first-line%
```bash
$ cargo generate --git="https://github.com/infinyon/fluvio-smartmodule-template"
⚠️   Unable to load config file: ~/.cargo/cargo-generate.toml
🤷   Project Name : filter-map
🔧   Generating template ...
✔ 🤷   Which type of SmartModule would you like? · array-map
[1/7]   Done: .cargo/config.toml
[2/7]   Done: .cargo
[3/7]   Done: .gitignore
[4/7]   Done: Cargo.toml
[5/7]   Done: README.md
[6/7]   Done: src/lib.rs
[7/7]   Done: src
🔧   Moving generated files into: `filter-map`...
✨   Done! New project created filter-map
```

We'll want to `cd` into the project directory for the rest of the commands
to work:

%copy first-line%
```bash
$ cd filter-map
```

%copy%
```rust
// todo
```

### Running the FilterMap

Before getting started, make sure you have [downloaded the Fluvio CLI] and followed
the getting started guide to get up and running with a Fluvio cluster. Then, if you
haven't done so already, you'll need to install the `wasm32-unknown-unknown` target
for Rust using the following command:

%copy first-line%
```bash
$ rustup target add wasm32-unknown-unknown
```

Now we'll be able to compile the FilterMap SmartModule. Let's use release mode so
we get the smallest WASM binary possible:

%copy first-line%
```bash
$ cargo build --release
```

Next, we'll need to create a new Fluvio topic to produce and consume our data using
this command:

%copy first-line%
```bash
$ fluvio topic create filter-map
topic "array-map" created
```

Now we can produce some test data to our topic.

%copy first-line%
```bash
$ fluvio produce filter-map
>
```

Finally, let's consume our data using our FilterMap SmartModule and see TODO

%copy first-line%
```bash
$ fluvio consume array-map -B --filter-map=target/wasm32-unknown-unknown/release/filter_map.wasm
```

Congratulations, you just completed your first FilterMap example! You can find the
[full source code for this example on GitHub], along with the full sources for many
other SmartModules examples.

### Read next

- [Explore map use-cases](https://www.infinyon.com/blog/2021/08/smartstream-map-use-cases/)
- [Writing a JSON filter]({{< ref "/docs/smartmodules/filter" >}})
- [Writing an aggregate to sum numbers]({{< ref "/docs/smartmodules/aggregate" >}})

[downloaded the Fluvio CLI]: https://www.fluvio.io/download/
[using ArrayMap to break apart paginated API requests]: https://infinyon.com/blog/2021/10/smartstream-array-map-reddit/
[full source code for this example on GitHub]: https://github.com/infinyon/fluvio/blob/095d8f0cbbcc79ebc71cea464cd653ffde7af4e0/crates/fluvio-smartstream/examples/array_map_json_array/src/lib.rs
