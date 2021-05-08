---
title: MacOS Installation
menu: MacOS
weight: 10
---


Welcome! Thanks for your interest in Fluvio, the high-performance, low-latency streaming platform for real-time applications. 

In this guide, we're going to walk through the setup process for getting started with Fluvio.
To start, we'll show you how to install the Fluvio CLI, then later we'll show you how to get access to a Fluvio cluster, either locally or via Fluvio Cloud. Note that both of these options requires you to have the Fluvio CLI installed.

## Install Fluvio CLI

The Fluvio CLI (_command-line interface_) is an all-in-one tool for setting
up, managing, and interacting with Fluvio.

#### Download and Install

Downloading and installing Fluvio is as simple as running the following command!

%copy first-line%

```bash
$ curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

If everything works successfully, you should see output similar to this:

```bash
fluvio: ⏳ Downloading Fluvio 0.8.0 for x86_64-apple-darwin...
fluvio: ⬇️ Downloaded Fluvio, installing...
fluvio: ✅ Successfully installed ~/.fluvio/bin/fluvio
fluvio: ☁️ Installing Fluvio Cloud...
fluvio: 🎣 Fetching latest version for package: fluvio/fluvio-cloud...
fluvio: ⏳ Downloading package with latest version: fluvio/fluvio-cloud:0.1.5...
fluvio: 🔑 Downloaded and verified package file
fluvio: ☁️ Installing Fluvio Runner...
fluvio: 🎣 Fetching latest version for package: fluvio/fluvio-run...
fluvio: ⏳ Downloading package with latest version: fluvio/fluvio-run:0.8.0...
fluvio: 🔑 Downloaded and verified package file
fluvio: 🎣 Fetching latest version for package: fluvio/fluvio-run...
fluvio: ⏳ Downloading package with latest version: fluvio/fluvio-run:0.8.0...
fluvio: 🔑 Downloaded and verified package file
fluvio: 🎉 Install complete!
fluvio: 💡 You'll need to add '~/.fluvio/bin/' to your PATH variable
fluvio:     You can run the following to set your PATH on shell startup:
fluvio:       For bash: echo 'export PATH="${HOME}/.fluvio/bin:${PATH}"' >> ~/.bashrc
fluvio:       For zsh : echo 'export PATH="${HOME}/.fluvio/bin:${PATH}"' >> ~/.zshrc
fluvio:
fluvio:     To use Fluvio you'll need to restart your shell or run the following:
fluvio:       export PATH="${HOME}/.fluvio/bin:${PATH}"
```

Notice the steps it lists at the bottom. In order to use the `fluvio` command,
you'll need to make sure that `~/.fluvio/bin/` (the directory where it was installed to)
is part of your system's PATH - the list of directories where your shell looks for
executables. Once you've followed those instructions, make sure you can run the following
commands:

%copy first-line%

```bash
$ fluvio version
Fluvio CLI           : 0.8.0
Fluvio CLI SHA256    : 493abdb87591eb439884b8caf1e103135a30318e3d9d48efab117fb96b35c67b
Fluvio Platform      : 0.8.0 (local)
Git Commit           : 2b1b06f27e68a79a58617cc670ff38ef5abe8985
OS Details           : Darwin 10.16 (kernel 20.3.0)
=== Plugin Versions ===
Fluvio Runner (fluvio-run)     : 0.1.0
Fluvio Cloud CLI (fluvio-cloud) : 0.1.5
```

Fluvio Cloud library is an add-on module that was automatically installed for:

%copy first-line%

```bash
$ fluvio cloud
fluvio-cloud 0.1.5

USAGE:
    fluvio-cloud <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    login     Log into Fluvio Cloud with a username and password
    logout    Log out of a Fluvio Cloud account
    version   Print the current version of fluvio-cloud
```

## Next Steps

- [Create a free Fluvio Cloud account (Recommended)]
- [Install a Fluvio cluster locally]

[Create a free Fluvio Cloud account (Recommended)]: ./fluvio-cloud
[Install a Fluvio cluster locally]: ./fluvio-local
