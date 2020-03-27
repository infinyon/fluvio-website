---
title: Architecture Overview
menu: Overview
weight: 10
---

**Fluvio Data Streaming** is a modern Cloud Native software stack designed for **high speed real-time data** processing. The software was designed to be _fast_, _scalable_, _self-healing_, _pluggable_, and _user-friendly_.

#### Built in Rust

Fluvio is **built in {{< target-blank title="Rust" url="https://www.rust-lang.org/" >}}**, a system programming language with **higher performance** than Java and **better code safety** than C/C++. Rust has a powerful multi-threaded asynchronous engine that runs natively in multi-core and low powered embedded systems. Zero cost abstraction and **no garbage collection**, makes this language ideal for low network latency and high IO throughput systems.

The choice of programming language makes Fluvio a low memory, high performance product that **compiles natively** in many software distributions such as MacOS, Linux, Windows, and small footprint embedded systems such as {{< target-blank title="Raspberry Pi" url="https://www.raspberrypi.org/" >}}.

#### Cloud Native by Design

Fluvio is a **Cloud Native** platform designed to work with any infrastructure type from bare bones hardware to containerized platforms. As a **Cloud Native** first product, Fluvio is natively integrated with **{{< target-blank title="Kubernetes" url="https://kubernetes.io" >}}**. Any infrastructure running **Kubernetes** can install **Fluvio Helm Chart** and get up and running in a matter of minutes. For additional details, [Kubernetes integration]({{< relref "k8-integration" >}}) section. 

#### Fluvio Cloud

If you don't have Kubernetes installed or prefer to run **Fluvio as a Service**, you can use **[Fluvio Cloud]({{< relref "../fluvio-cloud/cloud-platform" >}})**. The cloud installation hides all complexity associated with the infrastructure and exposes only relevant streaming APIs. Use the **[Quick Start]({{< relref "../getting-started/quick-start" >}})** guide to setup your own dedicated cloud environment.


## High Level Architecture

Fluvio architecture is centered around **real time streaming** where the platform can **scale horizontally** to accommodate large volumes of data.

{{< image src="sc-spu.svg" alt="Architecture Components - SC/SPUs" justify="center" width="440" type="scaled-90">}}

**Streaming Controller (SC)** manages **SPU life cycle** and optimizes **data stream distribution** across the cluster. **Streaming Processing Unit (SPU)** is responsible for data streaming.

SCs and SPUs are **independent**, **loosely coupled** services. Each service can be **restarted**, **upgraded**, or **scaled** independently without impacting traffic. 

 
### Streaming Controller (SC)

Fluvio is designed to address a variety of **deployment scenarios** from public clouds to private data centers, edge networks and IOT devices. **SC** maintains the **topology map** of the **SPUs** and serves as the first point of contact for producers and consumers.

{{< image src="cloud-edge-iot.svg" alt="DC, Cloud, Edge, IoT" justify="center" width="580" type="scaled-90">}}

The **SC** handles **topology map dynamically** to simplify complex tasks such as increasing capacity, adding new infrastructure, or attaching a new geo-locations.

For a deep dive in the SC design, checkout the [SC]({{< relref "SC" >}}) section.


### Streaming Processing Unit

**Streaming Processing Units (SPUs)** are responsible for all data streaming related matters. Each SPU **receives** data from producers, **sends** data to consumers, and **saves** copies of the data to local storage.

{{< image src="spus.svg" alt="SPU produce/consume & replication" justify="center" width="330" type="scaled-60">}}

SPUs are also responsible for **data replication**. Data streams that are created with a __replication factor__ of 2 or more are managed by __a cluster__ of SPUs. One SPU is elected as leader and all others are followers. The leader receives the data from consumers and forwards a copy to followers. Followers save a copy in their local storage. If the leader goes offline, one of the followers takes over as leader. For additional information, checkout the [Replication]({{< relref "replication" >}}) section.

Each SPU performs **leader** and **follower** duties **on multiple data streams** in parallel. For optimal performance, Fluvio utilizing all available **CPU cores**. 
For a deep dive in the SPU design, checkout the [SPU]({{< relref "SPU" >}}) section.

### Topic/Partitions

**Topics** define groups of partitions. **Partitions** split the data into independent slices to allow producers and consumers to write and read messages in parallel. Each partition can be **replicated** across multiple SPUs. A **topic name** and **partition index** pair uniquely identifies a **partition**, which is also known as a **data stream**.

For example, a configuration with the 2 topics generates the replication map in the diagram:

* **topic-a** => 2 partitions, 2 replicas 
* **topic-b** => 1 partition, 3 replicas

SPU-1 is the leader for **topic-a/0** , SPU-2 for **topic-a/1**, and SPU-3 for **topic-b/0**.

{{< image src="topic-partition.svg" alt="Topic/Partitions" justify="center" width="650" type="scaled-90">}}

For additional information on partitions and replica assignments, checkout the [Replication]({{< relref "replication" >}}) section.


### Data Persistence

SPU leaders **save** all data stream messages received from producers on **local storage**. Based on platform availability, SPUs use **zero-copy IO** to transfer data from disk to network. Messages on local storage are **immutable**, and **ordered**. Fluvio guarantees **in-order writes** for all messages received on the same **replica**.

{{< image src="storage.svg" alt="Data Storage" justify="center" width="720" type="scaled-98">}}

Spu persistence was designed as **single-writer, multi-reader** with **zero-copy writes**. Each SPU can save large volumes of data at **wire speed**, and serve consumers and producers in **near real-time**.  

Fluvio adds messages local storage until the **retention period** is met. The retention periods should be set to cover **up to 80%** of the disk size. If the disk is full before the retention period is triggered, the SPU stops accepting messages and the overall health of the system may be compromised.

### Streaming APIs

Fluvio architecture places heavy emphasis on clean **user-friendly APIs**. There are two types of APIs, **external** and **internal**. The APIs use **TLS** to ensure secure communication. 

#### External APIs

**External APIs** are used by the **Fluvio CLI** and a growing number of programming language interfaces, such as  **Node** and **Rust**. There are two categories of APIs, control plane APIs and data plane APIs. **Control Plane APIs** manage the life cycle of the cluster objects such as SPUs, topics, and replicas.  **Data Plane APIs** handle data access for producers and consumers.

{{< image src="external-api.svg" alt="External APIs" justify="center" width="500" type="scaled-75">}}

For language specific API references, checkout [Node API]({{< relref "../node-api/api-reference" >}}) or [Rust API]({{< relref "../rust-api/api-reference" >}}) sections.

#### Internal APIs

**Internal APIs** are used by the **SC** communicate with the **SPUS** and the **SPUs** to communicate with their peers to elect leaders and exchange replica information. 

{{< image src="internal-api.svg" alt="Internal APIs" justify="center" width="500" type="scaled-75">}}

If you'd like to learn more about the **Internal APIs** checkout Fluvio development guide on {{< target-blank title="github" url="https://github.com/infinyon/fluvio" >}}.


{{< links >}}
* [SC Architecture]({{<relref "sc">}})
* [SPU Architecture]({{<relref "spu">}})
* [Replication]({{<relref "replication">}})
* [Kubernetes Integration]({{<relref "k8-integration">}})
* [Deployment Models]({{<relref "deployments">}})
{{< /links >}} 
