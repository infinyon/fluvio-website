---
title: Hub Download
menu: Download
weight: 30
toc: false
---

Download operation sends a copy of the SmartModule from the Hub to the local or Cloud cluster, defined by [`the profile`] in the Fluvio Client.

After downloading, all SmartModules, regardless of their source, are treated the same.

#### Download with Fluvio CLI

Downloading a SmartModule From the Hub is as follows: 

%copy first-line%
```bash
$ fluvio hub download infinyon/json-sql@0.1.0 
trying connection to fluvio router.infinyon.cloud:9003
downloading infinyon/json-sql@0.1.0 to infinyon-json-sql-0.1.0.ipkg
... downloading complete
... checking package
... cluster smartmodule install complete
```

#### Download with InfinyOn Cloud

 In InfinyOn Cloud, the Hub collapses multiple versions of the same SmartModule in one object. For example:

```
    infinyon/regex-filter
        -> `infinyon/regex-filter@0.1.0`
        -> `infinyon/regex-filter@0.1.2`
        -> `infinyon/regex-filter@0.2.1`
        ...
```

1. Click Hub from the top menu.
    All Public and Public-Owned SmartModules are displayed.

3. Choose the SmartModule you want to download, pick a version, and click Download
    The SmartModule is downloaded to your cluster and the state of the button changes.


[`the profile`]: {{< ref "/cli/client/profile" >}}
[`InfinyOn Cloud`]: https://infinyon.cloud/
[`fluvio hub list`]: {{< ref "/cli/smartmodules/hub" >}}
[ Download Fluvio CLI ]: {{< ref "/cli/client/overview" >}}
[ Provision InfinyOn Cloud Account ]: https://infinyon.cloud/