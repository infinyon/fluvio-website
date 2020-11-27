---
title: Fluvio CLI reference
menu: Fluvio CLI reference
toc: true
weight: 50
---

This page is the detailed reference for all the commands in the Fluvio CLI.
Most of the information given here can also be found in the help menu for
Fluvio, `fluvio help`, but here we will also explain when you may want to
use certain commands and show some examples.

## Command Table-of-Contents

Here is a list of all Fluvio commands that are available by default:

```
fluvio consume                    Reads messages from a topic/partition
fluvio produce                    Writes messages to a topic/partition
fluvio topic create               Creates a topic with the given name
fluvio topic delete               Deletes a topic with the given name
fluvio topic describe             Prints detailed information about a Topic
fluvio topic list                 Lists all of the Topics in the cluster
fluvio partition list             Lists all of the Partitions in the cluster
fluvio profile current            Prints the name of the current context
fluvio profile delete             Deletes the named profile
fluvio profile delete-cluster     Deletes the named cluster
fluvio profile switch             Switches to the named profile
fluvio profile sync               Syncs a profile from a cluster
fluvio profile view               Displays the entire configuration
fluvio cluster install            Installs a Fluvio cluster, locally or on Minikube
fluvio cluster uninstall          Uninstalls a Fluvio cluster from the local machine or Minikube
fluvio cluster check              Checks that all the requirements for cluster installation are met
fluvio cluster releases list      Shows a list of Fluvio release versions
fluvio cluster spu register       Registers a new custom SPU with the cluster
fluvio cluster spu unregister     Unregisters a custom SPU from the cluster
fluvio cluster spu list           Lists all SPUs known by this cluster (managed AND custom)
fluvio cluster spg create         Creates a new managed SPU Group
fluvio cluster spg delete         Deletes a managed SPU Group
fluvio cluster spg list           Lists all SPU Groups
fluvio cluster run sc             Runs a new Streaming Controller (SC)
fluvio cluster run spu            Runs a new Streaming Processing Unit (SPU)
fluvio install                    Install Fluvio plugins
fluvio update                     Update the Fluvio CLI
fluvio version                    Print Fluvio version information
fluvio help                       Prints help for Fluvio or a subcommand
```

#### Next Steps
----------------

*
