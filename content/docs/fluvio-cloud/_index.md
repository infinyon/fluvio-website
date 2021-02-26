---
title: Fluvio Cloud
folder: Fluvio Cloud
menu: Overview
weight: 100
---

Fluvio Cloud is a fully managed <a href="Github https://github.com/infinyon/fluvio/" target="_blank">Fluvio Open Source</a> provisioned and maintained by InfinyOn team. Fluvio Cloud is currently in alpha and you may face **instability** and **data loss between upgrades**.

-> The Fluvio community values your time and appreciates your feedback. Our pledge is to be actively engaged with Fluvio users and respond to issues and suggestions in the shortest possible time.

Please use the following channels for issues and recommendation:

* <a href="https://discordapp.com/invite/bBG2dTz" target="_blank">Discord</a> - for chat
* <a href="https://github.com/infinyon/fluvio/issues" target="_blank">Github</a> - for tracking issues

For step-by-step instruction, checkout [Getting Started with Fluvio Cloud](/docs/getting-started/fluvio-cloud).

## Fluvio Cloud

Each cloud account receives a dedicated  <a href="https://github.com/infinyon/fluvio" target="_blank">Fluvio Open Source</a> installation provisioned with 1 x [Streaming Controller](/docs/architecture/sc) (SC) and 1 x [Streaming Processing Units](/docs/architecture/spu) (SPU). 

{{< image src="fluvio-cloud/fluvio-cloud.svg" alt="Fluvio Cloud" justify="center" width="360" type="scaled-90">}}

[Getting Started](/docs/getting-started/#create-a-fluvio-cloud-account) provides a step-by-step guide on how to create a Fluvio Cloud account and stream "Hello World". 


### Data Streaming to Cloud

There are currently three interfaces to stream real-time data to Fluvio Cloud:
* [CLI](/docs/cli-reference)
* <a href="https://infinyon.github.io/fluvio-client-node/" target="_blank">Node API</a>
* <a href="https://docs.rs/fluvio/" target="_blank">Rust API</a>

All clients require a [Profile](/docs/cli/profiles) to stream data to and from a Fluvio cluster. The profile contains the authorization and access information the client uses to register and communicate with a cluster. All data between clients and Fluvio Cloud is **encrypted** in <a href="https://en.wikipedia.org/wiki/Transport_Layer_Security" target="_blank">TLS</a>.

The profile is generated during cluster setup and it is available for download in your <a href="https://cloud.fluvio.io" target="_blank">Fluvio Cloud Dashboard</a> account.


### Restrictions

Each installation supports an aggregate of **1Gb** of data streams and **1 SPU**.

#### Storage 

When the **1GB** storage limit is reached, older data is purged to make room for new data. Note that data streams are replicated by default. Loss of 1 or 2 SPUs will not impact system operation.

#### SPUs 
Cloud environment does not allow changes to the number of SPUs. 

Later versions will provide the ability to control **storage** and **SPU** configurations.


#### Related Topics
-------------------
* [Getting Started](/docs/getting-started)
* [Fluvio Architecture](/docs/architecture)