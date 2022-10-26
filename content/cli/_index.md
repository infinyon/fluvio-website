---
title: Fluvio CLI Reference
menu: Overview
section: CLI
toc: false
---

The Fluvio CLI is an all-in-one tool for installing and managing Fluvio,
as well as for producing and consuming data on the command-line. If you
have not installed it already, you may do so using the following command:

{{% inline-embed file="embeds/download-cli/curl-bash-copy.md" %}}

Let's have a look at some common commands you'll want to know when starting
out with Fluvio:

## Commands to Know

#### Setting up a cluster

- [Create your own Fluvio cluster with `fluvio cluster start`]({{< ref "/cli/local/cluster#fluvio-cluster-start" >}})
- [Connect to your Fluvio Cloud cluster with `fluvio cloud login`]({{< ref "cli/cloud/overview#fluvio-cloud-login" >}})
  
#### Sending and Receiving data from Fluvio

- [Create a topic with `fluvio topic create`]({{< ref "/cli/commands/topic#fluvio-topic-create" >}})
- [Produce data to a topic with `fluvio produce`]({{< ref "/cli/commands/produce#fluvio-produce" >}})
- [Consume data from a topic with `fluvio consume`]({{< ref "/cli/commands/consume#fluvio-consume" >}})

#### Enriching data with SmartModules

- [Overview for SmartModules]({{< ref "/smartmodules" >}})
- [Write a custom filtering SmartModule]({{< ref "/smartmodules/apis/filter" >}})
- [Consume enriched data using SmartModules]({{< ref "/cli/commands/consume#example-3-consume-using-a-smartstream" >}})

#### Viewing the status of the cluster

- [See all of your topics with `fluvio topic list`]({{< ref "/cli/commands/topic#fluvio-topic-list" >}})
- [See your partitions and data replication with `fluvio partition list`]({{< ref "/cli/commands/partition#fluvio-partition-list" >}})
- [See the status of the SPUs in your cluster with `fluvio cluster spu list`]({{< ref "/cli/local/cluster#fluvio-cluster-spu-list" >}})
