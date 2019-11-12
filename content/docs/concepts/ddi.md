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

The Distributed Data Infrastructure is a stack consists of four components:
* Distributed Control Plane
* Model Interpreter
* Data Flow Engine
* Data Streaming Engine

At high level, the _Distributed Control Plane_ sends an EventQL application definition to the _Model Interpreter_. The interpreter provisions the event controllers for in all Microservices of the application.

{{< image src="ddi.svg" alt="Custom vs. DDI" justify="center" width="600" type="scaled-90">}}

