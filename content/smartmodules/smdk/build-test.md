---
title: SMDK - Build & Test a SmartModule
menu: Build & Test
weight: 50
toc: false
---

SmartModules are binaries developed for real-time data stream processing, requiring in-line and live testing facilities. This section covers in-line testing, and the [next section] describes how to load SmartModules on the Cluster for live traffic testing.

##### Prerequisites

This section assumes that SMDK is [installed] and `my-filter` project has been [generated].

### Build - Operation

Navigate to `my-filter` project directory and run `build`:

%copy first-line%
```bash
$ smdk build
...
Compiling my-filter v0.1.0 (~/smdk/my-filter)
Finished release-lto [optimized] target(s) in 12.65s
```

The build process generated an binary optimized for WebAssembly. We are now ready to test it.

### Test - Operation

When testing a SmartModule you can provide the test input via direct `text`, specifying a `file`,
or reading from the `stdin`.

Let's perform the matching test for our `my-filter` that looks for letter `a`:

%copy first-line%
```bash
$ smdk test --text cat
loading module at: ~/smdk/my-filter/target/wasm32-unknown-unknown/release-lto/my_filter.wasm
1 records outputed
cat
```

Next, a non-matching test, reading the input from the `stdin`:

%copy first-line%
```bash
$ echo -n dog | smdk test --stdin
loading module at: ~/smdk/my-filter/target/wasm32-unknown-unknown/release-lto/my_filter.wasm
0 records outputed
```

Checkout the `smdk test -h` for additional parameters. 

Checkout the [next section] on how upload the SmartModule to your local cluster or [InfinyOn Cloud] for live traffic testing.

### Steps

1. [Install SMDK]({{< ref "install" >}})
2. [Generate a SmartModule]({{< ref "generate" >}})
3. **[Build and Test]({{< ref "build-test" >}})**
4. [Load to your Cluster]({{< ref "load" >}})
5. [Publish to SmartModule Hub]({{< ref "publish" >}})

[next section]: {{< ref "load" >}}
[installed]: {{< ref "install" >}}
[generated]: {{< ref "generate" >}}
[Load to your Cluster]: {{< ref "load" >}}
[Publish to SmartModule Hub]: {{< ref "publish" >}}
[InfinyOn Cloud]: https://infinyon.cloud
