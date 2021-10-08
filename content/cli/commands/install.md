---
title: Install
weight: 70
toc: false
---

Some Fluvio CLI commands are distributed as separate executables that we call
"extensions". This command installs and updates extensions by name, placing
them in the `~/.fluvio/extensions/` directory.

```
fluvio-install
Install Fluvio plugins

The Fluvio CLI considers any executable with the prefix `fluvio-` to be a CLI
plugin. For example, an executable named `fluvio-foo` in your PATH may be
invoked by running `fluvio foo`.

This command allows you to install plugins from Fluvio's package registry.

USAGE:
    fluvio install <package>

FLAGS:
    --develop    Install the latest prerelease rather than the latest
                 release
    -h, --help    Prints help information

OPTIONS:


ARGS:
    <package>    The ID of a package to install, e.g. "fluvio/fluvio-cloud"
```

Example usage:

%copy first-line%
```bash
$ fluvio install fluvio/fluvio-cloud
🎣 Fetching latest version for package: fluvio/fluvio-cloud...
⏳ Downloading package with latest version: fluvio/fluvio-cloud:0.1.0...
🔑 Downloaded and verified package file
```
