
---
title: SmartModule Development Kit (SMDK)
menu: Overview
toc: false
weight: 10
---

SmartModules Development Kit (SMDK) is an independent executable downloadable via [`Fluvio CLI`] to help developers build and test SmartModules, and publish them to the SmartModule Hub.

Currently, SMDK is limited to the Rust programming language. But you can be a Rust beginner and still take advantage of custom SmartModules.

##### Prerequisites

SMDK generator will prompt you to [`Install Rust`] language, add a Cargo target [`wasm32-unknown-unknown`] for compiling SmartModule Rust code into WebAssembly modules, and install [`cargo-generate`] for generating new Rust SmartModule projects.

### SMDK - Operations

SMDK helps developers build, test in-line, load to local or cloud Cluster, then publish to SmartModule Hub.

### Steps

1. **[Install SMDK]({{< ref "install" >}})**
2. [Generate a SmartModule]({{< ref "generate" >}})
3. [Build and Test]({{< ref "build-test" >}})
4. [Load to your Cluster]({{< ref "load" >}})
5. [Publish to SmartModule Hub]({{< ref "publish" >}})

[`Fluvio CLI`]: {{< ref "/cli/smartmodules/smdk" >}}
[`Install Rust`]: https://www.rust-lang.org/tools/install
[`wasm32-unknown-unknown`]: https://doc.rust-lang.org/rustc/platform-support/wasm64-unknown-unknown.html
[`cargo-generate`]: https://github.com/cargo-generate/cargo-generate
[Install SMDK]: {{< ref "install" >}}
[Generate a SmartModule]: {{< ref "generate" >}}
[Build and Test]: {{< ref "build-test" >}}
[Load to your Cluster]: {{< ref "load" >}}
[Publish to SmartMoudle Hub]: {{< ref "publish" >}}
