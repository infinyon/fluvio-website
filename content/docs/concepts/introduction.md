---
title: Overview
weight: 10
---

For the past few years companies made significant investments on building and deploying Microservices to accelerate product development and shorten the time to market. That fostered innovation around container technology, continuous deployment, GitOps, DevOps, Cloud, and Serverless. Challenges around deployment, networking, security, and monitoring are gradually addressed. Implementing distributed data across services remains a laborious task. 

{{< image src="monolithic-2-microservices.svg" alt="Monolith to Microservices" justify="center" width="500" type="scaled-90">}}

As organizations gain confidence in microservices, they transition from building small Apps with few services to decomposing legacy monolithic Apps. These legacy apps have many cross-cutting concerns and often yield dozens of services during decomposition. In monolithic App, services reside on the same server and communicate with each other through reliable mechanisms such as shared memory, pipes, sockets or RPCs. In distributed environments, the same calls become network requests.
 
As the number of services that share data over the network increase, they run into distributed data challenges, such as:

* data boundary definition
* data discovery and schema definition
* inter-service data consistency
* multi-service transactions
* data joins and cross-services queries

Today, these challenges are addressed through custom implementations. Each service team designs and embed their own flavor of distributed data layer into the service logic. The outcome is a diverse set of solutions across teams, services, and programming languages that creates unnecessary **accidental complexity**.

{{<idea>}}
We believe distributed data management should have a _dedicated infrastructure_ decoupled from the business logic that iss **shared by all services**. 
{{</idea>}}

In the next section, we describe the **three stages** most organizations use as they convert **monolithic** Apps into **microservices**. 


{{< links "Related Topics" >}}
* ["The Three Stages" from Monolith to Microservices]({{< relref "three-stages" >}})
* [Distributed Data Infrastructure (DDI)]({{< relref "ddi" >}})
{{< /links >}}
