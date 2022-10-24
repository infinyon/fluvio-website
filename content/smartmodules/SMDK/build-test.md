---
title: SMDK - Build & Test a SmartModule
menu: Build & Test
weight: 40
toc: false
---

SmartModules are binaries developed for real-time data stream processing, requiring in-line and live testing facilities. This section covers in-line testing, and the [next section] describes how to load SmartMoudules on the Cluster for live traffic testing.

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

Testing a SmartModule takes `text` of `files` from the commnad line. 

Let's perform the matching test for our `my-filter` that looks for letter `a`:

%copy first-line%
```bash
$ smdk test --text cat
loading module at: ~/smdk/my-filter/target/wasm32-unknown-unknown/release-lto/my_filter.wasm
1 records outputed
cat
```

Next, an non-matching test:

%copy first-line%
```bash
$ smdk test --text dog
loading module at: ~/smdk/my-filter/target/wasm32-unknown-unknown/release-lto/my_filter.wasm
0 records outputed
```

Checkout the `smdk test -h` for additional parameters. 

Checkout the [next section] on how upload the SmartModule to your local cluster or [InfinyOn Cloud] for live traffic testing.

### Next Steps

4. [Load to your Cluster]
5. [Publish to SmartMoudle Hub]


[next section]: {{< ref "load" >}}
[installed]: {{< ref "install" >}}
[generated]: {{< ref "generate" >}}
[Load to your Cluster]: {{< ref "load" >}}
[Publish to SmartMoudle Hub]: {{< ref "publish" >}}
[InfinyOn Cloud]: https://infinyon.cloud