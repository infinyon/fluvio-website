---
title: SmartModule Hub
menu: Hub
weight: 20
---

SmartModules Development Kit (SMDK) allows data pipeline operators to build, compile and test SmartModules in their development environment. 


## Fluvio Hub Commands

Commands for users to interact with the SmartModule hub are under the **fluvio hub** set of commands.

### `fluvio hub list`

The command **fluvio hub list** will list SmartModules and versions available for download

```
$ fluvio hub list -h

List available SmartModules in the hub

Usage: fluvio-latest hub list [OPTIONS]

Options:
  -O, --output <type>  Output [default: table] [possible values: table, yaml, json]
  -h, --help           Print help information (use `--help` for more detail)
```

The list command shows Smartmodules by **group/package@version**. Packages uploaded by InfinyOn will be in the
infinyon group.

```
$ fluvio hub list 

SMARTMODULE                    
infinyon/json-sql@0.1.0        
infinyon/regex-filter2@0.1.0 
...

```

### `fluvio hub download`

The command **fluvio hub download <group/package@version>** will download a hub smartmodule into your cluster so it can be configured and used in your pipeline.

```
$ fluvio hub download infinyon/regex-filter2@0.1.0                                                                                     main
downloading infinyon/regex-filter2@0.1.0 to infinyon-regex-filter2-0.1.0.ipkg
... downloading complete
... checking package
trying connection to fluvio router.dev.infinyon.cloud:9003
... cluster smartmodule install complete

```

Once a package is downloaded, a fluvio smartmodule list command will show the installation

```
$ flvio sm list
  SMARTMODULE                   SIZE     
  infinyon/regex-filter@0.1.0   316.1 KB 
```

See the smartmodule page for more details on how work with smartmodules once they are downloaded in your cluster.



