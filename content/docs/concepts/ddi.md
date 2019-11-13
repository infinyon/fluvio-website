---
title: Fluvio - Distributed Data Infrastructure (DDI)
menu: Fluvio DDI
weight: 30
---

**Distributed Data Infrastructure (DDI)** is a **standards-based**, **language agnostic** software layer designed to enable microservices to construct _data rich_ applications. Unlike other systems for distributed data, **Fluvio DDI** is a _dedicated infrastructure layer_ built outside of the service and managed by its own control plane.

{{< image src="ddi-abstract.svg" alt="DDI Overview" justify="center" width="340" type="scaled-50">}}

_Technical benefits_ for using **Fluvio DDI**:

* **Language Agnostic** - use your favorite language, Java, C++, TypeScript, Go, Scala, Ruby, etc.
* **Clean Code** - keeps distributed data implementation outside of the business logic.
* **Simplicity** - encapsulates SAGAs, CQRS, Commands, Projections behind API interfaces.
* **Consistency** - consolidates distributed data handling a uniform data interface.
* **Clarity** - uses expressive modeling language to define workflows, commands, transactions, reports, etc.
* **Traceability** - exposes monitoring interface for tracing data end-to-end.
* **Compliance** - has a control plane to ensure data exchange adheres to corporate policy.

_Business benefits_ for using **Fluvio DDI**:
 
* shorter time to market
* better code quality
* faster troubleshooting,
* ability to apply frictionless corporate policy

The **Distributed Data Infrastructure** is also multi-tenant aware. Multiple applications can run side-by-side without interfering with each other.

## DDI Stack

The **Distributed Data Infrastructure** stack has four core components:

* Distributed Control Plane
* Model Interpreter
* Data Flow Engine
* Event Streaming Engine

An _EventQL Model_ defines the hierarchy and interaction of the microservices in an application. When a new application is provisioned, the _Distributed Control Plane_ forwards the EventQL spec to the _Model Interpreter_. The interpreter provisions an _Event Controller_ for each microservice. The controllers connect to the _Event Streaming Engine_ and subscribe to the channels defined in the EventQL specification.

{{< image src="ddi.svg" alt="Custom vs. DDI" justify="center" width="640" type="scaled-90">}}

At runtime, microservices receive commands or events. _Commands_ are imperatives that ask services to perform an operation, whereas _events_ notify services of changes that occurred elsewhere. Microservices receive the input, perform the business logic, and send the output to the _Event Controller_. The controller notifies the _Data Flow Engine_ to run the workflow in the EventQL spec and publishes the output to corresponding channels. 

A command or event is finished processing when all services in the EventQL workflow completed their operations.

### Event Definition

_Events_ are facts, things of importance that occurred in the past. _Events_ are the source of truth expressed as immutable actions. When microservices publish events they communicate the behavior of their domain. This information helps the **DDI** gain business context, model workflows, and build hierarchical information tree for all data exchanges.

{{< image src="facts.svg" alt="Facts and Events" justify="center" width="600" type="scaled-75">}}

Microservices that exchange _events_ rather than _state_ share the full history all things of importance and ensure their data is future proof. For example, events can be played back in the future with new filtering criteria to retrieve additional insights.

**Fluvio DDI** assumes that all inter-service communication is handled through **events**. 


### EventQL Definition

EventQL is an open source query language (QL) that describes event oriented distributed data flows. It expresses data domains, events types, operations, and inter-service relationships. At core, EventQL is a modeling language that converts event-centric service interactions into code.

{{< code >}}
Aggregate Order {

    Event OrderSubmitted {...}
    Event OrderCreated {...}
    
    State OrderState {...}
    
    Command UpdateEmailAddress {...}

    ...
}   
{{< /code >}}

EventQL modeling can be used as a design language for quick prototyping. Write interaction model and the compiler generates language bindings for Java, Go, Rust, C#, TypeScript and Go.

It is a strong typed language by design to ensure no undefined behavior and it has built-in versioning for CI/CD pipeline and GitOps operation model.

