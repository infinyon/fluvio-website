---
title: SmartModules
menu: Overview
toc: false
---

SmartModules are user-defined functions written in Rust and compiled to <a href="https://webassembly.org/" target="_blank">WebAssembly</a>, allowing Fluvio users direct control over their streaming data with programmable APIs. SmartModules are lightweight and portable and can be integrated at many points in a data streaming pipeline, enabling flexibility and cost savings. They are also reusable, fostering collaboration between SmartModule developers and data pipeline operators.

The following building blocks make building, testing, and deploying SmartModule easy:

* [SmartModules]
* [SmartModule Hub]
* [SmartModule Development Kit (SMDK)]

#### SmartModule Development Kit (SMDK)

SmartModules Development Kit (SMDK) is an independent executable downloadable via [`Fluvio CLI`] to help developers build and test SmartModules, and publish them to the SmartModule Hub. Checkout [`SMDK section`].

#### SmartModule Hub

SmartModule Hub, powered by [`InfinyOn Cloud`], is a real-time apps store, where developers publish SmartModules, and users download and integrate them into their data pipelines. Checkout [`Hub section`].

#### SmartModules

SmartModules are programmable data streaming functions exposed to WebAssembly. SmartModules allows developers to manipulate data in real-time inside the Fluvio cluster, without the need for external services such as Lambda or Functions. Checkout [`SmartModule types`].


[SmartModule Development Kit (SMDK)]: {{< ref "/smartmodules/smdk/overview" >}}
[SmartModule Hub]: {{< ref "/smartmodules/hub/overview" >}}
[SmartModules]:  {{< ref "/smartmodules/transform/overview" >}}
[`Fluvio CLI`]: {{< ref "/cli/smartmodules/smdk" >}}
[`InfinyOn Cloud`]: https://infinyon.cloud/
[`SMDK section`]: {{< ref "/smartmodules/smdk/overview" >}}
[`Hub section`]: {{< ref "/smartmodules/hub/overview" >}}
[`SmartModule types`]: {{< ref "/smartmodules/transform/overview" >}}