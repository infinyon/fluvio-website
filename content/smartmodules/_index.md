---
title: SmartModules
menu: Overview
toc: false
---

SmartModules are programmable data streaming functions exposed to WebAssembly, allowing developers to manipulate data in real-time inside the Fluvio cluster without needing external services such as Lambda or Functions. SmartModules can be published to [SmartModule Hub] and downloaded to multiple clusters. Once downloaded, SmartModules can be chained together to build powerful data transformation pipelines.

Examples on how to use SmartModules in a pipeline:
* [Build HTTP to SQL Pipeline]
* [Build MQTT to SQL Pipeline]

Build, test, and publish your own SmartModules:

* [SmartModule Development Kit (SMDK)]
* [SmartModule Hub]

APIs to build your own SmartModules:
* [Transform]
* [Analytics]

InfinyOn Certified SmartModules published in the [SmartModule Hub]:
* [Jolt]
* [Json-Sql]


[SmartModule Development Kit (SMDK)]: {{< ref "/smartmodules/smdk/overview" >}}
[SmartModule Hub]: {{< ref "/smartmodules/hub/overview" >}}
[Transform]:  {{< ref "/smartmodules/transform/overview" >}}
[Analytics]:  {{< ref "/smartmodules/analytics/overview" >}}
[Jolt]:  {{< ref "/smartmodules/certified/jolt" >}}
[Json-Sql]:  {{< ref "/smartmodules/certified/json-sql" >}}
[Build HTTP to SQL Pipeline]: {{<ref "/docs/tutorials/data-pipeline.md" >}}
[Build MQTT to SQL Pipeline]: {{<ref "/docs/tutorials/mqtt-to-sql.md" >}}
