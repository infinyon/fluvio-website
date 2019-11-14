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

Microservices that exchange _events_ rather than _states_ share the full history all things of importance and ensure their data is future proof. For example, events can be played back with new filtering criteria anytime in the future.

**Fluvio DDI** assumes that all inter-service communication is handled through **events**. 


### EventQL Definition

_EventQL_ is an open source query language (QL) that describes distributed data flows. The language has a rich set of directives to define domain properties, events types, operations, and inter-service relationships. **Fluvio DDI** uses _EventQL_ models to create aggregates, setup data flows, provision projections and run transactions.

At core, EventQL is a modeling language that converts event-centric service interactions into code. It simplifies prototyping and accelerates development. Initially, EventQl models generate language bindings for Rust and it can be extended to other programming language (Java, Go, Python, C#, etc.).

#### EventQL Language Definition

EventQL language uses the following keywords to define distributed data flows: _types, events, states, aggregates, commands, transactions_, and _reactors_.


##### Types

EventQL primitive types are:

* int
* bool
* enum
* UTF8 string
* id (all events have ids)

Null types are not supported. 


##### Events

_Event_ are described in detail [above](http://localhost:1313/docs/concepts/ddi/#event-definition). _Events_ have two built-in fields:

* event ID (UUID)
* time

Example of an event definition:

{{< code >}}
event CustomerEmailChanged {
   user_id: ID,
   email: String
}
{{< /code >}}

Events are grouped inside _Aggregates_.

##### States

_States_ are the latest known condition of things. States can be derived from events or computed from a combination of indicators. Complex business logic is usually divided by states.

For example, an _AccountBalance_ state can be computed from _AccountDeposited_ and _AccountWithdrawn_ events:

{{< code >}}
state AccountBalance {
    account_id: ID,
    total: u16
}

event AccountDeposited {
    account_id: ID,
    amount: U16
 }
 
event AccountWithdrawn {
    account_id: ID,
    amount: U16
 }
{{< /code >}}

A discrete state can be defined as follows:

{{< code >}}
state CustomerOrder {
    Status(OrderStatus)
}

enum OrderStatus {
   OrderCreated,
   OrderShipped,
   OrderClosed
}
{{< /code >}}

States are derived from events inside _Aggregates_.


##### Aggregates

_Aggregates_ are business logic wrappers that describe _events_ and _service behavior_. Each service has one aggregate. Aggregates manage service state, transactions, and command handlers. They are responsible for enforcing business constrains.

Example of an aggregate definition:

{{< code >}}
aggregate Order {
    event OrderSubmitted ...
    event OrderCreated ...
    state OrderState ...
    command UpdateEmailAddress ...
}   
{{< /code >}}

Aggregates are called by the _Event Controller_.


##### Commands

_Commands_ express intent, an operations to be executed in the future. Commands may be executed or rejected by the _Command Handler_. A command may be use in combination with _state_ and can generate one or more _events_.

Example of a command definition:

{{< code >}}
command UpdateEmailAddress {
    user_id: ID
    email_address: String
}
{{< /code >}}

Commands are grouped inside _Aggregates_.


##### Transactions (SAGAs)

[SAGAs](http://www.cs.cornell.edu/andru/cs711/2002fa/reading/sagas.pdf) are the recommended mechanism for transaction management in a distributed system. Sagas keyword describes series of command/events that must be processed as an atomic operation. All components must succeed, or fail together. Therefore, every command must have compensating command/events combination.

Transactions are grouped inside _Aggregates_.

##### Reactors

_Reactors_ define reactive behavior triggered by other service events. Unlike commands that require a _Command Handler_, reactors, don't have _Reactor Handler_.


#### EventQL and Git
EventQL models are textual representation of distributed data flows for microservice applications. Models may be changed, versioned and reapplied to running Apps. They may be stored in git and applied by CI/CD pipelines in GitOps operation models.

