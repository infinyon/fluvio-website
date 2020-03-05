---
title: Introduction to Fluvio
---
As monoliths are migrated to microservices, the choice of inter-service __communication__ and the __type of data__ they exchange are important decisions. The choices made impact the glue logic and the amount of intelligence each service must implement to participate in the quorum. The larger the application the broader the impact.

#### Communication
The microservices community promotes the philosophy of __["smart endpoints and dumb pipes"] (https://martinfowler.com/articles/microservices.html#SmartEndpointsAndDumbPipes)__.  A pipe refers to a time-tested asynchronous protocol, where the client doesn't wait for a response is preferred over a synchronous protocol. This approach allows the clients to stay independent preserving "loose coupling" regardless of state of the server.

#### Type of Data
When microservices communicate, they can exchange __states__ or __events__. A __state__ is the particular condition that something is in at a specific time; for example "Wifi is On". An __event__ is something that happens for only a moment in time, for example "Wifi has turned On". When states are exchanged, all previous information is lost. When __events__ are exchanged through the intermediary of an __event store__, the store becomes the single source of truth for the system. Events can be played back to derive all previous states of the system. 
  
We believe an __asynchronous event driven architecture with built-in event store__ is the right architectural pattern for building scalable distributed applications.


## What is Fluvio?

Fluvio is a cloud native platform that reduces the complexity associated with building and orchestrating event-based Microservices. 

Some of the unique aspects of the Fluvio platform are:

* Declarative Management - An intention-based approach to data management.
* Event Store - Built-in event store with built-in replication to ensure availability.
* Cloud Native by Design - Built-in Kubernetes support.
* Real-time Architecture - Built for low latency and high throughput environments.
* Flexible Deployments - Fluvio Controller can manage Cloud and on-Premise services simultaneously.
* Simple to Operate - Powerful Command Line Interface.
* Compiled to Machine Code - Small, Secure, and Fast. No garbage collection!


{{< links "Next Steps" >}}
* [Getting Started](...)
{{< /links >}}
