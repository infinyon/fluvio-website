---
title: Fluvio CLI Reference
menu: Overview
section: CLI
---

The Fluvio CLI is an all-in-one tool for installing and managing Fluvio,
as well as for producing and consuming data on the command-line. If you
have not installed it already, you may do so using the following command:

%copy first-line%
```bash
$ curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

Let's have a look at some common commands you'll want to know when starting
out with Fluvio:

## Commands to Know

#### Setting up a cluster

- [Create your own Fluvio cluster with `fluvio cluster start`](/cli/commands/cluster#fluvio-cluster-start)
- [Connect to your Fluvio Cloud cluster with `fluvio cloud login`](/cli/plugins/cloud#fluvio-cloud-login)
  
#### Sending and Receiving data from Fluvio

- [Create a topic with `fluvio topic create`](/cli/commands/topic#fluvio-topic-create)
- [Produce data to a topic with `fluvio produce`](/cli/commands/produce#fluvio-produce)
- [Consume data from a topic with `fluvio consume`](/cli/commands/consume#fluvio-consume)

#### Enriching data with SmartStreams

- [Quick start for building and running SmartStreams](/docs/smartstreams/quick-start)
- [Write a custom filtering SmartStream](/docs/smartstreams/filter)
- [Consume enriched data using SmartStreams](/cli/commands/consume#example-3-consume-using-a-smartstream)

#### Viewing the status of the cluster

- [See all of your topics with `fluvio topic list`](/cli/commands/topic#fluvio-topic-list)
- [See your partitions and data replication with `fluvio partition list`](/cli/commands/partition#fluvio-partition-list)
- [See the status of the SPUs in your cluster with `fluvio cluster spu list`](/cli/commands/cluster#fluvio-cluster-spu-list)
