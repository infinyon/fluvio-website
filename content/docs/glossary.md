---
title: Glossary
weight: 600
---

A glossary of terms used to describe Fluvio data streaming platform.

### Complex Event Processing (CEP)

_<a href="https://complexevents.com/stanford/cep/" target="_blank">Complex Event Processing in Distributed Systems. by David C. Luckham and Brian Frasca</a>_

Complex Event Processing, or CEP, consists of a set of concepts and techniques developed in the early 1990s for processing real-time events and extracting information from event streams as they arrive. The goal of complex event processing is to identify meaningful events in real-time situations and respond to them as quickly as possible.

CEP has since become an enabling technology in many systems that are used to take immediate action in response to incoming streams of events. Applications are now to be found in many sectors of business including stock market trading systems, mobile devices, internet operations, fraud detection, the transportation industry, and governmental intelligence gathering.


### Event Sourcing
_<a href="https://martinfowler.com/eaaDev/EventSourcing.html" target="_blank">Event Sourcing, by Martin Fowler</a>_

Event Sourcing</a> ensures that all changes to application state are stored as a sequence of events. Each change to the state of an application is captured in an event object, and that these event objects are themselves stored in the sequence they were applied for the same lifetime as the application state itself.

While events describe current state, they can also tell us how we got there.

