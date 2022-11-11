---
title: Cloud Cluster
menu: Cluster
weight: 30
---

The `fluvio cloud cluster` family of commands is used to create, delete, and troubleshoot Fluvio clusters in cloud.

%copy first-line%
```bash
$  fluvio cloud cluster -h
```

{{% inline-embed file="embeds/cli/help/fluvio-cloud-cluster.md" %}}

---

## `fluvio cloud cluster create`

This command is used to provision a new cluster.

%copy first-line%
```bash
$  fluvio cloud cluster create -h
```

{{% inline-embed file="embeds/cli/help/fluvio-cloud-cluster-create.md" %}}


## `fluvio cloud cluster list`

Command to show the fluvio clusters in Cloud associated with current user.

%copy first-line%
```bash
$  fluvio cloud cluster list -h
```

{{% inline-embed file="embeds/cli/help/fluvio-cloud-cluster-list.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio cloud cluster list

 ID       ACTIVE  STATE      VERSION  SPU_COUNT 
 default  true    Installed  0.10.0   1         
```

---

## `fluvio cloud cluster delete`


This command deletes current cluster of current user.

%copy first-line%
```bash
$  fluvio cloud cluster delete -h
```

{{% inline-embed file="embeds/cli/help/fluvio-cloud-cluster-delete.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio cloud cluster delete foo@example.com
```
