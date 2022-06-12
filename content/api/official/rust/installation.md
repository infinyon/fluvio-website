---
title: Rust SDK Installation Guide
menu: Installation guide
weight: 10
---

## Install Rust development environment

The only requirement is to [install a basic Rust development environment](https://www.rust-lang.org/tools/install)

## Add [`fluvio`] crate

Then add [`fluvio`] under `[dependencies]` to your project's `Cargo.toml` 

%copy%
```toml
fluvio = "0.12"
```

[`fluvio`]: https://crates.io/crates/fluvio