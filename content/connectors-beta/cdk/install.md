---
title: Install 
weight: 20
---

### Install

CDK is an independent executable downloadable via [`Fluvio CLI`](https://www.fluvio.io/cli/utilities/install/):

```bash
$ fluvio install cdk
```

The executable is installed in `~/.fluvio/bin` directory, the same directory as `fluvio`. The command should be immediately accessible at the prompt:

{{% inline-embed file="embeds/cli/help/cdk.md" %}}

### Steps

1. **[Install CDK]({{< ref "install" >}})**
2. [Generate a SmartConnector]({{< ref "generate" >}})
3. [Build and Test]({{< ref "build-test" >}})
4. [Start and Shutdown]({{< ref "start-shutdown" >}})
5. [List and Logs]({{< ref "list-log" >}})
6. [Publish to SmartConnector Hub]({{< ref "publish" >}})