---
title: SmartModule Hub
menu: Hub
weight: 20
---

## Introduction to the SmartModule Hub Service

The SmartModule Hub offers ready-to-use packages for easy Fluvio pipeline construction, letting users focus on business logic. It provides tested modules with high-level configurable inputs, supporting a mix of Hub packages and custom SmartModules without requiring module rebuilds.

A view of available and installed SmartModules is accessible from both the `fluvio` cli.


## Fluvio Hub Smartmodule Commands

Commands for users to interact with the SmartModule hub are under the **fluvio hub smartmodule** set of commands.

- `fluvio hub smartmodule download group/package@version`
- `fluvio hub smartmodule list`

### `fluvio smartmodule hub download`

{{% inline-embed file="embeds/cli/help/fluvio-hub-smartmodule-download.md" %}}

The command **fluvio hub smartmodule download <group/package@version>** will download a hub smartmodule into your cluster so it can be configured and used in your pipeline.

%copy first-line%
```bash
$ fluvio hub smartmodule download infinyon/regex-filter2@0.1.0

downloading infinyon/regex-filter2@0.1.0 to infinyon-regex-filter2-0.1.0.ipkg
... downloading complete
... checking package
trying connection to fluvio router.infinyon.cloud:9003
... cluster smartmodule install complete

```

Once a package is downloaded, a fluvio smartmodule list command will show the installation

%copy first-line%
```bash
$ flvio sm list
  SMARTMODULE                   SIZE     
  infinyon/regex-filter@0.1.0   316.1 KB 
```

Once a SmartModule is download, it can be configured and applied in a pipeline. See the SmartModule page for more details on how work with SmartModules once they are downloaded in your cluster.


### `fluvio hub smartmodule list`

The command **fluvio hub smartmodule list** will list SmartModules and versions available for download

{{% inline-embed file="embeds/cli/help/fluvio-hub-smartmodule-list.md" %}}

The list command shows Smartmodules by **group/package@version** in the SMARTMODULE column. The Visibility column describes if the package is accessible to public or private views.  If package visibility is **private** only your user login can list or download the package. Packages uploaded by InfinyOn will be in the infinyon group.


%copy first-line%
```bash
$ fluvio hub smartmodule list 

  SMARTMODULE                  Visibility 
  mypriv/foo-priv@0.1.1        private    
  infinyon/jolt@0.1.0          public     
  infinyon/json-sql@0.1.0      public     
  infinyon/regex-filter@0.1.0  public

```


## SmartModule Packages

SmartModule Packages are listed in the hub as in the form of `<group>/<package name>@version`. 

Packages in the infinyon group are published by InfinyOn, the creators of Fluvio. Third-party developers can upload public packages under different groups. The Hub enforces signing and publishing restrictions on uploaded packages. Once a signed package is published in a group, only the group owner can update it or add new packages within that group.

### Private and Public Packages

By default, published packages are private. To make a package public, set it to public using the following CLI commands:

```bash
cd smartmodule-dev-dir
smdk set-public
```

Once set to public, you'll be prompted for verification on every publish. Note that once a package version is publicly published, it cannot be revoked.