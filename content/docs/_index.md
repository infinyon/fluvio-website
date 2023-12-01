---
title: Quick Start
section: Docs
description: "An open source streaming platform that aggregates, correlates, and applies programmable intelligence to data in motion"
img: docs/architecture/images/social/fluvio-stack-social.png
twitter-card: summary_large_image
---


Fluvio is an open-source data streaming platform that aggregates, correlates, and apply programmable intelligence to data in motion. 


<img src="architecture/images/fluvio-stack.svg" alt="Fluvio Stack" justify="center" height="440">


Written in Rust, Fluvio provides low-latency, high-performance programmable streaming on cloud-native architecture.

SmartModules, powered by WebAssembly, provide users the ability to perform inline computations on streaming
data without leaving the cluster. – [Learn more about SmartModules]({{<ref "/smartmodules" >}})

Fluvio provides client libraries for several popular programming languages. – [Learn more about Fluvio's client libraries]({{<ref "/api" >}})


This guide outlines using the `Fluvio CLI` for streaming data on your local machine or self-hosted environment.

We will cover:
- Starting a cluster
- Storing data with **Topics**
- Sending and reading data with **Producers** and **Consumers**
- Creating **Connectors**
- Developing and using **SmartModules**

## Requirements

- Basic command-line experience for using the `fluvio` CLI
- A Rust development environment for SmartModule development
- The `fluvio` and `smdk` CLIs

## Installing the CLI

Download and install the CLI by running:

%copy first-line%
```bash
$ curl -fsS https://hub.infinyon.cloud/install/install.sh | bash
```

`fvm` will be installed at `~/.fvm/bin`, and will install `fluvio` and the rest of the development tools at `~/.fluvio/bin`.

You will need to add these directories to your shell's `PATH` environment variable.

{{< h-list tabTotal="3" tabID="1" tabName1="Bash" tabName2="Zsh" tabName3="Fish">}}

{{< h-item tabNum="1">}}
%copy first-line%
```shell
$ echo 'export PATH="${HOME}/.fvm/bin:${HOME}/.fluvio/bin:${PATH}"' >> ~/.bashrc
```
{{< /h-item>}}

{{< h-item tabNum="2">}}
%copy first-line%
```shell
$ echo 'export PATH="${HOME}/.fvm/bin:${HOME}/.fluvio/bin:${PATH}"' >> ~/.zshrc
```
{{< /h-item>}}

{{< h-item tabNum="3">}}
%copy first-line%
```shell
$ fish_add_path ~/.fluvio/bin
```
{{< /h-item>}}

{{< /h-list>}}

Example output

%copy first-line%
```shell
$ curl -fsS https://hub.infinyon.cloud/install/install.sh | bash

☁️  Downloading fluvio version manager, fvm
   target arch aarch64-apple-darwin
⬇️ Installing fvm
done: FVM installed successfully at /Users/telant/.fvm
help: Add FVM to PATH using source $HOME/.fvm/env
fluvio install dir /Users/$USER/.fluvio
If version of fluvio is already installed, you can run 'fvm install' or 'fvm switch' to change versions
☁️ Installing fluvio
info: Downloading (1/5): fluvio@0.11.0
info: Downloading (2/5): fluvio-cloud@0.2.15
info: Downloading (3/5): fluvio-run@0.11.0
info: Downloading (4/5): cdk@0.11.0
info: Downloading (5/5): smdk@0.11.0
done: Installed fluvio version 0.11.0
done: Now using fluvio version 0.11.0
🎉 Install complete!
fluvio: 💡 You'll need to add '~/.fvm/bin' and ~/.fluvio/bin/' to your PATH variable
fluvio:     You can run the following to set your PATH on shell startup:
fluvio:       echo 'export PATH="${HOME}/.fvm/bin:${HOME}/.fluvio/bin:${PATH}"' >> ~/.zshrc
```

## Start a cluster

%copy first-line%
```shell
$ fluvio cluster start
```

<TODO screenshots>

## Create your first topic

Topics store and send data streams. Create one with:

%copy first-line%
```shell
$ fluvio topic create quickstart-topic
```

## Produce data to your topic

Send data to your topic with:

%copy first-line%
```shell
$ fluvio produce quickstart-topic
> hello world!
Ok!
```

Exit the prompt with `Ctrl+C`. You can also send files or stream output:

%copy%
```shell
fluvio produce -f ./path/to/file.txt
echo "hello world!" | fluvio produce quickstart-topic
```

## Consume data from your topic

Read data from your topic with:

%copy first-line%
```shell
$ fluvio consume quickstart-topic -B -d
```

This will display data sent to the topic.

## Create your first SmartModule

SmartModules are user-defined functions that process data streams. 

### Install SMDK

The SmartModules Development Kit (SMDK) is required for building SmartModules:

%copy first-line%
```bash
$ fluvio install smdk
```

### Build a Filter SmartModule

Create and navigate to your SmartModule project:

%copy%
```bash
smdk generate
cd quickstart
```

A filter SmartModule might look like this:

%copy%
```rust
use fluvio_smartmodule::{smartmodule, Result, Record};

#[smartmodule(filter)]
pub fn filter(record: &Record) -> Result<bool> {
    let string = std::str::from_utf8(record.value.as_ref())?;
    Ok(string.contains('a'))
}
```

Build and test it with:

%copy%
```shell
smdk build
smdk test --text "cats"
```

Load it onto your cluster with:

```shell
smdk load
```

Verify it's loaded:

```shell
fluvio smartmodule list
```

### Using SmartModules with Topics

Create a topic and use SmartModules with producers and consumers:

%copy%
```shell
fluvio topic create fruits
fluvio consume fruits --smartmodule=quickstart
fluvio produce fruits
```

Only records matching the SmartModule's filter will be shown.

