---
title:  Publish
weight: 80
---

This section assumes `my-connector` project has been [generated]({{< ref "generate" >}}).

Connector Hub is a public repository of connectors. You can publish your connector as `private` to use on different computers or `pubic` to share it with the community.

### Publish to Connector Hub

The final step would be to use `cdk publish` and publish the Connector to the Hub.

If run without arguments, it will pack everything needed into a package and push the package to the Hub. 

You inspect the package file and make modifications before the push:

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

The `publish` command uses `hub/package-meta.yml` file with the metadata needed for the Hub. If it doesn’t exist, `cdk` creates it for you. Then, you can modify it, and the changes will be picked up on the subsequent command execution.

## Steps

1. [Generate a Connector]({{< ref "generate" >}})
2. [Build and Test]({{< ref "build-test" >}})
3. [Start and Shutdown]({{< ref "start-shutdown" >}})
4. [Troubleshooting]({{< ref "troubleshooting" >}})
5. [Secrets]({{< ref "secrets" >}})
6. **[Publish to Connector Hub]({{< ref "publish" >}})**
7. [Use Examples in Github]({{< ref "github-examples" >}})
