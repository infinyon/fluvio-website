---
title: Map
weight: 30
toc: false
---

SmartModule Maps are used to transform or edit each Record in a stream.
We say that these SmartModules "map" each input record into a new output
record by applying a function to the input data. This type of SmartModule
may be used for many use-cases, such as:

- Narrowing large records into a smaller subset of important fields
- Scrubbing sensitive fields of data to be invisible to downstream consumers
- Computing rich, derived fields from simple raw data

Let's create a brand-new SmartModule Map to see what a minimal working
example looks like.

### Create a new Project

Let's use `cargo-generate` to set up a blank SmartModule Map project. If you
don't have it yet, run the following command to install it:

%copy first-line%
```bash
$ cargo install cargo-generate
```

Then, run the following command and be sure to select the `map` option.

%copy first-line%
```bash
$ cargo generate --git https://github.com/infinyon/fluvio-smartmodule-template
🤷   Project Name : map-example
🔧   Creating project called `map-example`...
🤷   Which type of SmartModule would you like? [filter, map] [default: filter]: map
[1/5]   Done: .cargo/config.toml
[2/5]   Done: .gitignore
[3/5]   Done: Cargo.toml
[4/5]   Done: README.md
[5/5]   Done: src/lib.rs
✨   Done! New project created map-example
```

We should see a new folder has been created for our project, `map-example`. We
can go inside and take a look at the sample Map generated for us by the template:

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

This template SmartModule will parse each record as an `i32` integer, then
multiply that value by 2. To test it out, make sure you are connected to an
active Fluvio Cluster (see the getting started sections in the top left), then
follow the instructions in the next section:

### How to run a SmartModule Map with a consumer

Create a new Topic:

%copy first-line%
```bash
$ fluvio topic create map-double
topic "map-double" created
```

Build the SmartModule WASM module. In your project folder, run:

%copy first-line%
```bash
$ cargo build --release
```

Your WASM binary is now ready for use.

-> You may need to run **% rustup target add wasm32-unknown-unknown**

Now, open a consumer and use the `--map` flag to point it to your WASM module:

%copy first-line%
```bash
$ fluvio consume map-double -B --map=target/wasm32-unknown-unknown/release/map_example.wasm
Consuming records from the beginning of topic 'map-double'
```

This command will stay open, waiting for records to arrive on the `map-double` topic.
Now let's open a new terminal and produce some data to watch this work end-to-end.

In a new terminal, run the following:

%copy first-line%
```bash
$ fluvio produce map-double
> 1
> 2
> 3
> 4
> 5
> ^C
```

When the prompt `>` appears, the data you type on each line will be sent to the topic.
If we check on our consumer window, we should see each record get doubled and printed out:

%copy first-line%
```bash
$ fluvio consume map-double -B --map=target/wasm32-unknown-unknown/release/map_example.wasm
Consuming records from the beginning of topic 'map-double'
2
4
6
8
10
```

### Read next

- [Explore map use-cases](https://www.infinyon.com/blog/2021/08/smartstream-map-use-cases/)
- [Writing a JSON filter]({{< ref "/docs/smartmodules/filter" >}})
- [Writing an aggregate to sum numbers]({{< ref "/docs/smartmodules/aggregate" >}})
