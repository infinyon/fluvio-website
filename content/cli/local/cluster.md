---
title: Fluvio Cluster
menu: Cluster
weight: 20
---

The `fluvio cluster` family of commands is used to install and manage your own
Fluvio cluster. The two primary forms of self-hosted cluster are Kubernetes
(e.g. via Minikube) and Local clusters, both of which can be automatically set up
by cluster commands. Please make sure you have followed the
[getting started guide] and installed all the necessary dependencies before
attempting to install your own cluster.

[getting started guide]: {{< ref "/docs/get-started/cloud" >}}

## `fluvio cluster check`

This command is used to check whether you have all the required dependencies
installed correctly. If this runs and returns successfully, you should be all
set to start a Fluvio cluster.

{{% inline-embed file="embeds/cli/help/fluvio-cluster-check.md" %}}

### Example usage:

%copy first-line%
```bash
$ fluvio cluster check
Running pre-startup checks...
     ✅ Kubernetes config is loadable
     ✅ Supported kubernetes version is installed
     ✅ Supported helm version is installed
     ✅ Can create service
     ✅ Can create customresourcedefinitions
     ✅ Can create secret
     ✅ Fluvio system charts are installed
🎉 All checks passed!
You may proceed with cluster startup
next: run `fluvio cluster start`
```

## `fluvio cluster start`

This command is used to start your own Fluvio cluster, with all the
machinery needed to receive, process, and serve streaming messages.

There are two main variations of this command. The default variation
is invoked simply by `fluvio cluster start`. This will install Fluvio
to a Kubernetes cluster, typically Minikube. The other variation is
`fluvio cluster start --local`, which will start the cluster components
as plain processes on your local machine.

{{% inline-embed file="embeds/cli/help/fluvio-cluster-start.md" %}}

### Example usage:

To start a cluster on Minikube:

%copy first-line%
```bash
$ fluvio cluster start
📝 Running pre-flight checks
     ✅ Kubernetes config is loadable
     ✅ Supported helm version is installed
     ✅ Fixed: Missing Fluvio system charts.
     ✅ Previous fluvio installation not found
🛠️  Installing Fluvio
     ✅ Fluvio app chart has been installed
🔎 Found SC service addr: 172.19.0.2:30814
👤 Profile set
🤖 SPU group launched (1)
     ✅ All SPUs confirmed
🎯 Successfully installed Fluvio!
```

To start a cluster locally (as processes on your machine):

%copy first-line%
```bash
$ fluvio cluster start --local
📝 Running pre-flight checks
     ✅ Supported helm version is installed
     ✅ Supported kubernetes version is installed
     ✅ Kubernetes config is loadable
     ✅ Fixed: Missing Fluvio system charts.
🖥️  SC Launched
🤖 SPU group launched (1)
👤 Profile set
🎯 Successfully installed Fluvio!
```

## `fluvio cluster delete`

Deletes a Fluvio cluster and all data associated with it. 

~> **CAUTION**: Be careful, this cannot be undone.

{{% inline-embed file="embeds/cli/help/fluvio-cluster-delete.md" %}}

### Example usage:

To uninstall Fluvio from Kubernetes (e.g. Minikube):

%copy first-line%
```bash
$ fluvio cluster delete
```

To uninstall Fluvio from your local machine:

%copy first-line%
```bash
$ fluvio cluster delete --local
```

## `fluvio cluster spu list`

This command shows details about the active SPUs in your cluster.
It is mostly useful for checking on the status of individual SPUs
to see whether they are still online, and which addresses they live at.

{{% inline-embed file="embeds/cli/help/fluvio-cluster-spu-list.md" %}}

### Example usage:

%copy first-line%
```bash
$ fluvio cluster spu list
 ID    NAME             STATUS  TYPE      RACK  PUBLIC          PRIVATE
 5001  custom-spu-5001  Online  "custom"   -    localhost:9010  localhost:9011
```
