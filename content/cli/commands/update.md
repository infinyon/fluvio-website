---
title: Update
weight: 80
toc: false
---

This command performs a self-update for the Fluvio CLI. It takes no arguments,
and just simply downloads the latest version of `fluvio` and overwrites itself.

```
fluvio-update 0.6.0
Update the Fluvio CLI

USAGE:
    fluvio update [FLAGS] [plugins]...

FLAGS:
        --develop    Update to the latest prerelease rather than the latest
                     release
    -h, --help       Prints help information

ARGS:
    <plugins>...    (Optional) the name of one or more plugins to update
```

Example usage:

%copy first-line%
```bash
$ fluvio update
🎣 Fetching latest version for fluvio/fluvio...
⏳ Downloading Fluvio CLI with latest version: fluvio/fluvio:0.6.0-beta.1...
🔑 Downloaded and verified package file
✅ Successfully installed ~/.fluvio/bin/fluvio
