---
title: Upgrade
weight: 30
toc: false
---

When upgrading to a new version of Fluvio, the first thing you need to do
is [update the Fluvio CLI] using the following command:

[update the Fluvio CLI]: {{< ref "/cli/client/update" >}}

%copy first-line%
```bash
$ fluvio update
```

After updating the CLI, we can upgrade the Fluvio _cluster_ using this command:

%copy first-line%
```bash
$ fluvio cluster upgrade
```

Internally, the `fluvio cluster upgrade` command is actually using the
<a href="https://helm.sh/docs/helm/helm_upgrade/" target="_blank">helm</a>
package manager to upgrade the Fluvio helm charts installed on
your Kubernetes cluster.

You may optionally specify a specific Fluvio chart version by adding
the `--chart-version` argument to the upgrade command, such as the following:

%copy first-line%
```bash
$ fluvio cluster upgrade --chart-version=0.8.3
```

~> Note that upgrading is not reversible, as downgrading is not supported.

The possible versions you may pass to `--chart-version` correspond to the
available helm charts published on the Fluvio chart museum. You may view
this list of releases quickly with the command:

%copy first-line%
```bash
$ fluvio cluster releases list
```
