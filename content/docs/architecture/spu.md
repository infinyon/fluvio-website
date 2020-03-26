---
title: Streaming Processing Unit (SPU) Design
menu: SPU
weight: 30
---

In addition to being an active coordinator for endpoint communication, SPUs also play a passive role. SPUs in an active role are called leaders and in a passive role followers. 

{{< image src="spu-architecture.svg" alt="SPU Architecture" justify="center" width="600" type="scaled-98">}}

SPU was designed for horizontal scale.

SPU is authoritative for all Leader/Follower election status.

#### Default Ports

Fluvio SPU has a **public** and a **private** server that are attached to the following ports:

* **Public Port**: 9005
* **Private Port**: 9006


## Outline ....

Metadata Dispatcher

* Leader Controller
* Follower Controller

Action Dispatcher

* Strorage Module

