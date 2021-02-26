---
title: Upgrading Fluvio
menu: Upgrading Fluvio
toc: true
weight: 400
---

In this guide, we're going to talk about how to upgrade your Fluvio installation
so that you can be working with the latest version of Fluvio.

There are two major components that will occasionally need upgrading. The Fluvio
CLI and the Fluvio cluster you're working with. Upgrading the Fluvio CLI is easy,
it can do it for you!

```
$ fluvio update
🎣 Fetching latest version for fluvio/fluvio...
⏳ Downloading Fluvio CLI with latest version: fluvio/fluvio:0.7.0...
🔑 Downloaded and verified package file
✅ Successfully updated /Users/nick/.fluvio/bin/fluvio
```

Run the version command to check the if fluvio CLI and the platform (cluster) are in sync:

```
$ fluvio version
Fluvio CLI        : 0.7.0
Fluvio CLI SHA256 : ba6a3f659080446d5951fb1e1906f0aebef35c83bd797515a4b734ee53acfd24
Fluvio Platform   : 0.7.0 (minikube)
Git Commit        : f4cf81b52dbd000cd6fc87b59927aeb73e737d8a
OS Details        : Darwin 10.16 (kernel 20.3.0)
```

Upgrading the Fluvio cluster works differently depending on whether your cluster is on
Minikube or if it is being run locally. If you are running on Minikube, just use the
following command:

~> Warning: This may cause your entire cluster's data to be wiped.

If you are using Fluvio on Minikube, you can upgrade your cluster with these commands:

```
$ fluvio cluster upgrade
```

If you are using Fluvio locally, you'll need to first delete your cluster, then re-create it.

~> Warning: This _will_ cause your entire cluster's data to be wiped.

```
$ fluvio cluster delete --local
$ fluvio cluster start --local
```

