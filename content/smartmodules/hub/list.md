---
title: Hub List
menu: List
weight: 20
toc: false
---

List operation retrieves all SmartModules available for download in the SmartModule Hub for a specific user.

By default, users can retrieve all public SmartModules and all private SmartModules that match their group assignment.

#### List with Fluvio CLI

Listing Hub SmartModule from the CLI is staright forward: 

%copy first-line%
```bash
$ fluvio hub list
  SMARTMODULE                  Visibility 
  infinyon/jolt@0.1.0          public     
  infinyon/json-sql@0.1.0      public     
  infinyon/regex-filter@0.1.0  public
  ...
```

[`InfinyOn Cloud`]: https://infinyon.cloud/
[`fluvio hub list`]: {{< ref "/cli/smartmodules/hub" >}}
[ Download Fluvio CLI ]: {{< ref "/cli/client/overview" >}}
[Provision InfinyOn Cloud Account]: https://infinyon.cloud/
[`Download`]: {{< ref "download" >}}
