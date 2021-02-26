---
title: Getting Started with Fluvio
folder: Getting Started
menu: Overview
toc: true
weight: 10
---

Welcome! Thanks for your interest in Fluvio, the high-performance, low-latency streaming platform for real-time applications. 

In this guide, we're going to walk through the setup process for getting started with Fluvio.
To start, we'll show you how to install the Fluvio CLI, then later we'll show you how to get access to a Fluvio cluster, either locally or via Fluvio Cloud. Note that both of these options requires you to have the Fluvio CLI installed.

## Installing the Fluvio CLI

The Fluvio CLI (_command-line interface_) is an all-in-one tool for setting
up, managing, and interacting with Fluvio.

-> Please note that the CLI currently only support **Linux** and **MacOS**.

#### Download and Install

Downloading and installing Fluvio is as simple as running the following command!

```bash
$ curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

If everything works successfully, you should see output similar to this:

```bash
fluvio: ⏳ Downloading Fluvio 0.7.0 for x86_64-apple-darwin...
fluvio: ⬇️ Downloaded Fluvio, installing...
fluvio: ✅ Successfully installed ~/.fluvio/bin/fluvio
fluvio: ☁️ Installing Fluvio Cloud...
fluvio: 🎣 Fetching latest version for package: fluvio/fluvio-cloud...
fluvio: ⏳ Downloading package with latest version: fluvio/fluvio-cloud:0.1.4...
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

```bash
$ fluvio version
Fluvio CLI        : 0.7.0
Fluvio CLI SHA256 : ba6a3f659080446d5951fb1e1906f0aebef35c83bd797515a4b734ee53acfd24
Fluvio Platform   : 0.7.0 (minikube)
Git Commit        : f4cf81b52dbd000cd6fc87b59927aeb73e737d8a
OS Details        : Darwin 10.16 (kernel 20.3.0)
```

```bash
$ fluvio cloud
fluvio-cloud 0.1.4

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

### Next Steps

- [Create a free Fluvio Cloud account (Recommended)]
- [Install a Fluvio cluster locally]

[Create a free Fluvio Cloud account (Recommended)]: ./fluvio-cloud
[Install a Fluvio cluster locally]: ./fluvio-local
