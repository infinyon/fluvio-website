---
title: Connectors Hub
menu: Hub
weight: 20
---

## Introduction to the Connectors Hub Service

The SmartConnectors Hub allows users to start building their data streaming pipeline with SmartConnectors which have been uploaded into the Hub as ready to use packages. The Hub allows Fluvio pipeline construction focusing on the business logic by providing already built, reliable and tested modules. The modules contain high level configurable inputs which can be applied without needing to rebuild the module. Pipelines can employ a mix of Hub downloaded packages as well as custom developed SmartModules.

A view of available and installed SmartModules is accessible from both the `fluvio` cli as well as on the Fluvio cloud dashboard.


## Fluvio Hub Commands

Commands for users to interact with the SmartModule hub are under the **fluvio hub connector** set of commands.

- `fluvio hub connector download group/package@version`
- `fluvio hub connector list`

### `fluvio hub connector download`

{{% inline-embed file="embeds/cli/help/fluvio-hub-connector-download.md" %}}

The command **fluvio hub connector download <group/package@version>** will download a hub SmartConnector `.ipkg` locally, which can be used with `cdk`.

%copy first-line%
```bash
$ % fluvio hub connector download infinyon/mqtt-source@0.1.2
downloading infinyon/mqtt-source@0.1.2 to infinyon-mqtt-source-0.1.2.ipkg
... downloading complete
```

### `fluvio hub connector list`

The command **fluvio hub connector list** will list Connectors and versions available for download

{{% inline-embed file="embeds/cli/help/fluvio-hub-connector-list.md" %}}

The list command shows Connectors by **group/package@version** in the CONNECTOR column. The Visibility column describes if the package is accessible to public or private views.  If package visibility is **private** only your user login can list or download the package. Packages uploaded by InfinyOn will be in the infinyon group.


%copy first-line%
```bash
$ fluvio hub connector list
Using hub https://hub.infinyon.cloud
  CONNECTOR                    Visibility 
  infinyon/http-source@0.1.1   public     
  infinyon/kafka-sink@0.1.1    public     
  infinyon/kafka-source@0.4.0  public     
  infinyon/mqtt-source@0.1.2   public     
  infinyon/sql-sink@0.1.1      public
```

## Connector Packages

Connector Packages are listed in the hub as in the form of `<group>/<package name>@version`. 

Group `infinyon` packages are published by InfinyOn, the maker of Fluvio.  Third-party developers may also upload public packages which appear in other groups. The Hub service enforces signing and publishing restrictions on uploaded packages. Once a signed package of given group has been published, only the group owner may update that package or add new packages within that group.

### Private and Public Packages

Published packages are by default private. In order to make a published package publicly available, the package should be set to public. The cli method to do this is below:

```bash
cd connector-dev-dir
cdk set-public
```

After the package is set to public, any publish will ask to verify the publish operation. Once a package version is publicly published, the package cannot revoked.