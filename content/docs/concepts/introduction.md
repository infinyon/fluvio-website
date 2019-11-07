---
title: Overview
weight: 10
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


{{< links "Related Topics" >}}
* ["Three Data Models" for Microservices]({{< relref "three-data-models" >}})
* [Distributed Data Infrastructure (DDI)]({{< relref "ddi" >}})
{{< /links >}}
