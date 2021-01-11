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
⏳ Downloading Fluvio CLI with latest version: fluvio/fluvio:0.6.0-rc.5...
🔑 Downloaded and verified package file
✅ Successfully updated /Users/you/.fluvio/bin/fluvio$ fluvio update
```

Upgrading the Fluvio cluster is slightly more involved and less polished.
As of this writing, the only way to update your cluster is to delete it and re-start it.

~> Warning: This will cause your entire cluster's data to be wiped. We are working on a better upgrade experience.

If you are using Fluvio on Minikube, you can upgrade your cluster with these commands:

```
$ fluvio cluster delete
$ fluvio cluster start
```

If you are using Fluvio locally, the upgrading process is only slightly different:

```
$ fluvio cluster delete --local
$ fluvio cluster start --local
```

