---
title: SmartModule
weight: 10
---

The `fluvio smart-module` family of commands is used to create and delete SmartModules,
as well as to view basic information about existing SmartModules.

## `fluvio smart-module create`

This command is used to create new Fluvio SmartModules. A SmartModule must be given
a name and provided with a path to a WASM binary which will be uploaded and used for
the SmartModule.

```
fluvio-smart-module-create
Create a new SmartModule with a given name

USAGE:
    fluvio smart-module create [OPTIONS] <name> --wasm-file <wasm-file>

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
$ fluvio smart-module create json-filter --wasm-file=target/wasm32-unknown-unknown/release/json_filter.wasm
```

---

## `fluvio smart-module list`

This command shows all the registered SmartModules in your cluster.

```
fluvio-smart-module-list 0.0.0
List all existing SmartModules

USAGE:
    fluvio smart-module list [OPTIONS]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -O, --output <type>    Output [default: table]  [possible values: table, yaml, json]
```

Example usage:

%copy first-line%
```bash
$ fluvio smart-module list
 NAME          STATUS             SIZE
 json-filter   SmartModuleStatus  142843
```

---

## `fluvio smart-module delete`

This command will delete an existing SmartModule by name.

```
fluvio-smart-module-delete 0.0.0
Delete an existing SmartModule with the given name

USAGE:
    fluvio smart-module delete <name>

FLAGS:
    -h, --help    Prints help information

ARGS:
    <name>
```

Example usage:

%copy first-line%
```bash
$ fluvio smart-module delete json-filter
```