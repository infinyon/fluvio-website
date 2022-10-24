
---
title: SmartModule Development Kit (SMDK)
menu: Overview
toc: true
weight: 10
---

SmartModules Development Kit (SMDK) is an independent command line downloadable via [`Fluvio CLI`] to help developers build and test SmartModules, and publish them to the SmartModule Hub. Currently the SmartModule development is limited to the Rust programming language, but you can be a Rust beginner and still take advantage of custom SmartModules!

##### Presequisites

SMDK generator will prompt you to [`Install Rust`] language, add a Cargo target [`wasm32-unknown-unknown`] for compiling SmartModule Rust code into WebAssembly modules, and install [`cargo-generate`] for generating new Rust SmartModule projects. If you want to be proactive about it, install them now, it's a one-time setup.

### Install SMDK
....

Typical Journey:
* Generate -> build -> test -> load

Another
* Generate -> bild -> test - publish

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

Use [SmartModule filters]({{<ref "/smartmodules/apis/filter" >}}) to build a WASM file.

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

For example, this is how you would define a `filter` type SmartModule named `my-filter` to use with our [http source connector]({{<ref "/connectors/inbound/http">}}), to apply the filter to each HTTP response before publishing to the topic:

%copy%
```yaml
# connect.yml
version: 0.3.0
name: cat-facts
type: http-source
topic: cat-facts
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 30s
  filter: my-filter 
```

[`Fluvio CLI`]: {{< ref "/cli/smartmodules/smdk" >}}
[`Install Rust`]: https://www.rust-lang.org/tools/install
[`wasm32-unknown-unknown`]: https://doc.rust-lang.org/rustc/platform-support/wasm64-unknown-unknown.html
[`cargo-generate`]: https://github.com/cargo-generate/cargo-generate