---
title: SmartModules
menu: Overview
toc: false
---

SmartModules are user-defined functions written in Rust and compiled to <a href="https://webassembly.org/" target="_blank">WebAssembly</a>, allowing Fluvio users direct control over their streaming data with programmable APIs. SmartModules are lightweight and portable and can be integrated at many points in a data streaming pipeline, enabling flexibility and cost savings. They are also reusable, fostering collaboration between SmartModule developers and data pipeline operators.

The following building blocks make building, testing, and deploying SmartModule easy:

* [SmartModule Development Kit (SMDK)]
* [SmartModule Hub]
* [SmartModule API Library]

#### SmartModule Developer Kit (SMDK)

SmartModules Development Kit (SMDK) is an independent executable downloadable via [`Fluvio CLI`] to help developers build and test SmartModules, and publish them to the SmartModule Hub. Checkout [`SMDK section`].

#### SmartModule Hub

SmartModule Hub, powered by [`InfinyOn Cloud`], is a real-time apps store, where developers publish SmartModules, and users download and integrate them into their data pipelines. Checkout [`Hub section`].

#### SmartModule API Library

SmartModule APIs are programmable data streaming functions exposed to WebAssembly. SmartModules allows developers to manipulate data in real-time inside the Fluvio cluster, without the need for external servicees such as Lambda or Functions. Checkout [`APIs section`].


## Use Cases

SmartModules are small and portable making the suitable for a variety of edge to core use cases Today SmartModules can be used by consumers, producers, and connectors. Future releases will allow them to operate on data inside data streams.

<img src="/smartmodules/images/smartmodule-overview.svg" alt="SmartModule Overview" justify="center" height="480">

#### Edge (IoT)

SmartModules are ideal for edge (IoT) scenarios, where backhaulling the data to the cloud is prohibitively expensive. SmartModuels can filter, map or aggregate data before sending it to the cluster, significantly _reducing_ network traffic.

#### ETL to STL Migration

Traditional ETL pipelines are batch driven and often have a fragmented architecture that is difficult to manage and includes lots of software or microservices to make data usable. Fluvio users can now migrate to Stream, Transform and Load (STL) pipelines, where transformation is performed in real time before events are loaded into a database or data lake. Deploy SmartModules on the source or sink connectors to transform and structure data within your STL pipeline. ETL to STL migration reduces the cost and complexity of your architecture.


[SmartModule Development Kit (SMDK)]: #smartmodule-developer-kit-smdk
[SmartModule Hub]: #smartmodule-hub
[SmartModule API Library]:  #smartmodule-api-library
[`Fluvio CLI`]: {{< ref "/cli/smartmodules/smdk" >}}
[`InfinyOn Cloud`]: https://infinyon.cloud/
[`SMDK section`]: {{< ref "/smartmodules/smdk/overview" >}}
[`Hub section`]: {{< ref "/smartmodules/hub/overview" >}}
[`APIs section`]: {{< ref "/smartmodules/apis/overview" >}}
