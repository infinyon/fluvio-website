---
title: Fluvio Cloud
menu: Fluvio Cloud
weight: 40
---

Fluvio Cloud is a fully managed <a href="Github https://github.com/infinyon/fluvio/" target="_blank">Fluvio Open Source</a> provisioned and maintained by InfinyOn team. 

-> Fluvio Cloud is currently in **alpha** and is not suitable for production environments.

Please use the following channels for issues and recommendation:

* <a href="https://discordapp.com/invite/bBG2dTz" target="_blank">Discord</a> - for chat
* <a href="https://github.com/infinyon/fluvio/issues" target="_blank">Github</a> - for tracking issues

For step-by-step instruction, checkout [Getting Started with Fluvio Cloud](/docs/getting-started/fluvio-cloud).

## Fluvio Cloud

Each cloud account receives a dedicated <a href="https://github.com/infinyon/fluvio" target="_blank">Fluvio Open Source</a> installation provisioned with 1 x [Streaming Controller]({{< ref "/docs/architecture/cluster.md" >}}) (SC) and 1 x [Streaming Processing Units]({{< ref "/docs/architecture/cluster.md" >}}) (SPU). 

<img src="../images/fluvio-cloud.svg"
     alt="Fluvio Cloud"
     style="justify: center; max-width: 360px" />

### Data Streaming to Cloud

All clients require a [Profile](/docs/cli/profiles) to stream data to and from a Fluvio cluster. The profile contains the authorization and access information the client uses to register and communicate with a cluster. All data between clients and Fluvio Cloud is **encrypted** in <a href="https://en.wikipedia.org/wiki/Transport_Layer_Security" target="_blank">TLS</a>.

The profile is generated during cluster setup and it is available for download in your <a href="https://cloud.fluvio.io" target="_blank">Fluvio Cloud Dashboard</a> account.

#### Storage 

When the **1GB** storage limit is reached, older data is purged to make room for new data.

#### Streaming Processor Units (SPU)

For technical information about SPUs, see [Fluvio Architecture].

Cloud environment does not allow changes to the number of SPUs. 

Later versions will provide the ability to control **storage** and **SPU** configurations.


#### Related Topics
* [Fluvio Architecture]

[Fluvio Architecture]: {{< ref "/docs/architecture/overview.md" >}}