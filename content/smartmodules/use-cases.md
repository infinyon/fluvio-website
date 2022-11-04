---
title: SmartModule Use Cases 
menu: Use cases
toc: false
---

SmartModules are small and portable making them suitable for a variety of edge to core use cases Today SmartModules can be used by consumers, producers, and connectors. Future releases will allow them to operate on data inside data streams.

<img src="/smartmodules/images/smartmodule-overview.svg" alt="SmartModule Overview" justify="center" height="480">

#### Edge (IoT)

SmartModules are ideal for edge (IoT) scenarios, where backhauling the data to the cloud is prohibitively expensive. SmartModules can filter, map or aggregate data before sending it to the cluster, significantly _reducing_ network traffic.

#### ETL to STL Migration

Traditional ETL pipelines are batch driven and often have a fragmented architecture that is difficult to manage and includes lots of software or microservices to make data usable. Fluvio users can now migrate to Stream, Transform and Load (STL) pipelines, where transformation is performed in real time before events are loaded into a database or data lake. Deploy SmartModules on the source or sink connectors to transform and structure data within your STL pipeline. ETL to STL migration reduces the cost and complexity of your architecture.
