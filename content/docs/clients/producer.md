---
title: Producers
weight: 10
---

Producers are applications that "produce" streaming data.
These applications may be monitoring the status of a system, collecting sensor
data, watching traffic to a website, or otherwise observing events in some way.
Producers may be general-purpose tools such as [the Fluvio CLI], or they may be
special-purpose programs built to meet a specific need, in which case the
producer program would leverage [one of the programmatic APIs]. Let's talk about
the core concepts that producers deal with in order to better understand how
they work.

[the Fluvio CLI]: {{< ref "/cli" >}}
[one of the programmatic APIs]: {{< ref "/api" >}}


## Start Producing

In order to get started with producing streaming data, you'll need to:

- [Have a Fluvio cluster up and running]({{< ref "/docs/get-started/linux" >}}),
- [Create a Topic to produce data to]({{< ref "/cli/commands/topic#fluvio-topic-create" >}}), then
- Choose one of the following producer interfaces to use:
  - [Fluvio CLI]({{< ref "/cli/commands/consume" >}})
  - [Rust]({{< ref "/api/official/rust/installation" >}})
  - [Node]({{< ref "/api/official/node/installation" >}})
  - [Python]({{< ref "/api/official/python/installation" >}})
  - [Java]({{< ref "/api/official/java/installation" >}})