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

Solutions in the market are custom built and embedded in the implementation of each service. That brings **accidental complexity** to each service across different teams and programming languages.

We believe, this functionality should have its own dedicated infrastructure, a "Data Mesh" that is **language agnostic** and **shared** by all services. **Data Mesh is the glue that connects services to distributed data**.


## Introduction to Data Mesh

**Data Mesh** is a language agnostic dedicated infrastructure that allows services to share data without coding the data layer itself. 

how? call apis... etc...

{{< image src="data-mesh.svg" alt="Custom vs. Data Mesh" justify="center" width="700" type="scaled-90">}}

The benefits are better code quality, accelerate development, and the ability to add out of band data management for access, auditing, and monitoring.


* Language agnostic. You can use your favorite language such as Java, C++, TypeScript, Go, Scala, Ruby, and etc.
* Codeless.  Everyone can communicate their use case and workflow without writing code.
* Democratization.  Anyone can operate the data mesh without requiring expertise in distributed data.
* Governance.  We can perform centralized policy enforcement without rebuilding or deploying application code.
* Monitoring.  Debugging and monitoring is much easier since there is common model.
*



### SubSection 2.1
...


