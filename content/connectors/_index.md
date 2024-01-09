---
title: Connectors
menu: Overview
section: Connectors
toc: true
---

Fluvio Connectors make the process of importing or exporting data simple.
You can import data with an `Inbound` connector and export data with an `Outbound` connector.

Inbound and outbound connectors fundamentally work in the same way. The only difference is the direction your data is streaming with respect to a Fluvio topic.

<img src="/images/connectors/smart-connectors-extra.svg"
     alt="Connectors"
     style="justify: center; max-width: 600px" />

* Learn more about the connector's 4 steps of transformation in [core concepts]({{<ref "core-concepts.md">}})

* For the development of your own connectors, continue on to the [Connector Development Kit docs]({{<ref "/connectors/cdk/overview" >}})
