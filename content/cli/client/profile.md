---
title: Profile
weight: 30
---

A Fluvio profile contains the a reference to cluster's connection details.

Profile details are stored in the Fluvio configuration file at `~/.fluvio/config`.

## `fluvio profile current`

Prints out the name of the active Fluvio profile. 

[`fluvio topic create greeting`]: {{< ref "/cli/commands/topic#fluvio-topic-create" >}}

{{% inline-embed file="embeds/cli/help/fluvio-profile-current.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio profile current
local
```

## `fluvio profile delete`

Deletes the specified Fluvio cluster connection profile from your Fluvio configuration (`~/.fluvio/config`).

This will not delete a cluster. See [`fluvio cluster delete`]({{<ref "/cli/local/cluster.md#fluvio-cluster-delete" >}}) for more info.

{{% inline-embed file="embeds/cli/help/fluvio-profile-delete.md" %}}


## `fluvio profile delete-cluster`

Deletes only cluster connection information from your profile. 

This will not delete a cluster. See [`fluvio cluster delete`]({{<ref "/cli/local/cluster.md#fluvio-cluster-delete" >}}) for more info.

[`fluvio cluster delete`]: {{< ref "/cli/local/cluster#fluvio-cluster-delete" >}}

{{% inline-embed file="embeds/cli/help/fluvio-profile-delete-cluster.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio profile delete-cluster local
Cluster local deleted
```

## `fluvio profile switch`

Chqnges the active CLI profile. 

{{% inline-embed file="embeds/cli/help/fluvio-profile-switch.md" %}}

## `fluvio profile list`

Prints a table of your profiles, including the address of the associated
cluster and which profile is active.


{{% inline-embed file="embeds/cli/help/fluvio-profile-list.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio profile list
    PROFILE    CLUSTER    ADDRESS                          TLS
 *  minikube   minikube   10.99.16.213:9003                Disabled
    local      local      localhost:9003                   Disabled
```
