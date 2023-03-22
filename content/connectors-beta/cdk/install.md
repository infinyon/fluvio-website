---
title: Install 
weight: 20
---

### Install

CDK is an independent executable downloadable via [`Fluvio CLI`](https://www.fluvio.io/cli/utilities/install/):

```bash
$ fluvio install cdk

# Until it is officially released
# fluvio install --hub cdk

# Alternative, recommended only for dev purposes:
# cargo install --git https://github.com/infinyon/fluvio cdk
```

The executable is installed in `~/.fluvio/bin` directory, the same directory as `fluvio`. The command should be immediately accessible at the prompt:

```bash
$ cdk

Connector Development Kit

Usage: cdk <COMMAND>

Commands:
  build       Build the Connector in the current working directory
  test        Build and run the Connector in the current working directory
  deploy      Deploy the Connector from the current working directory
  publish     Publish Connector package to the Hub
  set-public  Set connector visibility to public
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```