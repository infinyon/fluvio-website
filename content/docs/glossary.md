---
title: Glossary of Domain Driven Design Terminology
menu: Glossary of DDD Terms
toc: true
weight: 600
---

A list of technology used to describe various aspects of the Fluvio data streaming platform.

### DDD - Domain Driven Design

_<a href="https://domainlanguage.com/wp-content/uploads/2016/05/DDD_Reference_2015-03.pdf" target="_blank">Domain-Driven Design Reference, by Eric Evans</a>_

A _domain_ is the sphere of knowledge and activity around which the application logic revolves. _Domain-Driven Design (DDD)_ is an approach to developing software for complex needs by deeply connecting the implementation to an evolving model of the core business concepts.

Evans’ Domain-Driven Design further defines a few common terms that are useful when describing and discussing DDD practices:

* **Context**: The setting in which a word or statement appears that determines its meaning. Statements about a model can only be understood in a context.

* **Model**: A system of abstractions that describes selected aspects of a domain and can be used to solve problems related to that domain.

* **Ubiquitous Language**: A language structured around the domain model and used by all team members to connect all the activities of the team with the software.

* **Bounded Context**: A description of a boundary (typically a subsystem, or the work of a specific team) within which a particular model is defined and applicable.


### Domain Events

_<a href="http://codebetter.com/gregyoung/2010/04/11/what-is-a-domain-event/" target="_blank">What is a Domain Event? by Greg Young</a>_

An _event_ is something that has happened in the past - a **fact** or **state transition** that can be shared with other parts of the system.

All events should be represented as verbs in the past tense such as _CustomerRelocated_, _CargoShipped_, or _InventoryLossageRecorded_.


### CQRS - Command Query Responsibility Segregation

CQRS means "Command-query responsibility segregation". We segregate the responsibility between commands (write requests) and queries (read requests). The write requests and the read requests are handled by different objects.



### Aggregates

_<a href="https://martinfowler.com/bliki/DDD_Aggregate.html" target="_blank">DDD Aggregate, by Martin Fowler</a>_

An aggregate is a group of domain objects that can be treated as a single unit. 

Use	the	same aggregate boundaries to govern transactions and distribution.

Transactions should not cross aggregate boundaries. Within an aggregate boundary, apply consistency rules synchronously. Across	 boundaries, handle updates asynchronously.


### CEP - Complex Event Processing

_<a href="https://complexevents.com/stanford/cep/" target="_blank">Complex Event Processing in Distributed Systems. by David C. Luckham and Brian Frasca</a>_

Complex Event Processing, or CEP, consists of a set of concepts and techniques developed in the early 1990s for processing real-time events and extracting information from event streams as they arrive. The goal of complex event processing is to identify meaningful events in real-time situations and respond to them as quickly as possible.

CEP has since become an enabling technology in many systems that are used to take immediate action in response to incoming streams of events. Applications are now to be found in many sectors of business including stock market trading systems, mobile devices, internet operations, fraud detection, the transportation industry, and governmental intelligence gathering.


### Data - Then and Now

_<a href="http://cidrdb.org/cidr2005/papers/P12.pdf" target="_blank">Data on the Outside versus Data on the Inside by Pat Helland</a>_

...

### Data Driven

Your are data-driven when the progress in an activity is compelled by data. Businesses asystematically and methodically use data instead of motions, external pressure, or instinct to power decisions.



### Event Streams

...


### Projections

You can change a projection, you can only create a new projects.


### Rolling Snapshots

Tke a snapshot of all the events up to a certain point.

Capture the state up from previous snapshot up to this current point.

A snapshot is a memorization of a previous events. 


### Event Sourcing

_<a href="https://martinfowler.com/eaaDev/EventSourcing.html" target="_blank">Event Sourcing, by Martin Fowler</a>_

Event Sourcing ensures that all changes to application state are stored as a sequence of events. Each change to the state of an application is captured in an event object, and that these event objects are themselves stored in the sequence they were applied for the same lifetime as the application state itself.

While events describe current state, they can also tell us _how we got there_.

Each state transition should in your system is **a fact**. Event sourcing ensures all facts in your system are stores as a sequence of immutable events.

### Event Storming

...

### Microservices

...


### Other References

* _<a href="https://www.youtube.com/watch?v=JHGkaShoyNs" target="_blank">CQRS and Event Sourcing, by Greg Young</a>_
