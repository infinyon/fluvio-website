---
title: Profile
weight: 30
---

Fluvio provides support for multiple profiles out of the box, meaning that you can
communicate with distinct clusters in an isolated and managed way. A profile is a
collection of settings and credentials that are applied to a Fluvio session. Each profile
defines the means to communicate with a particular Fluvio cluster. One profile
is active (or "current") at any given time, and all other Fluvio commands will
interact with the cluster defined by the active profile. Profiles are defined
in the Fluvio configuration file at `~/.fluvio/config`, and the `fluvio profile`
family of commands is used to view and manipulate profiles.

## `fluvio profile current`

Prints out the name of the active Fluvio profile. Let's say you have a Fluvio Cloud account
whose profile name is "cloud", and you have a local Fluvio cluster installed that
you use for development, whose profile name is "local".

If your current profile is "local", then a command such as [`fluvio topic create greeting`] will
create a "greeting" topic on your local cluster.

[`fluvio topic create greeting`]: {{< ref "/cli/commands/topic#fluvio-topic-create" >}}

{{% inline-embed file="embeds/cli/help/fluvio-profile-current.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio profile current
local
```

## `fluvio profile delete`

Deletes a profile from your Fluvio configuration (`~/.fluvio/config`). This will
not delete a cluster (for that, see [`fluvio cluster delete`]), but it will cause
the Fluvio CLI to no longer be able to interact with that cluster.

{{% inline-embed file="embeds/cli/help/fluvio-profile-delete.md" %}}

If you try to delete the current profile, you will get a warning, because other CLI
commands depend on having a current profile being set.

%copy first-line%
```bash
$ fluvio profile delete local
profile local deleted
warning: this removed your current profile, use 'fluvio profile switch' to select a different one
```

## `fluvio profile delete-cluster`

Deletes cluster connection information from your Fluvio configuration. This will not delete
a cluster itself (for that see [`fluvio cluster delete`]), but it will cause the Fluvio CLI to
no longer be able to connect to that cluster.

[`fluvio cluster delete`]: {{< ref "/cli/local/cluster#fluvio-cluster-delete" >}}

{{% inline-embed file="embeds/cli/help/fluvio-profile-delete-cluster.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio profile delete-cluster local
Cluster local deleted
```

## `fluvio profile switch`

This switches the "current" profile. After switching current profiles, all
subsequent Fluvio CLI commands that interact with a cluster will target the
cluster of the new "current" profile.


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
