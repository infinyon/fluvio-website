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

```
fluvio-cloud-cluster 
View Fluvio Cluster information

USAGE:
    fluvio-cloud cluster <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    create    Create a new Fluvio cluster
    delete    Delete an existing Fluvio cluster
    help      Print this message or the help of the given subcommand(s)
    list      List all Fluvio clusters
```

---

## `fluvio cloud cluster create`

This command is used to provision a new cluster.

%copy first-line%
```bash
$  fluvio cloud cluster create -h
```

```
Create a new Fluvio cluster

USAGE:
    fluvio-cloud cluster create [OPTIONS]

OPTIONS:
    -h, --help                 Print help information
        --profile <PROFILE>    The name of the Profile to save
```

## `fluvio cloud cluster list`

Command to show the fluvio clusters in Cloud associated with current user.

%copy first-line%
```bash
$  fluvio cloud cluster list -h
```

```
List all Fluvio clusters

USAGE:
    fluvio-cloud cluster list

OPTIONS:
    -h, --help    Print help information
```

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

```
USAGE:
    fluvio-cloud cluster delete <email>

ARGS:
    <email>    The email for the cluster to be deleted

OPTIONS:
    -h, --help    Print help information
```

Example usage:

%copy first-line%
```bash
$ fluvio cloud cluster delete foo@example.com
```
