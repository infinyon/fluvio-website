---
title: Architecture Overview
menu: Overview
weight: 10
---

**Fluvio Data Streaming** is a modern Cloud Native software stack designed for **high speed real-time data** processing. The software was designed to be _fast_, _scalable_, _self-healing_, _pluggable_, and _user-friendly_.

#### Built in Rust

Fluvio is **built in {{< target-blank title="Rust" url="https://www.rust-lang.org/" >}}**, a system programming language with **higher performance** than Java and **better code safety** than C/C++. Rust has a powerful multi-threaded asynchronous engine that runs natively in multi-core and low powered embedded systems. Zero cost abstraction and **no garbage collection**, makes this language ideal for low network latency and high IO throughput distributed system products.

The choice of programming language makes Fluvio a low memory, high performance product that **compiles natively** in many software distributions, from MacOS, Linux, and Windows, to small footprint embedded systems such as {{< target-blank title="Raspberry Pi" url="https://www.raspberrypi.org/" >}}.


## High Level Architecture

Fluvio architecture is centered around **high speed data** processing, where multiple endpoints exchange a large volume of data in **real time**. To address this challenge, Fluvio was designed for **horizontal scale**.

{{< image src="sc-spu.svg" alt="Architecture Components - SC/SPUs" justify="center" width="440" type="scaled-90">}}

**Streaming Controller (SC)** is the central coordinator responsible for the size of the cluster and data stream distribution across **Streaming Processing Units (SPUs)**.

 
### Streaming Controller (SC)

Fluvio was designed to address a variety of deployment scenarios from private data centers and public clouds, to edge and IOT devices. **Streaming Controller (SC)** is the central coordinator responsible for the **topology map** and the overall **traffic distribution**.
When producers and consumers join a cluster, the **SC** ensures they are routed to the appropriate **SPU** for data processing.

{{< image src="cloud-edge-iot.svg" alt="DC, Cloud, Edge, IoT" justify="center" width="580" type="scaled-90">}}

SC and SPU were designed as **independent**, **loosely coupled** services. Each service can be **restarted**, **upgraded**, or **scaled** independently without impacting traffic. The ability to change the **topology map dynamically** allows us to simplify complex tasks such as increasing capacity, adding new infrastructure, or attaching a new geo-locations.

For a deep dive in the SC design, checkout the [SC section]({{< relref "SC" >}}).


### Streaming Processing Unit

**Streaming Processing Units (SPUs)** are the most important components of the architecture. They are responsible for all data streaming related matters. Each SPU **receives** data from producers, **sends** data to consumers, and **saves** copies of the data to local storage.

{{< image src="spus.svg" alt="SPU produce/consume & replication" justify="center" width="380" type="scaled-75">}}

SPUs are also responsible for **data replication**. Data streams that are created with a __replication factor__ of 2 or more are managed by __a cluster__ of SPUs. One SPU is elected as leader and all others are followers. The leader receives the data from consumers and forwards a copy to followers. Followers save a copy in their local storage. If the leader goes offline, one of the followers takes over as leader. For additional information, checkout the [Replication section]({{< relref "replication" >}}).

Each SPU performs **leader** and **follower** duties **on multiple data streams** in parallel. For optimal performance, Fluvio utilizing all available **CPU cores**. 
For a deep dive in the SPU design, checkout the [SPU section]({{< relref "SPU" >}}).

### Topic/Partitions

Data Streams are provisioned through **topics**. Each topic has one or more partitions and a replication factor. A **topic/partition pair** creates a **unique identifier** for a data stream. The **replication factor** specifies the number a copies each topic/partition should be have.

{{< image src="topic-partition.svg" alt="Topic/Partitions" justify="center" width="620" type="scaled-90">}}



### Streaming APIs

Fluvio architecture places strong emphasis on ease of use. From the user point of view it translates into well designed and documented APIs.
In addition, Fluvio aims to offer native integrations in most common programming languages.

{{< image src="external-api.svg" alt="External APIs" justify="center" width="500" type="scaled-90">}}


{{< image src="internal-api.svg" alt="Internal APIs" justify="center" width="440" type="scaled-90">}}

test


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

{{< links >}}
* [SC Design]({{<relref "sc">}})
* [SPU Design]({{<relref "spu">}})
* [Topic/Partition Design]({{<relref "topic-partition">}})
* [Replication Design]({{<relref "replication">}})
* [Kubernetes Integration Design]({{<relref "k8-integration">}})
* [Deployment Models]({{<relref "deployments">}})
{{< /links >}} 
