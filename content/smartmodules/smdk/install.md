
---
title: Install SMDK
menu: Install
toc: false
weight: 20
---

SMDK is an independent executable downloadable via [`Fluvio CLI`].

### Install - Operation

Install SMDK:

%copy first-line%
```bash
$ fluvio install smdk
Current channel: stable
🎣 Fetching stable version for package: fluvio/smdk...
⏳ Downloading package with latest version: fluvio/smdk:0.10.0+dc420af88d7a67599547266ebfefdd71a0f85754...
🔑 Downloaded and verified package file
```

The executable is installed in `~/.fluvio/bin/` directory, the same directry as fluvio, hence the command should be immediate accesible at the prompt. Let's check it out:


{{% inline-embed file="embeds/cli/help/smdk.md" %}}

### Steps

1. **[Install SMDK]({{< ref "install" >}})**
2. [Generate a SmartModule]({{< ref "generate" >}})
3. [Build and Test]({{< ref "build-test" >}})
4. [Load to your Cluster]({{< ref "load" >}})
5. [Publish to SmartModule Hub]({{< ref "publish" >}})

[`Fluvio CLI`]: {{< ref "/cli/smartmodules/smdk" >}}
[Generate a SmartModule]: {{< ref "generate" >}}
[Build and Test]: {{< ref "build-test" >}}
[Load to your Cluster]: {{< ref "load" >}}
[Publish to SmartModule Hub]: {{< ref "publish" >}}
