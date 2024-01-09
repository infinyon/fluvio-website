---
title: SmartModule Development Kit (SMDK)
menu: Overview
toc: false
weight: 10
---

SmartModules Development Kit (SMDK) is an independent executable downloadable via [`Fluvio CLI`] to help developers build and test SmartModules, and publish them to the SmartModule Hub.

Currently, SMDK is limited to the Rust programming language. However, you can be a Rust beginner and still take advantage of custom SmartModules.

##### Prerequisites

SMDK generator will prompt you to [`Install Rust`] on your system, add a Cargo target `wasm32-unknown-unknown` for compiling SmartModule Rust code into WebAssembly modules, and install [`cargo-generate`] for generating new Rust SmartModule projects.

### SMDK - Operations

SMDK helps developers build, test in-line, load to local or cloud Cluster, then publish to SmartModule Hub.

### Steps

1. [Generate a SmartModule]({{< ref "generate" >}})
2. [Build and Test]({{< ref "build-test" >}})
3. [Load to your Cluster]({{< ref "load" >}})
4. [Publish to SmartModule Hub]({{< ref "publish" >}})

[`Fluvio CLI`]: {{< ref "/cli/smartmodules/smdk" >}}
[`Install Rust`]: https://www.rust-lang.org/tools/install
[`cargo-generate`]: https://github.com/cargo-generate/cargo-generate
[Install SMDK]: {{< ref "install" >}}
[Generate a SmartModule]: {{< ref "generate" >}}
[Build and Test]: {{< ref "build-test" >}}
[Load to your Cluster]: {{< ref "load" >}}
[Publish to SmartModule Hub]: {{< ref "publish" >}}
