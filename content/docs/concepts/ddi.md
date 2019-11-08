---
title: Distributed Data Infrastructure (DDI)
menu: DDI
weight: 30
---

**Distributed Data Infrastructure (DDI)** is a **standards-based**, **language agnostic** software that glues services to distributed data. It allows services to share data without coding the data layer itself.

**Distributed Data Infrastructure (DDI)** is a **infrastructure layer** that move distributed data concerns out of Microservice.

* It lets developers to focus on business logic not distributed data problems.
* It provides saga based transactions.
* It uses Event as first class citizen.  All operations and tooling can operate that level.
* It is implemented as distribute architecture to offer highly resilient and scalable services.
* It is language agonistic.  You can use any language (Java, C#, C++, Go, Scala, etc).
* Centralized governess thru control plane.
* It use Domain Specification Language to describe event syntax and lifecycle.



The benefits are shorter time to market, better code quality, and the ability to add out of band data management features such as role based access, auditing, monitoring, and troubleshooting.

{{< image src="ddi.svg" alt="Custom vs. DDI" justify="center" width="700" type="scaled-90">}}

The **Distribute Data Infrastructure** is:

* **Language agnostic** - use your favorite language, Java, C++, TypeScript, Go, Scala, Ruby, etc.
* **Versatile** - covers many distributed data data use cases from CQRS to Saga out of the box.
* **Painless** - you don't have to be a distributed data wizard to operate it.
* **Expressive** - just define the workflow, the chain of services and the data they exchange, no coding required.
* **Traceable** - track, debug, and monitor any data as it traverses services.
* **Secure** - apply corporate data policy without rebuilding or deploying application code.
* **Compliant** - perform data audits without impacting the application.

The DDI **democratizes** the use of data across the organization.