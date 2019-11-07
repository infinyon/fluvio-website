---
title: Overview
---

For the past few years companies made significant investments on building and deploying Microservices to accelerate product development and shorten the time to market. That fostered innovation around container technology, continuous deployment, GitOps, DevOps, Cloud, and Serverless. Challenges around deployment, networking, security, and monitoring are gradually addressed. Implementing distributed data across services remains a laborious task. 

{{< image src="monolithic-2-microservices.svg" alt="Monolith to Microservices" justify="center" width="500" type="scaled-90">}}

Companies start small and gradually move to larger more complex Apps. As they gain confidence and realize the benefits of Microservices, they start decomposing existing Monolithic Apps. These Monoliths that made the company successful have years of accumulated technical debt and cross cutting concerns. 
 
The Strangler approach is a popular design pattern that incrementally transforms a Monolithic App into Microservices by gradually replacing a particular functionality with a new service. A service is the technical authority for a specific business domain. Any piece of data or rule associated with a domain must be owned by only one service. When services share data over the network, they face a series of distributed data challenges:

* Define data boundaries
* Ensure data consistency
* Perform transactions
* Join data from multiple services

Solutions in the market are custom built with each team embedding their own flavor of distributed data layer into the service logic. The outcome is a diverse set of solutions across teams, services, and programming languages that creates unnecessary **accidental complexity**.

{{<idea>}}
We believe distributed data management should have a _dedicated infrastructure_ decoupled from the business logic that is **shared by all services**. 
{{</idea>}}

In the next section we describe **three models for data handling in Microservices** that gave us the inspiration for building this open source project. 

## "Three Data Models" for Microservices

Monoliths are easy to develop but **lack boundary enforcement** which makes it nearly impossible to keep code modular. As the number of components grows, module interdependencies snowball, the speed of development decreases, and costs of building software skyrockets.

{{< image src="monolith.svg" alt="Monolith" justify="center" width="420" type="scaled-75">}}

Organizations turn to Microservices to reduce cross cutting concerns, gain modularity, and accelerate time to market. In theory, Microservices bring **modularity to business logic and data**, however our research shows organizations gain business logic separation and only partial data separation. 

In practice, organizations use **Three Data Models** in their transition to Microservices:

* [Shared Data Model](#shared-data-model)
* [Distributed Data Model](#distributed-data-model)
* [Event Driven Data Model](#event-driven-data-model)

Before we evaluate the strengths and weaknesses for these three models, we need to take a look at **domains**. A domain gives each service a bounded context, a definition of its purpose in the world and a data model to describes it. Eric Evans coined the term [Domain Driven Design](https://en.wikipedia.org/wiki/Domain-driven_design) where it tackles this topic at length.

The core takeaway is that Microservices should be built around domain driven bounded context. 


### Shared Data Model

A **Shared Data Model** describes Microservices that implement domain centric business logic, communicate over the network, and share the same database. This model is used by conservative teams as a first step towards Microservices or as an intermediate step during a Monolith decomposition.

{{< image src="msvs-networked.svg" alt="Networked Microservices" justify="center" width="420" type="scaled-75">}}

#### Advantages 

**Shared Data Model** is easy to adopt as it has the same data access characteristics of its older brother, the Monolith. The main advantages of this model are:

* Encapsulation for domain specific business logic
* API interface for service definition
* Same data handling code for transactions, joins, etc.

While not the end goal, it is a reasonable first step in the journey to Microservices. 

#### Disadvantages 

Data is still a Monolith and direct access to data from all services has the following disadvantages:

* Service API can be bypassed
* Data format changes require cross team coordination

#### Conclusion 

The **Shared Data Model** achieves clean business logic separation, but favors ease of use over data segregation. It may be suitable for small Apps or but in general it is an intermediate step towards a completely segregated Microservice.


### Distributed Data Model

**Distributed Data Model** is used when Microservices own the business logic as well as the data for its bounded context. In this model, the data is gated by the service interface and only accessible through APIs.

{{< image src="distributed-data.svg" alt="Microservices with Distributed Data" justify="center" width="420" type="scaled-75">}}


#### Advantages 

* Data is owned by the same Microservices team
* Clean bounded context
* Clean separation between access and data format 


#### Disadvantages

* Service availability impacts data availability
* Difficult to optimize for large data sets
* Distributed transactions manager required
* Requires a solution for data consistency 

A detailed document is available to discuss all these issues at depth [link](link).

#### Conclusions

**Distributed Data Model** provides clean bounded context for business logic and data but brings along a series of distributed data challenges. These 


### Event Driven Data Model

**Event Driven Data Model** decouples services from data availability.  availability challenges for distributed data.

Microservices communicate with each other over Event Streams.

{{< image src="msvs-event-driven.svg" alt="Event Driven Microservices" justify="center" width="420" type="scaled-75">}}


## Distributed Data Infrastructure (DDI) Introduction

**Distributed Data Infrastructure (DDI)** is a **standards-based**, **language agnostic** software that glues services to distributed data. It allows services to share data without coding the data layer itself.

The benefits are shorter time to market, better code quality, and the ability to add out of band data management features such as role based access, auditing, monitoring, and troubleshooting.

{{< image src="ddi.svg" alt="Custom vs. DDI" justify="center" width="700" type="scaled-90">}}

The **Distribute Data Infrastructure** is:

* **Language agnostic** - use your favorite language, Java, C++, TypeScript, Go, Scala, Ruby, etc.
* **Versatile** - covers many distributed data data use cases from CQRS to Saga out of the box.
* **Painless** - you don't have to be a distributed data wizard to operate it.
* **Expressive** - just define the workflow, the chain of services and the data they exchange, no coding required.
* **Traceable** - track, debug, and monitor any data as it traverses services.
* **Secure** - apply corporate data policy without rebuilding or deploying application code.
* **Compliant** - perform data audits without impacting the application.

The DDI **democratizes** the use of data across the organization.