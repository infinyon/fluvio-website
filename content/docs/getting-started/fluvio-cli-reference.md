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
fluvio consume                    Read messages from a topic/partition
fluvio produce                    Write messages to a topic/partition
fluvio topic create               Create a topic with the given name
fluvio topic delete               Delete a topic with the given name
fluvio topic describe             Print detailed information about a Topic
fluvio topic list                 List all of the Topics in the cluster
fluvio partition list             List all of the Partitions in the cluster
fluvio profile current            Print the name of the current context
fluvio profile delete             Delete the named profile
fluvio profile delete-cluster     Delete the named cluster
fluvio profile switch             Switch to the named profile
fluvio profile sync k8            Sync a profile from a Kubernetes cluster
fluvio profile sync local         Sync a profile from a local cluster
fluvio profile view               Display the entire configuration
fluvio cluster install            Install a Fluvio cluster, locally or on Minikube
fluvio cluster uninstall          Uninstall a Fluvio cluster from the local machine or Minikube
fluvio cluster check              Check that all the requirements for cluster installation are met
fluvio cluster releases list      Show a list of Fluvio release versions
fluvio cluster spu register       Register a new custom SPU with the cluster
fluvio cluster spu unregister     Unregister a custom SPU from the cluster
fluvio cluster spu list           List all SPUs known by this cluster (managed AND custom)
fluvio cluster spg create         Create a new managed SPU Group
fluvio cluster spg delete         Delete a managed SPU Group
fluvio cluster spg list           List all SPU Groups
fluvio cluster run sc             Run a new Streaming Controller (SC)
fluvio cluster run spu            Run a new Streaming Processing Unit (SPU)
fluvio install                    Install Fluvio plugins
fluvio update                     Update the Fluvio CLI
fluvio version                    Print Fluvio version information
fluvio help                       Print help for Fluvio or a subcommand
```

### `fluvio consume`

The `fluvio consume` command is a way to read the contents of messages in a Fluvio Topic
from a command-line environment. This can be useful if you are developing an application
with Fluvio and want real-time visibility into what is streaming through your topics.
It can also be handy for writing shell scripts that can read and react to messages in a topic.

Arguments:



#### Next Steps
----------------

*
