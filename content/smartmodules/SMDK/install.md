
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

%copy first-line%
```bash
$ smdk -h
SmartModule Development Kit utility

Usage: smdk <COMMAND>

Commands:
  build     Builds SmartModule into WASM
  generate  Generates a new SmartModule Project
  test      Test SmartModule
  load      Load SmartModule into Fluvio cluster
  publish   Publish SmartModule to Hub
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

### Next Steps

2. [Generate a SmartModule]
3. [Build and Test]
4. [Load to your Cluster]
5. [Publish to SmartMoudle Hub]


[`Fluvio CLI`]: {{< ref "/cli/smartmodules/smdk" >}}
[Generate a SmartModule]: {{< ref "generate" >}}
[Build and Test]: {{< ref "build-test" >}}
[Load to your Cluster]: {{< ref "load" >}}
[Publish to SmartMoudle Hub]: {{< ref "publish" >}}
