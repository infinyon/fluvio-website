---
title: Overview
weight: 10
---

**Fluvio Data Streaming** is a modern Cloud Native software stack designed for **high speed real-time data** processing. The software was designed to be _fast_, _scalable_, _self-healing_, _pluggable_, and _user-friendly_.

#### Built in Rust

Fluvio is **built in {{< target-blank title="Rust" url="https://www.rust-lang.org/" >}}**, a system programming language with **higher performance** than Java and **better code safety** than C/C++. Rust has a powerful multi-threaded asynchronous engine that runs natively in multi-core and low powered embedded systems. It has zero cost abstraction and **no garbage collection**, ideal for low network latency and high IO throughput distributed system products.

The choice of programming language makes Fluvio a low memory, high performance product that **compiles natively** in many software distributions, from MacOS, Linux, and Windows, to small footprint embedded systems such as {{< target-blank title="Raspberry Pi" url="https://www.raspberrypi.org/" >}}.


## High Level Architecture

Fluvio architecture is centered around **high speed data** processing, where multiple endpoints exchange a large volume of data in **real time**. To address this challenge, Fluvio was designed for **horizontal scale**.

{{< image src="sc-spu.png" alt="Fluvio - Cloud Streaming" justify="center" width="auto" type="scaled-90">}}

**Streaming Controller (SC)** is the central coordinator responsible for the size of the cluster and data stream distribution across **Streaming Processing Units (SPUs)**.

 
### Streaming Controller (SC)

Fluvio was designed to address a variety of deployment scenarios from private data centers and public clouds, to edge and IOT devices. **Streaming Controller (SC)** is the central coordinator responsible for the **topology map** and the overall **traffic distribution**.

{{< image src="cloud-edge-iot.svg" alt="DC, Cloud, Edge, IoT" justify="center" width="580" type="scaled-90">}}

When a producer/consumer join the cluster, the **SC** ensures it is routed to the appropriate **SPU** for data processing.

The decision to separate the **SC** from the **SPU** has the following benefits:

* clean division of responsibility
    * SC handles control plane concerns
    * SPU handles data plane concerns
* deployment flexibility
* SC and SPU can scale independently

For a deep dive in the SC design, checkout the [SC section]({{< relref "SC" >}}).


### Streaming Processing Unit

**Streaming Processing Unit (SPU)** is the most important component of the architecture, it is responsible for all data streaming related matters. It receives data from producers, serves data to consumers, saves copies of the data to local storage, sends copies of the data to peer SPUs.

Each SPU performs all these operations **on multiple data streams** in parallel by utilizing all available system CPU cores. 


### Designed for Kubernetes

...

### Deployment Models

Fluvio architecture is designed to cover various deployment models from cloud to edge, mobile to IOT devices, in any combination. This diversity in deployment models, required clean separation between the **Control** and **Data** planes.

### Streaming APIs

Libraries

* Native Rust API
* Native Node API
* CLI

External APIs

* Control Plane
* Data Plane

Internal APIs

* SC to SPUs
* SPUs to SPUs

Everything is TLS enabled.



<hr />

#### Things to cover:

* horizontal replication
* append-only event streaming
* predictable
* use-friendly (apis, cli)
* expandable (Kubernetes)
    * pluggable in your own ecosystem
* control & data plane separation
* self-healing
    * eventual consistency
    * predictable
