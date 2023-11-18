---
title: Producers
weight: 10
---

Producers are applications that "produce" streaming data.
These applications may be monitoring the status of a system, collecting sensor
data, watching traffic to a website, or otherwise observing events in some way.
Producers may be general-purpose tools such as [the Fluvio CLI], or they may be
special-purpose programs built to meet a specific need, in which case the
producer program would leverage [one of the programmatic APIs].

[the Fluvio CLI]: {{< ref "/cli" >}}
[one of the programmatic APIs]: {{< ref "/api" >}}


## Start Producing

In order to get started with producing streaming data, you'll need to:

- [Have a Fluvio cluster up and running]({{< ref "/docs/get-started/linux" >}}),
- [Create a Topic to produce data to]({{< ref "/cli/cluster/topic#fluvio-topic-create" >}}), then
- Choose one of the following producer interfaces to use:
  - [Fluvio CLI]({{< ref "/cli/client/consume" >}})
  - [Rust]({{< ref "/api/official/rust/installation" >}})
  - [Node]({{< ref "/api/official/node/installation" >}})
  - [Python]({{< ref "/api/official/python/installation" >}})
  - [Java]({{< ref "/api/official/java/installation" >}})
