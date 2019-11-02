---
title: Overview
---

For the past few years companies made significant investments on building and deploying Microservices to accelerate product development and shorten the time to market. That fostered innovation around container technology, continuous deployment, GitOps, DevOps, Cloud, and Serverless. Challenges around deployment, networking, security, and monitoring have been addressed. Implementing distributed data across services remains a laborious task. 

Companies start small and gradually move to larger more complex Apps. As they gain confidence and realize the benefits of Microservices, they start decomposing existing Monolithic Apps. These Monoliths that made the company successful have years of accumulated technical debt and cross cutting concerns. 
 
The Strangler approach is a popular design pattern that incrementally transforms a Monolithic App into Microservices by gradually replacing a particular functionality with a new service. A service is the technical authority for a specific business domain. Any piece of data or rule associated with a domain must be owned by only one service. When services share data over the network, they face a series of distributed data challenges:

* Define data boundaries
* Ensure data consistency
* Perform transactions
* Join data from multiple services

While solutions exits, they are custom built and embedded in the implementation of each service. 
That brings **accidental complexity** to each service across different teams and programming languages.

We believe, this functionality should have its own dedicated infrastructure that is language agnostic and shared by all services. We call this infrastructure layer a **"Data Mesh"**.
The **Data Mesh** spares services from implementing distributed data relate concerns and allows the services to stay focused on the business logic.


## Introduction to Data Mesh 

The **Data Mesh** is a dedicated infrastructure layer that implements distributed data related concerns. The benefits are better code quality, accelerate development, and the ability to add out of band data management for access, auditing, and monitoring.


* Language agnostic
* Enables transactions
* 
* Governanace
* Monitoring
*



### SubSection 2.1
...


