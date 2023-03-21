---
title: Build & Test 
weight: 40
---
### 1.1. Build

Use `cdk build` to build a new connector:

```bash
$ cdk build --help
Build the Connector in the current working directory

Usage: cdk build [OPTIONS] [-- <EXTRA_ARGUMENTS>...]

Arguments:
  [EXTRA_ARGUMENTS]...  Extra arguments to be passed to cargo

Options:
      --release <RELEASE>            Release profile name [default: release]
  -p, --package-name <PACKAGE_NAME>  Optional package/project name
  -h, --help                         Print help
```

It compiles and builds a binary of the new Connector. The built binary is located in the default Cargo’s target directory.

