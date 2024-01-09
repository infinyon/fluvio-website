---
title: Using Connectors with SmartModules
weight: 10
hidden: true
_build:
  render: never
---
## SmartModules

{{<caution>}}
Changes to this process are coming soon, so that Connectors can take advantage of new SmartModules features 
{{</caution>}}

Fluvio's official connectors have support for applying SmartModules to perform inline
compute on the data passing through - when used together this way, we call them
"Smart Connectors". One of the reasons Smart Connectors are so beneficial is because
they can help save streaming costs. For example, when using an Inbound  connector to
stream data from a third-party data platform, you may only be interested in receiving
a subset of the available data. With Smart Connectors, you can write custom logic to
filter out irrelevant data _before_ it gets sent over the network and persisted in
your topic, saving on bandwidth and storage.

### Create a SmartModule

Let's create a new SmartModule that we can use with the Http Connector to pre-process
our Cat Facts. From the examples above, we know our raw input records from the API look
like this:

```json
{"length":116,"fact":"A cat almost never meows at another cat, mostly just humans. Cats typically will spit, purr, and hiss at other cats."}
```

It would be nice to remove the `length` field since it is redundant, and make our fact
a top-level string, like this:

```json
"A cat almost never meows at another cat, mostly just humans. Cats typically will spit, purr, and hiss at other cats."
```

To create this SmartModule, we can use `cargo-generate` to start a project with a template
to help us get started. You can install `cargo-generate` with the following command:

%copy first-line%
```bash
$ cargo install cargo-generate
```

Once we have it, we can use it as follows to create our SmartModule:

%copy first-line%
```bash
$ cargo generate --git="https://github.com/infinyon/fluvio-smartmodule-template"
⚠️   Unable to load config file: ~/.cargo/cargo-generate.toml
🤷   Project Name : catfact-map
🔧   Generating template ...
✔ 🤷   Which type of SmartModule would you like? · map
[1/7]   Done: .cargo/config.toml
[2/7]   Done: .cargo
[3/7]   Done: .gitignore
[4/7]   Done: Cargo.toml
[5/7]   Done: README.md
[6/7]   Done: src/lib.rs
[7/7]   Done: src
🔧   Moving generated files into: `catfact-map`...
✨   Done! New project created catfact-map
```

Make sure to navigate into the project directory:

%copy first-line%
```bash
$ cd catfact-map
```

Now, let's write the actual body of the SmartModule. Edit your `src/lib.rs` file
to have the following contents:

%copy%
```rust
use fluvio_smartmodule::{smartmodule, Result, SmartModuleRecord, RecordData};
use serde_json::Value;

#[smartmodule(map)]
pub fn map(record: &SmartModuleRecord) -> Result<(Option<RecordData>, RecordData)> {
    let input: Value = serde_json::from_slice(record.value.as_ref())?;
    let fact = &input["fact"];
    let output = serde_json::to_string(fact)?;

    Ok((record.key.clone(), output.into()))
}
```

Here, we're simply parsing the input as JSON and extracting the `fact` field from
the object.

Next, we need to build the SmartModule and register it with Fluvio so that our
connector will be able to find it. To build it, use the following command:

%copy first-line%
```bash
$ cargo build --release
```

Then to register the SmartModule with Fluvio, use this command:

%copy first-line%
```bash
$ fluvio smartmodule create catfact-map --wasm-file=target/wasm32-unknown-unknown/release/catfact_map.wasm
```

### Apply to Cloud Connectors

Launching a Smart Cloud Connector only requires updating the `connect.yml` configuration.
For this example, we would add `map` to the `parameters` section, like so:

%copy%
```yaml
# connect.yml
version: 0.4.3
name: cat-facts
type: http-source
topic: cat-facts
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10s
  map: "catfact-map"
```

Followed by launching it with `fluvio cloud connector`:

%copy first-line%
```bash
$ fluvio cloud connector create --config=./connect.yml
```




