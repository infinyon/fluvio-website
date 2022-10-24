---
title: Hub List
menu: List
weight: 20
toc: false
---

List operation retrieves all SmartModuels available for download in the SmarModule Hub for a specific user. By default, users can retrieve all public SmartModules and all private SmartModules that match their group assignment. Organization-level sharing rules will be available in future releases. 

##### Prerequisites

SmartModule Hub is powered by [`InfinyOn Cloud`], which requires a Cloud account and the CLI:

* [Provision InfinyOn Cloud Account]
* [Download Fluvio CLI] 

### List - Operation

There are two methods for listing SmartModules in SmartModule Hub: 
* [List with InfinyOn Cloud](#list-with-infinyon-cloud)
* [List with Fluvio CLI](#list-with-fluvio-cli)

#### List with InfinyOn Cloud

SmartModule Hub has a dedicated section in the InfinyOn Cloud interface.

1. Clck Hub on from top menu.
    
    All Public and Public-Owned SmartModules are displayed.

2. User Filters to choose a specific `Group` or `All`.

    Only SmartModules matching the filter are displayed.



#### List with Fluvio CLI

Listing Hub SmartModule from the CLI is staright forward: 

%copy first-line%
```bash
$ fluvio hub list
  SMARTMODULE                    
  infinyon/jolt@0.1.0            
  infinyon/regex-filter@0.1.2    
  infinyon/json-sql@0.1.0
  ...
```

### Next Steps

* [`Download`] SmartModules from the Hub.


[`InfinyOn Cloud`]: https://infinyon.cloud/
[`fluvio hub list`]: {{< ref "/cli/smartmodules/hub" >}}
[ Download Fluvio CLI ]: {{< ref "/cli/client/overview" >}}
[Provision InfinyOn Cloud Account]: https://infinyon.cloud/
[`Download`]: {{< ref "download" >}}