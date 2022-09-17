---
title: SmartModules
menu: Overview
toc: true
---

SmartModules are one of Fluvio's unique features, allowing users to have
direct control over their streaming data by providing a programmable API
for inline data manipulation.

SmartModules are user-defined functions, written in Rust and compiled to
<a href="https://webassembly.org/" target="_blank">WebAssembly</a> - that
means they're lightweight and portable, and enable enormous flexibility for
being integrated at many points in a streaming pipeline.

They enhance the entire streaming platform: reducing network costs by allowing precision 
filtering along the stream and encouraging code-reuse and collaboration.

The following diagram shows common components which may be configured with SmartModules
performing inline computation.

<img src="/smartmodules/images/smartmodule-overview.svg" alt="SmartModule Overview" justify="center" height="480">

The diagram shows five places where SmartModules may currently be used:

- In **Source Connectors** and **Sink Connectors**, and
- In programmatic **Producers**, **Consumers**, and the **Fluvio CLI**

SmartModules are typically applied to data _before_ the data is sent from
one location to another.

This is so that any filtering that happens will
result in _reduced_ network traffic and more savings.

* For Stream **inputs** (such as Source Connectors and Producers)
  * SmartModules will apply before sending data _to the Fluvio cluster_

* For Stream **outputs** (such as Sink Connectors, Consumers, and the Fluvio CLI)
  * SmartModules will apply before sending data _to the requester_

This helps to overcome "**Data Gravity**" by moving only the minimum amount of data necessary.

## Types of SmartModules

Fluvio supports the following types of SmartModules:

* [Filter]({{<ref "#filter" >}})
* [Map]({{<ref "#map" >}})
* [FilterMap]({{<ref "#filtermap" >}})
* [ArrayMap]({{<ref "#arraymap" >}})
* [Aggregate]({{<ref "#aggregate" >}})

### Transform

#### Filter

A [Filter SmartModule]({{<ref "transform/filter" >}}) takes an input record and allows you to check if the record value meets certain criteria.

If `false` the record is discarded, or `true` if the record is saved, and continues downstream.

<img src="/smartmodules/images/smartmodule-filter.svg" alt="SmartModule Filter" justify="center" height="180">

#### Map
A [Map SmartModule]({{<ref "transform/map" >}}) takes an input record allows you to apply any data transformations to the record before it continues downstream.

"Map" refers to the [programming language term](https://en.wikipedia.org/wiki/Map_(higher-order_function)), which simply is a function that is applied to all input data.


<img src="/smartmodules/images/smartmodule-map.svg" alt="SmartModule Map" justify="center" height="180">

#### FilterMap

A [FilterMap SmartModule]({{<ref "transform/filter-map" >}}) takes one input record and returns zero or one output record.

As the name may imply, FilterMap is the combination of [filter]({{<ref "#filter" >}}) and [map]({{<ref "#map" >}})

You can check for conditions in the data and if met, apply transformations. Or if the conditions are not met, discard the record.

<img src="/smartmodules/images/smartmodule-filtermap.svg" alt="SmartModule FilterMap" justify="center" height="180">

#### ArrayMap

An [ArrayMap SmartModule]({{<ref "transform/array-map" >}}) takes one input record and returns zero or many output records.

The Array in ArrayMap refers to [a JSON array](https://www.w3schools.com/js/js_json_arrays.asp).

Given a single record that is a JSON array, you may flatten the single input array. The result is the creation of several individual records, which you may additionally apply transformations before returning.

<img src="/smartmodules/images/smartmodule-arraymap.svg" alt="SmartModule ArrayMap" justify="center" height="180">

### Analytics

#### Aggregate

An [Aggregate SmartModule]({{<ref "analytics/aggregate" >}}) ability to create a feedback loop, by providing a Accumulator Record that is persistent per stream. Each event may modify the Accumulator, and the changes will be available for next input to use.

The value of the Accumulator Record is also returned after each input. 

For example, if you're trying to Sum up stream of numbers, you would add each input value to the current value of the Accumulator Record.

<img src="/smartmodules/images/smartmodule-aggregate.svg" alt="SmartModule Aggregate" justify="center" height="220">

## Build your own SmartModule

### Basic setup

Currently the SmartModule development is limited to the Rust programming language, but you can be a Rust beginner and still take advantage of custom SmartModules!

In addition to the [basic Rust development environment](https://www.rust-lang.org/tools/install), we need the following Rust tools installed:

* Cargo WebAssembly target: `wasm32-unknown-unknown`

This is needed to compile the SmartModule Rust code into a WebAssembly module

%copy first-line%
```shell
$ rustup target install wasm32-unknown-unknown
```

* [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)

This will be used for creating new Rust SmartModule projects

%copy first-line%
```shell
$ cargo install cargo-generate
```

### Create a new project

Using `cargo generate`, you can answer a few prompts and generate the code for a SmartModule

%copy first-line%
```bash
$ cargo generate gh:infinyon/fluvio-smartmodule-template
```

Example:

We are creating a `filter` type of SmartModule, named `my-filter`

%copy first-line%
```bash
$ cargo generate gh:infinyon/fluvio-smartmodule-template
🤷   Project Name : my-filter
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
🔧   Moving generated files into: `/home/User/my-filter`...
💡   Initializing a fresh Git repository
✨   Done! New project created /home/User/my-filter
```

Navigate to your SmartModule directory, make your changes, then compile:

%copy first-line%
```bash
$ cargo build --release
```

Now that we have the SmartModule binary compiled let's see it in action.

## Registering SmartModules with your cluster

After building a SmartModule as a WASM binary, it may be registered with Fluvio using
the `fluvio smart-module` command, providing a name and a path to the binary.

Use [SmartModule filters]({{<ref "transform/filter" >}}) to build a WASM file.

%copy first-line%
```bash
$ fluvio smart-module create my-filter --wasm-file ./target/wasm32-unknown-unknown/release/my_filter.wasm
```

After creating one or more SmartModules, one may use the `fluvio smart-module list` command
to see the available SmartModules:

%copy first-line%
```bash
$ fluvio smart-module list
   NAME       STATUS             SIZE
   my-filter  SmartModuleStatus  90442
```

## Using SmartModules

### SmartModules with Consumers

#### Using Registered SmartModules
You may use a Registered SmartModule anywhere that SmartModules may be used. To use them,
you'll need to provide the name of the SmartModule as well as its type. 

For example, if we want to apply our registered SmartModule `my-filter` while consuming from our topic `my-topic`,
provide it's name to the `--filter` argument.

%copy first-line%
```bash
$ fluvio consume my-topic -B --filter my-filter
```

#### Advanced: Using SmartModules without registering

During the development you may find it convenient for testing to skip the SmartModule registration step.

With the Fluvio CLI Consumer, you may pass a file path to `--filter` that points to the SmartModule WASM file,
like this:

%copy first-line%
```bash
$ fluvio consume my-topic -B --filter=target/wasm32-unknown-unknown/release/my_filter_in_development.wasm
```

### In Connectors

For our [official source and sink connectors]({{<ref "/connectors/">}}) you can apply SmartModules can be applied to any `source` or `sink` connector.

You just need to provide the type of module (`filter`, `map`, `array-map`, `filter-map`, `aggregate`) and it's registered name as a parameter.

For example, this is how you would define a `filter` type SmartModule named `my-filter` to use with our [http source connector]({{<ref "/connectors/sources/http">}}), to apply the filter to each HTTP response before publishing to the topic:

%copy%
```yaml
# cat-facts-connector.yml
name: cat-facts-connector
type: http
topic: cat-facts 
create_topic: true
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 30
  filter: my-filter 
```
