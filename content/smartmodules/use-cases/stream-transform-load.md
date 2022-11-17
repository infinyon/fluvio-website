---
title: Stream, Transform, Load (STL)
---

STL, which stands for stream, transform, is our integration process that combines data from multiple data sources, then apply customizable logic directly on your data and reshape it before saving to storage or sending it over the network. 

This is in contrast with the traditional ETL process, or extract, transform, load. ETL pipelines are batch driven can require managing  difficult to manage infrastructure that can be complex with many microservices to make data usable.

Migrating your ETL pipelines to STL pipelines brings logic to your data and gives you control of how you want to use it before sending it over your network. STL can support collaboration between teams sharing data by moving logic of assessing the quality of your data (validity, correctness, format, content) before you even receive it.

Moving from ETL to STL will reduce the cost and complexity of your architecture, because real time events can be transformed before it is loaded into a database or data lake.

Fluvio users transform data using [SmartModules]({{< ref "/smartmodules" >}}). SmartModules can be combined with [Connectors]({{< ref "/connectors" >}}) as part of your STL pipeline.