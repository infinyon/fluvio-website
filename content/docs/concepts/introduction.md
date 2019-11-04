---
title: Overview
---

For the past few years companies made significant investments on building and deploying Microservices to accelerate product development and shorten the time to market. That fostered innovation around container technology, continuous deployment, GitOps, DevOps, Cloud, and Serverless. Challenges around deployment, networking, security, and monitoring have been addressed. Implementing distributed data across services remains a laborious task. 

{{< image src="monolithic-2-microservices.svg" alt="Monolith to Microservices" justify="center" width="500" type="scaled-90">}}

Companies start small and gradually move to larger more complex Apps. As they gain confidence and realize the benefits of Microservices, they start decomposing existing Monolithic Apps. These Monoliths that made the company successful have years of accumulated technical debt and cross cutting concerns. 
 
The Strangler approach is a popular design pattern that incrementally transforms a Monolithic App into Microservices by gradually replacing a particular functionality with a new service. A service is the technical authority for a specific business domain. Any piece of data or rule associated with a domain must be owned by only one service. When services share data over the network, they face a series of distributed data challenges:

* Define data boundaries
* Ensure data consistency
* Perform transactions
* Join data from multiple services

Solutions in the market are custom built with each team embedding their own flavor of distributed data layer into the service logic. The outcome is a diverse set of solutions across teams, services, and programming languages that creates unnecessary **accidental complexity**.

We believe distributed data management should have a _dedicated infrastructure_ decoupled from the business logic and **shared by all services**. 


## What is Data Mesh?

**Data Mesh** is a **standards-based**, **language agnostic** infrastructure that glues services to distributed data. Data Mesh gives services the ability to share data without coding the data layer itself.

The benefits are shorter time to market, better code quality, and the ability to add out of band data management features such as role based access, auditing, monitoring, and troubleshooting.

{{< image src="data-mesh.svg" alt="Custom vs. Data Mesh" justify="center" width="700" type="scaled-90">}}

The **Data Mesh** is:

* **Language agnostic** - use your favorite language, Java, C++, TypeScript, Go, Scala, Ruby, etc.
* **Versatile** - covers many distributed data data use cases from CQRS to Saga out of the box.
* **Painless** - you don't have to be a distributed data wizard to operate it.
* **Expressive** - just define the workflow, the chain of services and the data they exchange, no coding required.
* **Traceable** - track, debug, and monitor any data as it traverses services.
* **Secure** - apply corporate data policy without rebuilding or deploying application code.
* **Compliant** - perform data audits without impacting the application.

Data Mesh is a powerful middle layer that **democratizes** the use of data across the organization.

### SubSection 2.1
...


