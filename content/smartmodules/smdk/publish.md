---
title: SMDK - Publish to Hub
menu: Publish
weight: 70
toc: false
---

SMDK publish give developers the ability to publish their own SmartModules to the [Hub]. 

This section assumes that `my-filter` project has been [built].

### Publish - Operation

From your `my-filter` directory and use the `publish` command:

%copy first-line%
```bash
$ smdk publish
Creating package john/my-filter@0.1.0
.. fill out info in hub/package-meta.yaml
Package hub/my-filter-0.1.0.ipkg created
Package uploaded!
```

#### Inspect Result

Ensure that your SmartModule has been uploaded to the [Hub]:

%copy first-line%
```bash
$ fluvio hub list
  SMARTMODULE                 Visibility 
  john/my-filter@0.1.0        private    
  ...
```

The SmartMoudule is now available for [download] by anyone with access to the `[Hub]`.

Congratulations :tada:!  You are now ready to build, test, and publish your own SmartModules.

* Checkout [SmartModule types] build your data streaming SmartModules.

### Steps

1. [Generate a SmartModule]({{< ref "generate" >}})
2. [Build and Test]({{< ref "build-test" >}})
3. [Load to your Cluster]({{< ref "load" >}})
4. **[Publish to SmartModule Hub]({{< ref "publish" >}})**

[Hub]: {{< ref "/smartmodules/hub/overview" >}}
[SmartModule types]: {{< ref "/smartmodules/transform/overview" >}}
[built]: {{< ref "build-test/#build---operation" >}}
[download]: {{< ref "../hub/download" >}}
