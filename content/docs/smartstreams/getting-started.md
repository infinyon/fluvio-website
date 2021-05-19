---
title: Fluvio SmartStreams
menu: Getting Started
weight: 10
---

SmartStreams are Fluvio data streams that can be customized with user-defined
functions to perform inline operations such as filtering. SmartStreams are compiled
WASM modules that are uploaded by clients to the Fluvio cluster, where they are
applied to streams of records.

Currently, SmartStreams are "consumer facing", meaning that all records that are
produced to a topic are stored as-is, and SmartStreams may be applied by consumers.
When a consumer applies a SmartStream to a topic, they receive a stream of records
that have been processed by the SmartStream on the server side (however, the records
persisted in the Topic are not modified). For use-cases like
filtering, this means that consumers can define functions to select only particular
records from the topic that they are interested in, and save bandwidth and latency
by not over-fetching records they don't need.

We've put together some `cargo-generate` templates for creating SmartStreams. To
get started with your own SmartStreams, you can run the following:

```bash
$ cargo install cargo-generate
$ cargo generate --git https://github.com/infinyon/fluvio-smartstream-template
```

The `cargo generate` command will prompt you about which type of SmartStream you'd
like to create. Based on the use-cases you're interested in, you'll want to read more
about that type of SmartStream in the next pages of this section. However, regardless
of the type of SmartStream you're working with, you'll need to know how to build and
run SmartStreams, which we'll talk about next.

#### Building SmartStreams

If you used the cargo-generate template, you should be all set to build the
SmartStream using cargo. We recommend using release mode in order to produce a
smaller, more efficient binary.

```bash
$ cargo build --release
```

If you are not using the template, then there are a few configurations you will need
to set up in your project. A few big points are:

- Setting the build target to `wasm32-unknown-unknown`
- Configuring the project as a `cdylib` to produce a `.wasm` file

You can accomplish these by creating a `.cargo/config.toml` and editing `Cargo.toml`:

```toml
# .cargo/config.toml
[build]
target = "wasm32-unknown-unknown"
```

```toml
# Cargo.toml
[package]
# ...

[lib]
crate-type = ["cdylib"]
```

After applying these configuration changes, running a cargo build should produce a
`.wasm` file in your target directory.

```bash
$ cargo build --release
$ ls -la target/wasm32-unknown-unknown/release/your-package-name.wasm
.rwxr-xr-x  1.5Mi user 19 May 10:07   your-package-name.wasm
```

#### Applying SmartStreams

Once we have a built `.wasm` file, we can provide it to Fluvio using the
[`fluvio consume`] command. This command optionally accepts a `--smart-stream`
argument which you can use to specify the path to your SmartStream module.

[`fluvio consume`]: /cli/commands/consume

```bash
$ fluvio consume my-topic --smart-stream=target/wasm32-unknown-unknown/release/your-package-name.wasm
```

At this point, your CLI consumer will now be consuming records that have had your
SmartStream applied to them. In the next handful of sections, we will dive deeper into
the available types of SmartStreams and get more concrete and hands-on about writing
custom SmartStreams.

### Read next

- [Writing a filter SmartStream](/docs/smartstreams/filter)

[comment]: <> (- SmartStreams are a feature of Fluvio that allow you dynamically edit streams inline)
[comment]: <> (- SmartStream modules are WASM code that can filter or transform records in a topic)
[comment]: <> (- Users write custom WASM SmartStreams to send to the cluster and execute inline)
[comment]: <> (- There are two primary types of SmartStreams, filters and maps)
[comment]: <> (- The easiest way to write SmartStreams is using our Rust project template)
[comment]: <> (- As of today, SmartStreams are applied at consume-time)
[comment]: <> (  - In the future, they may be applied at produce-time?)
