---
title: SmartModule
weight: 10
---

The `fluvio smartmodule` family of commands is used to create and delete SmartModules,
as well as to view basic information about existing SmartModules.

## `fluvio smartmodule create`

This command is used to create new Fluvio SmartModules. A SmartModule must be given
a name and provided with a path to a WASM binary which will be uploaded and used for
the SmartModule.

{{% inline-embed file="embeds/cli/help/fluvio-smartmodule-create.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio smartmodule create json-filter --wasm-file=target/wasm32-unknown-unknown/release/json_filter.wasm
```

---

## `fluvio smartmodule list`

This command shows all the registered SmartModules in your cluster.

{{% inline-embed file="embeds/cli/help/fluvio-smartmodule-list.md" %}}

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

{{% inline-embed file="embeds/cli/help/fluvio-smartmodule-delete.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio smartmodule delete json-filter
```