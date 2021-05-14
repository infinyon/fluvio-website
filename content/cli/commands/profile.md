---
title: Profile
weight: 50
---

Fluvio provides support for multiple profiles out of the box, meaning that you can
communicate with distinct clusters in an isolated and managed way. Each profile
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

[`fluvio topic create greeting`]: /cli/commands/topic#fluvio-topic-create

```
fluvio-profile-current
Print the name of the current context

USAGE:
fluvio profile current

FLAGS:
-h, --help    Prints help information
```

Example usage:

```
$ fluvio profile current
local
```

## `fluvio profile delete`

Deletes a profile from your Fluvio configuration (`~/.fluvio/config`). This will
not delete a cluster (for that, see [`fluvio cluster delete`]), but it will cause
the Fluvio CLI to no longer be able to interact with that cluster.

```
fluvio-profile-delete
Delete the named profile

USAGE:
    fluvio profile delete <profile name>

FLAGS:
    -h, --help    Prints help information

ARGS:
    <profile name>
```

Example usage:

```
$ fluvio profile delete local
profile local deleted
```

If you try to delete the current profile, you will get a warning, because other CLI
commands depend on having a current profile being set.

```
$ fluvio profile delete local
profile local deleted
warning: this removed your current profile, use 'fluvio profile switch' to select a different one
```

## `fluvio profile delete-cluster`

Deletes cluster connection information from your Fluvio configuration. This will not delete
a cluster itself (for that see [`fluvio cluster delete`]), but it will cause the Fluvio CLI to
no longer be able to connect to that cluster.

[`fluvio cluster delete`]: /cli/commands/cluster#fluvio-cluster-delete

```
fluvio-profile-delete-cluster
Delete the named cluster

USAGE:
    fluvio profile delete-cluster [FLAGS] <cluster name>

FLAGS:
    -f, --force    Deletes a cluster even if its active
    -h, --help     Prints help information

ARGS:
    <cluster name>    The name of a cluster connection to delete
```

Example usage:

```
$ fluvio profile delete-cluster local
Cluster local deleted
```

## `fluvio profile switch`

This switches the "current" profile. After switching current profiles, all
subsequent Fluvio CLI commands that interact with a cluster will target the
cluster of the new "current" profile.

```
fluvio-profile-switch
Switch to the named profile

USAGE:
    fluvio profile switch <profile name>

FLAGS:
    -h, --help    Prints help information

ARGS:
    <profile name>
```

Example usage:

```
$ fluvio profile switch cloud
```

## `fluvio profile view`

Prints a table of your profiles, including the address of the associated
cluster and which profile is active.

```
fluvio-profile-view
Display all Fluvio profiles

USAGE:
    fluvio profile view

FLAGS:
    -h, --help    Prints help information
    
OPTIONS:
    -O, --output <type>    Output [default: table]  [possible values: table,
                           yaml, json]
```

Example usage:

```
$ fluvio profile view
    PROFILE    CLUSTER    ADDRESS                          TLS
 *  minikube   minikube   10.99.16.213:9003                Disabled
    local      local      localhost:9003                   Disabled
```
