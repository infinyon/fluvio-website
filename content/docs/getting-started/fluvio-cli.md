---
title: Installing the Fluvio CLI
menu: Install Fluvio CLI
toc: true
weight: 20
---

The Fluvio CLI (or _command-line interface_) is an all-in-one tool for setting
up, managing, and interacting with Fluvio.

-> Please note that we currently only support Linux and MacOS

#### Download

Download the Fluvio CLI from our [github releases] page

[github releases]: https://github.com/infinyon/fluvio/releases

You'll want to rename the binary to `fluvio` and mark it as executable. For the
following command, make sure to use the filename of the release you downloaded.

```bash
$ mv fluvio-v0.6.0-alpha.2-x86_64-apple-darwin fluvio
$ chmod +x ./fluvio
```

Then, install the CLI by moving it into a bin folder in your PATH, like this

```bash
$ sudo mv ./fluvio /usr/local/bin/
```

Now you can run the `fluvio version` command to make sure it worked. Keep in mind
that you may see different versions than what's shown here.

```shell
$ fluvio version
Fluvio version : 0.6.0
Git Commit     : 3ff1ee52a31f51c2a3dc7d8fe3996d694fdd585d
OS Details     : Darwin 19.6.0 x86_64
Rustc Version  : 1.46.0 (04488af 2020-08-24)
```

#### Next Steps
----------------
* [Create a free Fluvio Cloud account](../fluvio-cloud)
* [Install Fluvio locally](../fluvio-local)