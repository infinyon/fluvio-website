---
title: Install
menu: Install (deprecated)
weight: 10
toc: false
---

{{<caution>}}
This command is deprecated since 0.10.16, and has been replaced by [`fvm install`]({{<ref "cli/utilities/fluvio-version-manager.md#fvm-install">}})
{{</caution>}}

Some Fluvio CLI commands are distributed as separate executables that we call
"extensions". This command installs and updates extensions by name, placing
them in the `~/.fluvio/extensions/` directory.

## `fluvio install`
{{% inline-embed file="embeds/cli/help/fluvio-install.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio install fluvio/fluvio-cloud
🎣 Fetching latest version for package: fluvio/fluvio-cloud...
⏳ Downloading package with latest version: fluvio/fluvio-cloud:x.y.z...
🔑 Downloaded and verified package file
```
