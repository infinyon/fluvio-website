---
title: Fluvio Cloud (Public Beta)
menu: Cloud Platform
weight: 10
---

Fluvio is an open source, **cloud native** platform designed to work with [Kubernetes]({{< relref "k8-integration" >}}). The platform has two core Microservices, SC and SPU that can be containerized, independently provisioned, and dynamically orchestrated. The platform was designed for **horizontal scale** where new components are seamlessly absorbed without disruption to the overall health of the system.

{{< image src="cloud-native.svg" alt="Fluvio Cloud" justify="center" width="560" type="scaled-90">}}


Fluvio can be deployed in most **public cloud** provider as well and **private data centers**. 


## Fluvio Cloud

Fluvio Cloud is a **Data Streaming as a Service (DSaaS)** platform manged by the Fluvio team. Each account receives a dedicated [Fluvio Open Source]({{< relref "../fluvio-oss/oss-platform/" >}}) installation in our public cloud.

The installation is provisioned with 1 x [Streaming Controller]({{< relref "../architecture/sc/" >}}) (SC) and 3 x [Streaming Processing Units]({{< relref "../architecture/spu/" >}}) (SPU). 

{{< image src="fluvio-cloud.svg" alt="Fluvio Cloud" justify="center" width="560" type="scaled-90">}}

Each SPU is provisioned with ...
* ...
* ...
* ...

Checkout [Quick Start]({{< relref "../getting-started/quick-start#create-a-fluvio-cloud-account" >}}) on instruction on how to create a cloud account. 


### Data Streaming to Cloud

There are currently 3 ways to stream real-time data to **Fluvio Cloud**, through:
* [CLI]({{< relref "../cli/overview" >}})
* [Node App]({{< relref "../getting-started/build-node-app" >}})
* [Rust App]({{< relref "../getting-started/build-rust-app" >}})

Data streaming clients require a [profile file]({{< relref "../cli/profiles" >}}) that contains the authorization information associated the cluster. The profile is generated during cluster setup and it is available for download in your {{< target-blank title="Fluvio Cloud" url="https://app.fluvio.io" >}} account.


{{< links "Related Topics" >}}
* [Quick Start]({{< relref "../getting-started/quick-start" >}})
* [CLI]({{< relref "../cli/overview" >}})
* [Architecture]({{< relref "../architecture/overview" >}})
{{< /links >}}