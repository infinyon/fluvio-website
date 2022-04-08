---
title: Fluvio SmartModules
menu: Overview
weight: 10
aliases:
    - /docs/smartmodules
toc: false
---

SmartModules are one of Fluvio's premiere features, allowing users to have
direct control over their streaming data by providing a programmable API
for inline data manipulation.
SmartModules are user-defined functions, written in Rust and compiled to
<a href="https://webassembly.org/" target="_blank">WebAssembly</a> - that
means they're lightweight and portable, and enable enormous flexibility for
being integrated at many points in a streaming pipeline. They enhance the
entire streaming platform: reducing network costs by allowing precision 
filtering along the stream and encouraging code-reuse and collaboration.

The following diagram shows common components which may be configured with SmartModules
performing inline computation.

<img src="/docs/smartmodules/images/smartmodule-overview.svg" alt="SmartModule Overview" justify="center" height="480">

The diagram shows five places where SmartModules may currently be used:

- In **Source Connectors** and **Sink Connectors**, and
- in programmatic **Producers**, **Consumers**, and the **Fluvio CLI**

SmartModules are typically applied to data _before_ the data is sent from
one location to another. This is so that any filtering that happens will
result in less network traffic and more savings. For example, stream inputs
(i.e. Source Connectors and Producers) will apply SmartModules before
sending data to the Fluvio cluster, and stream outputs (such as Sink Connectors,
Consumers, and the Fluvio CLI) will upload SmartModules to the Fluvio cluster
and process data before the stream is sent over the network. This helps to
overcome "Data Gravity" by moving only the most minimal amount of data necessary.

## Types of SmartModules

Fluvio features the following types of SmartModules:

#### Filter

A [Filter SmartModule](../filter) takes an input record and returns `false` if the record should
be discarded, or `true` if the record should be kept and sent downstream.

<img src="/docs/smartmodules/images/smartmodule-filter.svg" alt="SmartModule Filter" justify="center" height="180">

#### Map

A [Map SmartModule](../map) takes an input record and may edit and transform it, returning a
new record that is sent downstream.

<img src="/docs/smartmodules/images/smartmodule-map.svg" alt="SmartModule Map" justify="center" height="180">

#### FilterMap

A [FilterMap SmartModule](../filter-map) takes one input record and returns zero or one output record.
This effectively means that it may decide to discard or "filter" any input records, or
to keep them and apply a transformation to them at the same time.

<img src="/docs/smartmodules/images/smartmodule-filtermap.svg" alt="SmartModule FilterMap" justify="center" height="180">

#### ArrayMap

An [ArrayMap SmartModule](../array-map) takes one input record and returns zero or many output records.
This means that the output stream may have more records than the input stream.

<img src="/docs/smartmodules/images/smartmodule-arraymap.svg" alt="SmartModule ArrayMap" justify="center" height="180">

#### Aggregate

An [Aggregate SmartModule](../aggregate) takes input records and "accumulates" them, essentially adding
each input record to a rolling sum. Each input record gets "added" to the sum after the
previous record, and the output stream is full of the "summed" records.

<img src="/docs/smartmodules/images/smartmodule-aggregate.svg" alt="SmartModule Aggregate" justify="center" height="220">


## Building SmartModules

Fluvio SmartModule generator creates a Cargo project with all the necessary boilerplate to get you up and running quickly. 

Install `cargo-generate` if you didn't install before (one time operation):

%copy first-line%
```bash
$ cargo install cargo-generate 
```

Run SmartModule template generator:

%copy first-line%
```bash
$ cargo generate --git https://github.com/infinyon/fluvio-smartmodule-template
⚠️   Unable to load config file: /.cargo/cargo-generate.toml
🤷   Project Name : my-smartmodule
🔧   Generating template ...
✔ 🤷   Which type of SmartModule would you like? · filter
✔ 🤷   Want to use SmartModule parameters? · true
[1/7]   Done: .cargo/config.toml
[2/7]   Done: .cargo
[3/7]   Done: .gitignore
[4/7]   Done: Cargo.toml
[5/7]   Done: README.md
[6/7]   Done: src/lib.rs
[7/7]   Done: src
🔧   Moving generated files into: `/my-smartmodule`...
💡   Initializing a fresh Git repository
✨   Done! New project created /my-smartmodule
```

Navigate to your SmartModule directory, make your changes, then compile:

%copy first-line%
```bash
$ cargo build --release
```

Now that we have the SmartModule binary compiled, let's see it in action.

## Using SmartModules

SmartModules may be used in two ways: by registering them by name with the Fluvio cluster, or by
providing them "ad hoc" right when you want to use them. In development, it is useful to use ad-hoc
SmartModules, since the compiled WASM code is changing between each run. However, for most actual
use-cases, it typically becomes much easier to register the SmartModule and later refer to them by name.

#### Registering SmartModules

After building a SmartModule as a WASM binary, it may be registered with Fluvio using
the `fluvio smart-module` command, providing a name and a path to the binary. Use [SmartModule filters](/docs/smartmodules/filter/) to build a WASM file.

%copy first-line%
```bash
$ fluvio smart-module create my-filter --wasm-file=target/wasm32-unknown-unknown/release/my_filter.wasm
```

After creating one or more SmartModules, one may use the `fluvio smart-module list` command
to see the available SmartModules:

%copy first-line%
```bash
$ fluvio smart-module list
 NAME       STATUS             SIZE
 my-filter  SmartModuleStatus  108754
```

Finally, to delete a SmartModule, use `fluvio smart-module delete` and provide the name of the
SmartModule to delete.

%copy first-line%
```bash
$ fluvio smart-module delete my-filter
```

#### Using Registered SmartModules

You may use a Registered SmartModule anywhere that SmartModules may be used. To use them,
you'll need to provide the name of the SmartModule as well as its type. For example, when
using a registered filter SmartModule with the Fluvio CLI Consumer, provide its name
to the `--filter` argument, like so:

%copy first-line%
```bash
$ fluvio consume my-topic -B --filter=my-filter
```

#### Using Ad-hoc SmartModules

You may still use SmartModules even without registering them with Fluvio. When using
SmartModules this way, you simply provide a path to the WASM file directly when you
use it. For example, when using an ad-hoc SmartModule with the Fluvio CLI Consumer,
you may pass a file path to `--filter` that points to the SmartModule WASM file,
like this:

%copy first-line%
```bash
$ fluvio consume my-topic -B --filter=target/wasm32-unknown-unknown/release/my_filter.wasm
```
