---
title: Fluvio - Distributed Data Infrastructure (DDI)
menu: Fluvio DDI
weight: 30
---

**Distributed Data Infrastructure (DDI)** is a **standards-based**, **language agnostic** software layer designed to enable Microservices to construct _data rich_ applications. Unlike other systems for distributed data, **Fluvio DDI** is a _dedicated infrastructure layer_ built outside of the service and managed by a control plane.

{{< image src="ddi-abstract.svg" alt="DDI Overview" justify="center" width="340" type="scaled-50">}}

The _technical benefits_ of the **DDI** are as follows:

* **Language Agnostic** - use your favorite language, Java, C++, TypeScript, Go, Scala, Ruby, etc.
* **Clean Code** - keeps distributed data concerns outside of the business logic.
* **Simplicity** - exposes Saga, CQRS, Commands, Projections through simple interfaces.
* **Consistency** - consolidates data communication through a well defined data interface.
* **Clarity** - uses expressive modeling language to define workflows, commands, transactions, reports, etc.
* **Traceability** - provides interfaces for monitor data exchanges as it traverses services.
* **Compliance** - enables service team to gain instant feedback on whether data adheres to corporate policy.

The _business benefits_ are shorter time to market, better code quality, faster troubleshooting,
and ability to apply frictionless corporate policy.


## DDI Stack

The Distributed Data Infrastructure is a stack that consists of four components:

* Distributed Control Plane
* Model Interpreter
* Data Flow Engine
* Data Streaming Engine

The diagram below illustrates a hierarchical view of the components:

{{< image src="ddi.svg" alt="Custom vs. DDI" justify="center" width="640" type="scaled-90">}}

Microservices applications are defined using EventQL models. When a new model is provisioned, the _Distributed Control Plane_ forwards the model to the _Model Interpreter_. The interpreter looks-up the Microservices and provisions an _Event Controller_ for each service. Each controller connects to the _Data Streaming Engine_ and subscribes to various channels based on the EventQL definition.

At runtime, a Microservices receives a command or an event. Commands are imperatives that ask services to perform an operation, whereas an events notify services of changes that occurred elsewhere. The services business logic performs the necessary updates and send the result to the _Event Controller_. The controller notifies the _Data Flow Engine_ to run the workflow defined in the EventQL definition. The model describes the sequence of operations and the events that are to be published to peer services.


### Event Definition

An Event is a Fact, a thing of importance that occurred in the past. When Microservices communicate through events they can express a series of facts about the domain which helps the system gain context and build hierarchical information tree for data exchange.

{{< image src="facts.svg" alt="Facts and Events" justify="center" width="600" type="scaled-75">}}

**Fluvio DDI** assumes that all inter-service communication is handled through **events**. 

Events ensure a data rich future proof distributed data infrastructure. For example, events can be payed back with different filtering algorithms and retrieve new insights from the same data.


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

