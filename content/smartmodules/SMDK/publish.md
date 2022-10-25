---
title: SMDK - Publish to Hub
menu: Publish
weight: 60
toc: false
---

SMDK load give developers the ability to publish their own SmartModules to the [Hub]. 

##### Prerequisites

This section assumes that SMDK is [installed] and `my-filter` project has been [built].

### Publish - Operation

Navigate to your `my-filter` directory and use the `publish` command:

%copy first-line%
```bash
$ smdk publish
Creating package aj/my-filter@0.1.0
.. fill out info in hub/package-meta.yaml
Package hub/my-filter-0.1.0.ipkg created
Package successfully uploaded
```

#### Inspect Result

Ensure that your SmartModule has been uploaded to the [Hub]:

%copy first-line%
```bash
$ fluvio hub list
  SMARTMODULE                    
  aj/my-filter@0.1.0                    
```

The SmartMoudule is now available for [download] by anyone with access to the `[Hub]`.

Congratulations :tada:!  You are now ready to build, test, and publish your own SmartModules.

### Next Steps

* Checkout SmartModule [APIs] build your data streaming SmartModules.

[Hub]: {{< ref "../hub/overview" >}}
[APIs]: {{< ref "../apis/overview" >}}
[installed]: {{< ref "install" >}}
[built]: {{< ref "build-test/#build---operation" >}}
[download]: {{< ref "../hub/download" >}}
