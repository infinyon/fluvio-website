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

{{% inline-embed file="embeds/cli/help/fluvio-hub-download.md" %}}

The command **fluvio hub download <group/package@version>** will download a hub smartmodule into your cluster so it can be configured and used in your pipeline.

%copy first-line%
```bash
$ fluvio hub download infinyon/egex-filter2@0.1.0

downloading infinyon/regex-filter2@0.1.0 to infinyon-regex-filter2-0.1.0.ipkg
... downloading complete
... checking package
trying connection to fluvio router.dev.infinyon.cloud:9003
... cluster smartmodule install complete

```

Once a package is downloaded, a fluvio smartmodule list command will show the installation

%copy first-line%
```bash
$ flvio sm list
  SMARTMODULE                   SIZE     
  infinyon/regex-filter@0.1.0   316.1 KB 
```

Once a SmartModule is download, it can be configured and applied in a pipleine. See the SmartModule page for more details on how work with SmartModules once they are downloaded in your cluster.


### `fluvio hub list`

The command **fluvio hub list** will list SmartModules and versions available for download

{{% inline-embed file="embeds/cli/help/fluvio-hub-list.md" %}}

The list command shows Smartmodules by **group/package@version** in the SMARTMODULE column. The Visibility column describes if the package is accessible to
to the public or private.  If package visibility is **private** only your user login can list or download the package. Packages uploaded by InfinyOn will be in the infinyon group.


%copy first-line%
```bash
$ fluvio hub list 

  SMARTMODULE                  Visibility 
  mypriv/foo-priv@0.1.1        private    
  infinyon/jolt@0.1.0          public     
  infinyon/json-sql@0.1.0      public     
  infinyon/regex-filter@0.1.0  public
...

```


## SmartModule Packages

SmartModule Packages are listed in the hub as in the form of `<group>/<package name>@version`. 

Group `infinyon` packages are published by InfinyOn, the maker of Fluvio.  Third-party developers may also upload public packages which appear in other groups. The Hub service enforces signing and publishing restrictions on uploaded packages. Once a signed package of given group has been published, only the original owner of that package will be accepted into the same group.

### Private and Public Packages

Published packages are by default private. In order to make a published package publically available, the package should be set to public. The cli method to do this is below:

```bash
cd smartmodule-dev-dir
smdk set-public
```

After the package is set to public, any publish will ask to verify the publish operation. Once a package version is publically published, the package cannot revoked.




