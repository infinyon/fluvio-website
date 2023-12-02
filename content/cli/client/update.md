---
title: Update
weight: 20
toc: false
hidden: true
---

## `fluvio update`

This command performs a self-update for the Fluvio CLI and any plugins supplied as arguments. 

{{% inline-embed file="embeds/cli/help/fluvio-update.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio update
🎣 Fetching latest version for fluvio...
⏳ Downloading Fluvio CLI with latest version: x.y.z...
🔑 Downloaded and verified package file
✅ Successfully updated /.fluvio/bin/fluvio
🔧 Preparing update for 2 plugins:
   - fluvio-run (/.fluvio/extensions/fluvio-run)
   - fluvio-cloud (/.fluvio/extensions/fluvio-cloud)
⏳ Downloading plugin fluvio-run with version x.y.z
🔑 Downloaded and verified package file
✅ Successfully updated fluvio/fluvio-run: x.y.z at (/.fluvio/extensions/fluvio-run)
⏳ Downloading plugin fluvio-cloud with version x.y.z
🔑 Downloaded and verified package file
✅ Successfully updated fluvio/fluvio-cloud:x.y.z at (/.fluvio/extensions/fluvio-cloud)
