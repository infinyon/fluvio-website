---
title: SmartModule
weight: 30
---

The `fluvio smartmodule` family of commands is used to create and delete SmartModules,
as well as to view basic information about existing SmartModules.

## `fluvio smartmodule create`

This command is used to create new Fluvio SmartModules. A SmartModule must be given
a name and provided with a path to a WASM binary which will be uploaded and used for
the SmartModule.

```
fluvio-smartmodule-create
Create a new SmartModule with a given name

USAGE:
    fluvio smartmodule create [OPTIONS] <name> --wasm-file <wasm-file>

FLAGS:
    -h, --help    Prints help information

OPTIONS:
        --wasm-file <wasm-file>        The path to a WASM binary to create the SmartModule from

ARGS:
    <name>    The name of the SmartModule to create
```

Example usage:

%copy first-line%
```bash
$ fluvio smartmodule create json-filter --wasm-file=target/wasm32-unknown-unknown/release/json_filter.wasm
```

---

## `fluvio smartmodule list`

This command shows all the registered SmartModules in your cluster.

```
fluvio-smartmodule-list
List all existing SmartModules

USAGE:
    fluvio smartmodule list [OPTIONS]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -O, --output <type>    Output [default: table]  [possible values: table, yaml, json]
```

Example usage:

%copy first-line%
```bash
$ fluvio smartmodule list
 NAME          STATUS             SIZE
 json-filter   SmartModuleStatus  142843
```

---

## `fluvio smartmodule delete`

This command will delete an existing SmartModule by name.

```
fluvio-smartmodule-delete 0.0.0
Delete an existing SmartModule with the given name

USAGE:
    fluvio smartmodule delete <name>

FLAGS:
    -h, --help    Prints help information

ARGS:
    <name>
```

Example usage:

%copy first-line%
```bash
$ fluvio smartmodule delete json-filter
```