---
title: The Three Stages - from Monolith to Microservices
menu: The Three Stages
weight: 20
---

Monoliths are easy to develop but **lack boundary enforcement** which makes it nearly impossible to keep code modular. As the number of components grows, module interdependencies snowball, the speed of development decreases, and costs of building software skyrockets.

{{< image src="blog/monolith.svg" alt="Monolith" justify="center" width="420" type="scaled-75">}}

Organizations turn to Microservices to reduce cross cutting concerns, gain modularity, and accelerate time to market. In theory, Microservices bring **modularity to business logic and data**, however our research shows organizations gain business logic separation and only partial data separation. 

In practice, organizations go through **Three Stagess** in their transition to Microservices:

* [Stage 1: Shared Data](#stage-1-shared-data)
* [Stage 2: Distributed Data](#distributed-data-model)
* [Stage 3: Event Driven Data](#event-driven-data-model)

Before we deep dive into the three stages, we need to describe **domains**. A domain is a bounded context. A service should have a bounded context to describe its purpose in the world and a data model to describes define ........ Eric Evans coined the term [Domain Driven Design](https://en.wikipedia.org/wiki/Domain-driven_design) where it tackles this topic at length.

The core takeaway is that Microservices should be built around domain driven bounded context. 


## Stage 1: Shared Data

A **Shared Data Model** describes Microservices that implement domain centric business logic, communicate over the network, and share the same database. This model is often used as a first step towards Microservices or as an intermediate step during a Monolith decomposition.

{{< image src="blog/msvs-networked.svg" alt="Networked Microservices" justify="center" width="420" type="scaled-75">}}

### Advantages 

**Shared Data Model** is easy to adopt as it has the same data access characteristics of its older brother, the Monolith. The main advantages of this model are:

* Encapsulation for domain specific business logic
* Service functionality definition through an API
* Same data handling code for transactions, joins, etc.

While not the end goal, it is a reasonable first step in the journey to Microservices. 

### Challenges 

Data is still Monolithic which preserves data coupling leading to the following challenges:

* No clear boundaries for data ownership
* Data format changes require cross team coordination
* Open for abuse, different teams can override same data

### Conclusion 

**Shared Data Model** achieves some business logic separation, but favors data access simplicity over data segregation. This model is often an intermediate step towards fully segregated Microservices. It may also be suitable for small Apps.


## Distributed Data Model

**Distributed Data Model** describes Microservices that own both, the business logic and data for its bounded context. Data shared with other services is accessible exclusively through service APIs.

{{< image src="blog/distributed-data.svg" alt="Microservices with Distributed Data" justify="center" width="420" type="scaled-75">}}


### Advantages 
Microservices using a **Distributed Data Model** gain clean bounded context which has the following advantages:

* Same team is responsible for the business logic, data, and the API interface
* Data access is exposed through APIs
* Internal data changes are decoupled from the API

### Challenges

When Microservices share data over the network through service APIs they face distributed data challenges:

* Service availability impacts data availability
* Large data sets are difficult to optimize
* Distributed transactions require special handling
* Data consistency requires special handling

These challenges are described at length in the following blog "[...](link)".

### Conclusion

While **Distributed Data Model** provides clean bounded context, it exposes a series of distributed data challenge that are addressed below.


## Event Driven Data Model

* Data is not shared with its own api
* Data communicates over Pub/Sub asynchronous pattern.
* We communicate events to solve data consistency problem.

Microservices using **Event Driven Data Model** use databases for internal state but communicate through events over publisher/subscriber streams. Pub/Sub communication mechanism decouples data availability from the service availability, whereas events express an activity at a moment in time. These two mechanisms combined are known as Event Streams. 

**Event Streaming** is a powerful infrastructure layer that enables services to scale horizontally, exchange information in real time and solve many of the challenges in the Distributed Data Model. 

{{< image src="blog/msvs-event-driven.svg" alt="Event Driven Microservices" justify="center" width="420" type="scaled-75">}}

For addition insight in the power of **Event Streaming** checkout our blog at "[...](link)".

### Advantages 

Microservices that use an **Event Driven Data Model** have the following advantages:

* Decouples data availability from service availability
* Captures domain behavior and business intent
* Clean separation for internal and external data (see [Pat Helland](http://cidrdb.org/cidr2005/papers/P12.pdf))
* Suitable for gradual migration of legacy systems (see [Martin Fowler](https://martinfowler.com/articles/evo-arch-forward.html))
* Suitable for real-time services
* Built-in facility for auditing and playback

### Challenges

An **Event Driven Data Model** is a powerful infrastructure that brings along many moving parts which can lead to accidental complexity if not properly addressed:

* Event format and versioning
* Event tracing across services
* Event integration with legacy systems
* Consistent behavior across all services, languages in the app
* Consistent transaction & compensation management
* Multi-service data correlation


### Conclusion

Despite ample evidence that **Event Driven Data Model** is a future proof model for building scalable Microservices, adoption has been slow. There is a lack of off-the-shelf technologies to help organizations roll out and operate an  **Event Driven Data Model** for Microservices.

This observation inspired us to build **Fluvio**, an off-the-shelf solution to simplify the adoption of the **Event Driven Data Model** to deploy, manage, and monitor large scale Microservices Apps. 


## Fluvio - Distributed Data Infrastructure (DDI)

**Distributed Data Infrastructure (DDI)** is an _open source_, _standards-based_, _language agnostic_ software that glues services to distributed data. 

{{< idea >}}
**DDI** abstracts distributed data concerns into a separate layer shared by all Microservices and managed by the _Distributed Data Control Plane_. Application architects use an _EventQL Model_ to describe the interactions between Microservices and the **DDI** does the rest.
{{< /idea >}}

For example, when a transaction is required, the _Model Interpreter_ builds the state machines, the _Data Flow Engine_ applies the SAGA, and the _Event Streaming Engine_ sends the events. If one or more components fail, the engine calls the compensation state machine.

{{< image src="blog/ddi.svg" alt="Fluvio - Distributed Data Infrastructure" justify="center" width="600" type="scaled-90">}}

### Advantages 

A **Distributed Data Infrastructure** allows Microservices to outsource data related concerns which has several the following benefits:

* One consistent distributed data implementation for all services
* Improvements in the DDI layer benefits all services
* Compatible with services written in any programming language
* Easy to define, augment, or modify for multi-service App life cycle
* Centralized governance through the control plane
* Built-in transaction management
* Built-in tracing and monitoring
* Built-in versioning
