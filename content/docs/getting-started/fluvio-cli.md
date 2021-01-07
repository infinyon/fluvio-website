---
title: Installing the Fluvio CLI
menu: Install Fluvio CLI
toc: true
weight: 20
---

The Fluvio CLI (or _command-line interface_) is an all-in-one tool for setting
up, managing, and interacting with Fluvio.

-> Please note that we currently only support Linux and MacOS

#### Download and Install

Downloading and installing Fluvio is as simple as running the following command!

```bash
$ curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

If everything works successfully, you should see output similar to this:

```bash
fluvio: ⏳ Downloading Fluvio 0.6.0-rc.2 for x86_64-apple-darwin...
fluvio: ⬇️ Downloaded Fluvio, installing...
fluvio: ✅ Successfully installed ~/.fluvio/bin/fluvio
fluvio: ☁️ Installing Fluvio Cloud...
fluvio: 🎣 Fetching latest version for package: fluvio/fluvio-cloud...
fluvio: ⏳ Downloading package with latest version: fluvio/fluvio-cloud:0.1.1...
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
Fluvio CLI      : 0.6.0-rc.2
Fluvio Platform : 0.6.0-rc.2
Git Commit      : 2aebf3b5d8f224238ccefded2f460a7f47b885fc
OS Details      : Darwin 19.6.0 x86_64
Rustc Version   : 1.48.0 (7eac88a 2020-11-16)
```

```bash
$ fluvio cloud
fluvio-cloud 0.1.1

USAGE:
    fluvio-cloud <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    login     Log into Fluvio Cloud with a username and password
    logout    Log out of a Fluvio Cloud account
```

#### Next Steps
----------------
* [Create a free Fluvio Cloud account](../fluvio-cloud)
* [Install Fluvio locally](../fluvio-local)