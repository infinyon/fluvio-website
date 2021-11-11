---
title: Overview
weight: 10
hidden: true
---

Writing a new connector is meant to be easy. You may write your connector in
any language which has a [fluvio client library](/api). You should simply
follow a few conventions and will gladly add and maintain a connector in our
official catalog.

## Adding a new connector

A new connector should work as a stand alone program in development but when
merged into the [`fluvio-connectors`
repo](https://github.com/infinyon/fluvio-connectors/), this will be built into
a docker image. It is recommended to write your connector in Rust and add a new
[package in the
workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html#creating-the-second-package-in-the-workspace)
in the
[`rust-connectors/sources`](https://github.com/infinyon/fluvio-connectors/tree/main/rust-connectors/sources)
or `rust-connectors/sinks` directories.

### `metadata` subcommand

A connector should have a `metadata` command which prints a [json
schema](https://json-schema.org/) of the commandline arguments. This is the
command we use to build our connectors library and validate arguments passed to
a connector.

This `metadata` subcommand should print to stdout something of the following:
```json
{
    "name": "test-connector",
    "direction": "Source",
    "schema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "TestConnectorOpts",
        "type": "object",
        "properties": {
            "count": {
                "description": "The maximum number of records to send.",
                "type": [
                    "integer",
                "null"
                ],
                "format": "int64"
            },
        }
    },
    "version": "0.1.0",
    "description": "This is a description of our new connector"
}
```

With our connectors written in rust, we have a
[`fluvio-connectors-common`](https://github.com/infinyon/fluvio-connectors/tree/main/rust-connectors/common)
which has commonly used options.

The fields in a metadata json object should all be generated from various
attributes in the project.

In the case of our [mqtt rust
connector](https://github.com/infinyon/fluvio-connectors/blob/c674c960cb3ddef265c7ff34afc0ec8bfc4adb47/rust-connectors/sources/mqtt/src/main.rs#L55-L63)
we do the following:

```rust
let schema = schema_for!(MqttOpts);
let mqtt_schema = MySchema {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
    description: env!("CARGO_PKG_DESCRIPTION"),
    direction: "source"
    schema,
};
println!("{}", serde_json::to_string(&mqtt_schema).unwrap());
```

Our CI will take this `metadata` command, test that it fits the schema and when
merged, will generate a catalog of the connectors.

If you'd like to write to do something other than a `metadata` subcommand in
the executable having a `metadata` make rule in the connector directory is also
fine.

### Integration tests

A given connector must have a `Makefile` and at least have a `test` rule in it.
How integration tests are done, is up to the author of the connector however
we have used [`bats`](https://github.com/bats-core/bats-core) in [our `http`
connector](https://github.com/infinyon/fluvio-connectors/blob/c674c960cb3ddef265c7ff34afc0ec8bfc4adb47/rust-connectors/sources/http/Makefile#L1-L4).

Our continuous integration will run `make -C ./path-to-your-connector/Makefile
test` on each pull request.


### Custom build steps

Should your connector require special build steps such as depending on a C
static library, we'd ask you to have `build` make rule which handles these
parts.
