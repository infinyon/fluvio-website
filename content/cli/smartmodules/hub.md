---
title: SmartModule Hub
menu: Hub
weight: 20
---

## Introduction to the SmartModule Hub Service

The SmartModule Hub allows users to start building their data streaming pipline with SmartModules which have been uploaded into the Hub as ready to use packages. The Hub allows Fluvio pipeline construction focusing on the business logic by providing already built, reliable and tested modules. The modules contain high level configurable inputs which can be applied without needing to rebuild the module. Pipelines can employ a mix of Hub downloaded packages as well as custom developed SmartModules.

A view of available and installed SmartModules is accessible from both the fluvio cli as well as on the Fluvio cloud dashboard.


## Fluvio Hub Commands

Commands for users to interact with the SmartModule hub are under the **fluvio hub** set of commands.

- `fluvio hub download group/package@version`
- `fluvio hub list`

### `fluvio hub download`

The command **fluvio hub download <group/package@version>** will download a hub smartmodule into your cluster so it can be configured and used in your pipeline.

```
$ fluvio hub download infinyon/egex-filter2@0.1.0

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

Once a SmartModule is download, it can be configured and applied in a pipleine. See the SmartModule page for more details on how work with SmartModules once they are downloaded in your cluster.


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


## SmartModule Packages

SmartModule Packages are listed in the hub as in the form of `<group>/<package name>@version`. 

Group `infinyon` packages are published by InfinyOn, the maker of Fluvio.  Third-party developers may also upload public packages which appear in other groups. The Hub service enforces signing and publishing restrictions on uploaded packages. Once a signed package of given group has been published, only the original signing key will be accepted for other packages in the same group.

Currently all packages are public, with private package upload is in development.




