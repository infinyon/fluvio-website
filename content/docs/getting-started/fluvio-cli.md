---
title: Installing the Fluvio CLI
menu: Install Fluvio CLI
toc: true
weight: 20
---

The Fluvio CLI (or _command-line interface_) is an all-in-one tool for setting
up, managing, and interacting with Fluvio. You can download the CLI from our
[github releases] page. You'll need to open the archive, then move the `fluvio`
executable into a directory in your `PATH`.

-> Please note that we currently only support Linux and MacOS

[github releases]: https://github.com/infinyon/fluvio/releases

```shell
$ sudo mv fluvio /usr/local/bin/
```

Now you can run the `fluvio version` command to make sure it worked

```shell
$ fluvio version
Fluvio version : 0.6.0
Git Commit     : 3ff1ee52a31f51c2a3dc7d8fe3996d694fdd585d
OS Details     : Darwin 19.6.0 x86_64
Rustc Version  : 1.46.0 (04488af 2020-08-24)
```

-> Keep in mind that you'll probably see different versions than what's shown here.


#### Next Steps
----------------
* [Create a free Fluvio Cloud account](../fluvio-cloud)
* [Install Fluvio locally](../fluvio-local)