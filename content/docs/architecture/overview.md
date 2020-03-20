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

Fluvio is a **Cloud Native** platform designed to work with any infrastructure type from bare bones hardware to containerized platforms. As a **Cloud Native** first product, Fluvio is natively integrated with **{{< target-blank title="Kubernetes" url="https://kubernetes.io" >}}**. Any vendor vendor running **Kubernetes** can install **Fluvio Helm Chart** and get up and running in matter of minutes. For additional details, [Kubernetes integration]({{< relref "k8-integration" >}}) section.


## High Level Architecture

Fluvio architecture is centered around **high speed data** processing, where multiple endpoints exchange a large volume of data in **real time**. To address this challenge, Fluvio was designed for **horizontal scale**.

{{< image src="sc-spu.svg" alt="Architecture Components - SC/SPUs" justify="center" width="440" type="scaled-90">}}

**Streaming Controller (SC)** is the central coordinator responsible for the size of the cluster and data stream distribution across **Streaming Processing Units (SPUs)**.

 
### Streaming Controller (SC)

Fluvio was designed to address a variety of deployment scenarios from private data centers and public clouds, to edge and IOT devices. **Streaming Controller (SC)** is the central coordinator responsible for the **topology map** and the overall **traffic distribution**.
When producers and consumers join a cluster, the **SC** ensures they are routed to the appropriate **SPU** for data processing.

{{< image src="cloud-edge-iot.svg" alt="DC, Cloud, Edge, IoT" justify="center" width="580" type="scaled-90">}}

SC and SPU are **independent**, **loosely coupled** services. Each service can be **restarted**, **upgraded**, or **scaled** independently without impacting traffic. The ability to change the **topology map dynamically** allows us to simplify complex tasks such as increasing capacity, adding new infrastructure, or attaching a new geo-locations.

For a deep dive in the SC design, checkout the [SC]({{< relref "SC" >}}) section.


### Streaming Processing Unit

**Streaming Processing Units (SPUs)** are the most important components of the architecture. They are responsible for all data streaming related matters. Each SPU **receives** data from producers, **sends** data to consumers, and **saves** copies of the data to local storage.

{{< image src="spus.svg" alt="SPU produce/consume & replication" justify="center" width="380" type="scaled-60">}}

SPUs are also responsible for **data replication**. Data streams that are created with a __replication factor__ of 2 or more are managed by __a cluster__ of SPUs. One SPU is elected as leader and all others are followers. The leader receives the data from consumers and forwards a copy to followers. Followers save a copy in their local storage. If the leader goes offline, one of the followers takes over as leader. For additional information, checkout the [Replication]({{< relref "replication" >}}) section.

Each SPU performs **leader** and **follower** duties **on multiple data streams** in parallel. For optimal performance, Fluvio utilizing all available **CPU cores**. 
For a deep dive in the SPU design, checkout the [SPU]({{< relref "SPU" >}}) section.

### Topic/Partitions

**Topics** are the underlying primitives that define Data Streams. Each topic has one or more partitions and a replication factor. A **topic/partition pair** creates a **unique identifier** for each data stream. The **replication factor** is the number of copies desired for each topic/partition. 

For example, a configuration with the 2 topics below generates the replication map in the diagram:

* **topic-1** => 2 partitions, 2 replicas 
* **topic-2** => 1 partition, 3 replicas

SPU-1 is the leader for **topic-1:0** , SPU-2 for **topic-1:1**, and SPU-3 for **topic-2:0**.

{{< image src="topic-partition.svg" alt="Topic/Partitions" justify="center" width="650" type="scaled-90">}}

For additional information on partitions and replica assignment, checkout the [Topic/Partition]({{< relref "topic-partition" >}}) section.


### Data Persistence

SPU leaders **save** all data stream messages received from producers on local storage. The SPUs use **zero-copy** kernel operations to write data to disk. Files are placed in directory structures indexed by **topic/partition** in append-only **immutable queues**. Fluvio guarantees **in-order writes** for all messages on the same topic/partition.

{{< image src="storage.svg" alt="Data Storage" justify="center" width="720" type="scaled-98">}}

Spu persistence was designed as **single-writer, multi-reader** with **zero-copy writes**. Each SPU can save large volumes of data at **wire speed**, and serve consumers and producers in **near real-time**. The system is designed with configurable **retention period** that can span from minutes to days.

Fluvio's advanced persistence design is described in the [Data Persistence]({{< relref "persistence" >}}) section.

### Streaming APIs

Fluvio architecture places heavy emphasis on clean **user-friendly APIs**. There are two types of APIs, **external** and **internal**. 

#### External APIs

**External APIs** are used by the **Fluvio CLI** and a growing number of programming language native interfaces, such as  **Node** and **Rust** to communicate with Fluvio. **External APIs** route provisioning and monitoring requests to the **SC** and producer/consumer requests to the **SPU**.

{{< image src="external-api.svg" alt="External APIs" justify="center" width="500" type="scaled-75">}}

For **Native API** references, checkout [Node API]({{< relref "../node-api/api-reference" >}}) or [Rust API]({{< relref "../rust-api/api-reference" >}}) sections.

#### Internal APIs

**Internal APIs** are used by the **SC** communicate with the **SPUS** and the **SPUs** to communicate with their peers to elect leaders and exchange replica information. 

{{< image src="internal-api.svg" alt="Internal APIs" justify="center" width="500" type="scaled-75">}}

If you'd like to learn more about the **Internal APIs** checkout Fluvio development guide on {{< target-blank title="github" url="https://github.com/infinyon/fluvio" >}}.

Note, both **APIs** are TLS enabled to ensure secure communication. 


{{< links >}}
* [SC Design]({{<relref "sc">}})
* [SPU Design]({{<relref "spu">}})
* [Topic/Partition Design]({{<relref "topic-partition">}})
* [Replication Design]({{<relref "replication">}})
* [Data Persistence]({{<relref "persistence">}})
* [Kubernetes Integration]({{<relref "k8-integration">}})
* [Deployment Models]({{<relref "deployments">}})
{{< /links >}} 
