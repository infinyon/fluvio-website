---
title: Smart Connectors Core Concepts 
menu: Core Concepts
weight: 10
---

There are 4 steps to the connector:

<img src="/images/connectors/smart-connectors-extra.svg"
     alt="Smart Connectors"
     style="justify: center; max-width: 600px" />

- **Protocol**: Parses data according to the wire format of the connected data platform.
- **Extract**: Extracts raw data from the protocol format and packages it neatly into data structures
  that may be used by subsequent stages or be produced directly to a topic.
- **Filter** (optional): A user-provided SmartModule that may determine whether a given record
  should be discarded before sending it over the network to Fluvio, saving bandwidth.
- **Shape** (optional): A user-provided SmartModule that may take the extracted data structures and
  transform them in to an application-specific format.

The **Protocol** and **Extract** stages are built directly into the
connector. They offer basic access to your data through the various protocols your data sources use.

In the **Extract** stage, your data is structured from whatever protocol it is sourced from.

Additionally, You can apply custom pre-processing or post-processing to data, before it
arrives to or while it streams from a Fluvio topic. The **Filter** and **Shape** stages are provided through SmartModules.

Powered by WebAssembly (also called wasm), SmartModules are pre-packaged or user-provided operations such as filters, maps, or aggregators that can be applied to records at various points in the streaming pipeline.
Supporting access to your data while it is in transit provides you the ability to clean, transform and enrich your data before it is stored in a topic, or it exits the Fluvio cluster.

Use Connectors locally with [`cdk`]({{<ref "cdk/overview.md">}}) or start a connector on [InfinyOn Cloud](https://infinyon.cloud), and let us manage the infrastruture