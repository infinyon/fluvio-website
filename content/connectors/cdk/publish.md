---
title:  Publish 
weight: 70
---
##### Prerequisites

This section assumes that CDK is [installed]({{< ref "install" >}}) and `my-connector` project has been [generated]({{< ref "generate" >}}).


### Publish

The final step in this scenario would be publishing the SmartModule to the Hub - where other users can download and integrate it into their data pipelines.

If run without arguments, it will pack everything needed into a package and push the package to the Hub. (`cdk build`must be executed beforehand)

If you need to inspect the package before the push:

%copy first-line%
```bash
% cdk publish --pack
Using hub https://hub-dev.infinyon.cloud
Creating package aj/my-connector@0.1.0
.. fill out info in hub/package-meta.yaml
Package hub/my-connector-0.1.0.ipkg created
```

Check the file and then push. The file is a Tar Gzip archive.

%copy first-line%
```bash
$ cdk publish --push hub/my-connector-0.1.0.ipkg
```

`publish` command uses `hub/package-meta.yml` file with the metadata needed for the Hub. If it doesn’t exist, `cdk` creates it for you. Then, you can modify it, and the changes will be picked up on the subsequent command execution. 

1. [Install CDK]({{< ref "install" >}})
2. [Generate a SmartConnector]({{< ref "generate" >}})
3. [Build and Test]({{< ref "build-test" >}})
4. [Start and Shutdown]({{< ref "start-shutdown" >}})
5. [List and Logs]({{< ref "list-log" >}})
6. **[Publish to SmartConnector Hub]({{< ref "publish" >}})**