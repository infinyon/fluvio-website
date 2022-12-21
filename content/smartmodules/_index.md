---
title: SmartModules
menu: Overview
toc: false
---

SmartModules are programmable data streaming functions exposed to WebAssembly, allowing developers to manipulate data in real-time inside the Fluvio cluster without needing external services such as Lambda or Functions. SmartModules can be published to [SmartModule Hub] and downloaded to multiple clusters. Once downloaded, SmartModules can be chained together to build powerful data transformation pipelines.

<img src="/smartmodules/images/hub.jpg" alt="SmartModule Hub" justify="center" style="width: 85%; min-width: 330px;" >

## For Users

#### Certified SmartModules Examples

InfinyOn Certified SmartModules published in the [SmartModule Hub]:
<<<<<<< HEAD
* [Jolt] - JSON to JSON tranformation
* [Json-Sql] - JSON to SQL tranformation
=======
* [Jolt] - JSON to JSON transformation
* [Json-Sql] - JSON to SQL transformation
>>>>>>> master
* RegEx - Regex filtering

#### Pipelines with SmartModules Examples

Examples on how to use SmartModules in a pipeline:
* [Build HTTP to SQL Pipeline]
* [Build MQTT to SQL Pipeline]



## For Developers

SmartModule Development Kit `smdk` is a utility to generate, build, test, and publish SmartModules using Rust programming language. Install `smdk` through Fluvio CLI, and build your first SmartModule:

%copy first-line%
```shell
$ fluvio install smdk
```

%copy first-line%
```shell
$ smdk -h
SmartModule Development Kit utility

Commands:
  build       Builds SmartModule into WASM
  generate    Generates a new SmartModule Project
  test        Test SmartModule
  load        Load SmartModule into Fluvio cluster
  publish     Publish SmartModule to Hub
  set-public  Set package as public
  help        Print this message or the help of the given subcommand(s)
```

#### Build and Test
* [SmartModule Development Kit (SMDK)]

#### Publish to SmartModule Hub
* [SmartModule Hub]

#### Data Streaming APIs available in SmartModules
* [Filter]
* [Map]
* [FilterMap]
* [ArrayMap]
* [Aggregate]


[SmartModule Development Kit (SMDK)]: {{< ref "/smartmodules/smdk/overview" >}}
[SmartModule Hub]: {{< ref "/smartmodules/hub/overview" >}}
[Filter]:  {{< ref "/smartmodules/transform/filter" >}}
[Map]:  {{< ref "/smartmodules/transform/map" >}}
[FilterMap]:  {{< ref "/smartmodules/transform/filter-map" >}}
[ArrayMap]:  {{< ref "/smartmodules/transform/array-map" >}}
[Aggregate]:  {{< ref "/smartmodules/analytics/aggregate" >}}
[Jolt]:  {{< ref "/smartmodules/certified/jolt" >}}
[Json-Sql]:  {{< ref "/smartmodules/certified/json-sql" >}}
[Build HTTP to SQL Pipeline]: {{<ref "/docs/tutorials/data-pipeline.md" >}}
[Build MQTT to SQL Pipeline]: {{<ref "/docs/tutorials/mqtt-to-sql.md" >}}