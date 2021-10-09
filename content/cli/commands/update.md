---
title: Update
weight: 80
toc: false
---

This command performs a self-update for the Fluvio CLI. It takes no arguments,
and just simply downloads the latest version of `fluvio` and overwrites itself.

```
fluvio-update
Update the Fluvio CLI

USAGE:
    fluvio update [FLAGS] [plugins]...

FLAGS:
        --develop    Update to the latest prerelease rather than the latest release
    -h, --help       Prints help information

ARGS:
    <plugins>...    (Optional) the name of one or more plugins to update
```

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