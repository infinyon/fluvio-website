---
title: CLI Installation
menu: CLI Installation 
weight: 10
---
## Install Fluvio CLI

The Fluvio CLI (_command-line interface_) is an all-in-one tool for setting up, managing, and interacting with Fluvio.

* **Recommended method**: Installer script
  * supported: MacOS/Linux 
  * *experimental: Windows/WSL*

%copy first-line%

```bash
$ curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

~> **Note**: Make sure to follow the post-installation instructions. 

* Or download the Fluvio binaries from [Github Releases](https://github.com/infinyon/fluvio/releases)

~> **Note**: Remember to `fluvio` binary to your shell `$PATH`
### Fluvio CLI Plugins

The installer script additionally installs the following plugins into `~/.fluvio/extensions`, which extend the functionality of the `fluvio` CLI for running a server or using Fluvio Cloud, respectively.
* `fluvio-run`
* `fluvio-cloud`