---
title: Fluvio Cloud (Public Alpha)
menu: Cloud Platform
weight: 10
---

Fluvio cloud is currently in alpha and we are working diligently to fix any issues that may arise. For support, please notify us in <a href="https://discordapp.com/invite/bBG2dTz" target="_blank">Discord</a>.


## Fluvio Cloud

Fluvio Cloud is a **Data Streaming as a Service (DSaaS)** platform manged by the Fluvio team. Each account receives a dedicated [Fluvio Open Source]({{< relref "../fluvio-oss/oss-platform/" >}}) installation in our public cloud.

The installation is provisioned with 1 x [Streaming Controller]({{< relref "../architecture/sc/" >}}) (SC) and 3 x [Streaming Processing Units]({{< relref "../architecture/spu/" >}}) (SPU). 

{{< image src="architecture/fluvio-cloud.svg" alt="Fluvio Cloud" justify="center" width="560" type="scaled-90">}}

Checkout [Quick Start]({{< relref "../getting-started/quick-start#create-a-fluvio-cloud-account" >}}) on instruction on how to create a cloud account. 


### Data Streaming to Cloud

There are currently 3 ways to stream real-time data to **Fluvio Cloud**, through:
* [CLI]({{< relref "../cli/overview" >}})
* [Node App]({{< relref "../getting-started/build-node-app" >}})
* [Rust App]({{< relref "../getting-started/build-rust-app" >}})

Data streaming clients require a [profile file]({{< relref "../cli/profiles" >}}) that contains the authorization information associated with the cluster. The profile is generated during cluster setup and it is available for download in your <a href="https://app.fluvio.io" target="_blank">Fluvio Cloud</a> account.

All data between clients and Fluvio Cloud is encrypted using <a href="https://en.wikipedia.org/wiki/Transport_Layer_Security" target="_blank">TLS</a>.

### Restrictions

Each installation supports an aggregate of **1Gb** of data streams and **3 SPUs**.

#### Storage 

When the **1GB** storage limit is reached, older data is purged to make room for new data. Note that data streams are replicated by default. Loss of 1 or 2 SPUs will not impact system operation.

#### SPUs 
Cloud environment does not allow changes to the number of SPUs. 

Later versions will provide the ability to control **storage** and **SPU** configurations.


#### Related Topics
-------------------
* [Quick Start]({{< relref "../getting-started/quick-start" >}})
* [CLI]({{< relref "../cli/overview" >}})
* [Architecture]({{< relref "../architecture/overview" >}})
