---
title: Overview
weight: 10
aliases:
    - /docs/architecture
---

**Fluvio Data Streaming** is a modern Cloud Native software stack designed for **high speed, real-time data** processing. Fluvio is _fast_, _scalable_, _self-healing_, _pluggable_, and _user-friendly_.

#### Built in Rust

Fluvio is **built in <a href="https://www.rust-lang.org/" target="_blank">Rust</a>**, a systems programming language with **higher performance** than Java and **better code safety** than C/C++. Rust has a powerful multi-threaded asynchronous engine that runs natively in multi-core and low powered embedded systems. Zero cost abstractions and **no garbage collection** makes this language ideal for low network latency and high IO throughput systems.

The choice of programming language makes Fluvio a low memory, high performance product that **compiles natively** in many software distributions such as MacOS, Linux, Windows, and small footprint embedded systems such as <a href="https://www.raspberrypi.org/" target="_blank">Raspberry Pi</a>.

#### Cloud Native by Design

Fluvio is a **Cloud Native** platform designed to work with any infrastructure type from bare bones hardware to containerized platforms. As a **Cloud Native** first product, Fluvio is natively integrated with **<a href="https://kubernetes.io" target="_blank">Kubernetes</a>**. Any infrastructure running **Kubernetes** can install the **Fluvio Helm Chart** and get up and running in a matter of minutes. For additional details, check out the [Kubernetes install]({{< ref "/docs/kubernetes/install" >}}) section. 

#### Fluvio Cloud

If you don't have Kubernetes installed or prefer to run **Fluvio as a Service**, you can use **[Fluvio Cloud]({{< ref "/docs/get-started/cloud" >}})**. The cloud installation hides all the complexity associated with the infrastructure and exposes only relevant streaming APIs. Follow our Getting Started guide (for [MacOS] or [Linux]) to set up your dedicated cloud environment.

[MacOS]: {{< ref "/docs/get-started/mac" >}}
[Linux]: {{< ref "/docs/get-started/linux" >}}

## High Level Architecture

Fluvio's architecture centers around **real time streaming**, and the platform can **scale horizontally** to accommodate large volumes of data.

<img src="../images/sc-spu.svg"
     alt="Architecture Components - SC/SPUs"
     style="justify: center; max-width: 500px" />

A **Streaming Controller (SC)** manages the **SPU life cycle** and optimizes the distribution of data streams across the cluster. The **Streaming Processing Units (SPUs)** are responsible for data streaming.

SCs and SPUs are **independent**, **loosely coupled** services. Each service can be **restarted**, **upgraded**, or **scaled** independently without impacting traffic. 

 
## Streaming Controller (SC)

Fluvio is designed to address a variety of **deployment scenarios** from public clouds to private data centers, edge networks and IOT devices. **SC** maintains the **topology map** of the **SPUs** and serves as the first point of contact for producers and consumers.

<img src="../images/cloud-edge-iot.svg"
     alt="DC, Cloud, Edge, IoT"
     style="justify: center; max-width: 580px" />

The **SC** handles **topology map dynamically** to simplify complex tasks such as increasing capacity, adding new infrastructure, or attaching a new geo-locations.

For a deep dive in the SC design, checkout the [SC Architecture] section.

[SC Architecture]: {{< ref "/docs/architecture/sc" >}}

## Streaming Processing Unit (SPU)

**Streaming Processing Units (SPUs)** are responsible for all data streaming related matters. Each SPU **receives** data from producers, **sends** data to consumers, and **saves** copies of the data to local storage.

<img src="../images/spus.svg"
     alt="SPU produce/consume & replication"
     style="justify: center; max-width: 330px" />

SPUs are also responsible for **data replication**. Data streams that are created with a __replication factor__ of 2 or more are managed by __a cluster__ of SPUs. One SPU is elected as leader and all others are followers. The leader receives the data from producers and forwards a copy to followers. Followers save a copy in their local storage. If the leader goes offline, one of the followers takes over as leader. For additional information, check out [Replica Election].

[Replica Election]: {{< ref "replica-election" >}}

Each SPU performs **leader** and **follower** duties **on multiple data streams** in parallel. For optimal performance, Fluvio utilizes all available **CPU cores**. 

For a deep dive into the SPU design, check out the [SPU Architecture] section.

[SPU Architecture]: {{< ref "/docs/architecture/spu" >}}

## Topic/Partitions

A Topic in a streaming platform is like a Table in a database. Suppose you're building an online chat service.
You may have a "Chatroom" topic that streams events such as "Sent Message" and "Viewed Message", as well as other
related events. You can think of a Topic as a category of events that are related under your domain.

Now suppose that your chat service becomes a wild success and you have thousands of users. In order to keep up
with the increased traffic, you can divide your topic into multiple Partitions. This allows the events in your
topic to be distributed between multiple SPUs in parallel, increasing your traffic capacity by just changing a setting.

For example, a configuration with the 2 topics generates the replication map in the diagram:

* **topic-a** => 2 partitions, 2 replicas 
* **topic-b** => 1 partition, 3 replicas

SPU-1 is the leader for **topic-a/0** , SPU-2 for **topic-a/1**, and SPU-3 for **topic-b/0**.

<img src="../images/topic-partition.svg"
     alt="Topic/Partitions"
     style="justify: center; max-width: 650px" />

For additional information on partitions and replica assignments, checkout [Replica Assignment].

[Replica Assignment]: {{< ref "replica-assignment" >}}

## Data Persistence

SPU leaders **save** all data stream messages received from producers on **local storage**. Based on platform availability, SPUs use **zero-copy IO** to transfer data from disk to network. Messages on local storage are **immutable** and **ordered**. Fluvio guarantees **in-order writes** for all messages received on the same **replica**.

<img src="../images/storage.svg"
     alt="Data Storage"
     style="justify: center; max-width: 720px" />

SPU persistence is designed as **single-writer, multi-reader** with **zero-copy writes**. Each SPU can save large volumes of data at **wire speed**, and serve consumers and producers in **near real-time**.  

Fluvio adds messages local storage until the **retention period** is met. The retention periods should be set to cover **up to 80%** of the disk size. If the disk is full before the retention period is triggered, the SPU stops accepting messages and the overall health of the system may be compromised.

## APIs

The Fluvio architecture places heavy emphasis on clean, **user-friendly APIs**. There are two types of APIs, **external** and **internal**. The APIs use **TLS** to ensure secure communication. 

### External APIs

**External APIs** are used by the **Fluvio CLI** and a growing number of programming language interfaces, such as **Node** and **Rust**. There are two categories of APIs, control plane APIs and data plane APIs. **Control Plane APIs** manage the life cycle of the cluster objects such as SPUs, topics, and replicas. **Data Plane APIs** handle data access for producers and consumers.

<img src="../images/external-api.svg"
     alt="External APIs"
     style="justify: center; max-width: 500px" />

API reference guides for programming languages are available at: 

* <a href="https://infinyon.github.io/fluvio-client-node/" target="_blank">Node API</a> 
* <a href="https://docs.rs/fluvio/" target="_blank">Rust API</a>
* <a href="https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/package-summary.html" target="_blank">Java API</a>
* <a href="https://infinyon.github.io/fluvio-client-python/fluvio.html" target="_blank">Python API</a>

### Internal APIs

**Internal APIs** are used by the **SC** communicate with the **SPUs** and for the **SPUs** to communicate with their peers to elect leaders and exchange replica information. 

<img src="../images/internal-apis.svg"
     alt="Internal APIs"
     style="justify: center; max-width: 500px" />

For additional details about **Internal APIs** checkout Fluvio development guide on <a href="https://github.com/infinyon/fluvio" target="_blank">github</a>.
