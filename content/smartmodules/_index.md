---
title: SmartModules
menu: Overview
toc: true
---

SmartModules are user-defined functions written in Rust and compiled to <a href="https://webassembly.org/" target="_blank">WebAssembly</a>, allowing Fluvio users direct control over their streaming data with programmable APIs. SmartModules are lightweight and portable and can be integrated at many points in a data streaming pipeline, enabling flexibility and cost savings. They are also reusable, fostering collaboration between SmartModule developers and data pipeline operators.

The following building blocks make building, testing, and deploying SmartModule easy:

* [SmartModule Development Kit (SMDK)]
* [SmartModule Hub]
* [SmartModule API Library]

#### SmartModule Developer Kit (SMDK)

SmartModules Development Kit (SMDK) is an independent command line downloadable via [`Fluvio CLI`] to help SmartModule developers build and test SmartModules, and publish them to the SmartModule Hub. 

#### SmartModule Hub

SmartModule Hub is the store for real-time data pipelines. Developers publish SmartModules, and user operators download and integrate them into their pipelines. The Hub, powered by [`InfinyOn Cloud`], groups SmartModules by their owners, where each owner decides if their SmartModule should be private or publicly accessible. 

#### SmartModule API Library

The following diagram shows common components which may be configured with SmartModules
performing inline computation.

[SmartModule Development Kit (SMDK)]: #smartmodule-developer-kit-smdk
[SmartModule Hub]: #smartmodule-hub
[SmartModule API Library]:  #smartmodule-api-library
[`Fluvio CLI`]: {{< ref "/cli/smartmodules/smdk" >}}
[`InfinyOn Cloud`]: https://infinyon.cloud/

## Use Cases

SmartModules are small and portable making the suitable for a variety of edge to core use cases Today SmartModules can be used by consumers, producers, and connectors. Future releases will allow them to operate on data inside data streams.

<img src="/smartmodules/images/smartmodule-overview.svg" alt="SmartModule Overview" justify="center" height="480">

#### Edge (IoT)

SmartModules are ideal for edge (IoT) scenarios, where backhaulling the data to the cloud is prohibitively expensive. SmartModuels can filter, map or aggregate data before sending it to the cluster, significantly _reducing_ network traffic.

#### ETL...

