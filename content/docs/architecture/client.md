---
title: Client Library
toc: true
weight: 110
---

Fluvio client is responsible for all communication with a Fluvio cluster. The client uses a home grown communication protocol that was optimized for maximum performance, scalability, and low latency. Future versions will provide adaptors to standard protocols, such as: HTTP, WebSocket, gRPC, etc.


{{< image src="architecture/fluvio-client.svg" alt="External APIs" justify="center" width="500" type="scaled-75">}}


All communication between the clients and the servers is encrypted in <a href="https://en.wikipedia.org/wiki/Transport_Layer_Security" target="_blank">TLS</a> for maximum privacy and security.

## Native Language Bindings

Fluvio client library is written in Rust and it is designed to be natively embedded into other programming languages. 

### Node Binding

The first language binding implementation is for `Node.js`. Node binding allows web developer to add real-time data streaming to their applications with only a few lines of code.

* API Reference
    * [Node API Reference](/docs/node-api)

* Demo Apps
    * [Simple Chat](/docs/demo-apps/simple-chat-demo-node/) 
    * [CDC Producer](/docs/demo-apps/cdc-producer-node/) 

For additional details on Node binding implementation, checkout <a href="https://github.com/infinyon/node-bindgen" target="_blank">node-bindgen</a> library.


### Rust API

Rust API is native to the platform and no additional language binding is required. Checkout the following references to add real-time data streaming to your Rust application:

* API Reference
    * [Rust API Reference](/docs/rust-api)

* Demo Apps
    * [CDC Producer](/docs/demo-apps/cdc-producer-rust/) 
    * [DB Replication](/docs/demo-apps/db-replication-rust) 


### Fluvio CLI

Fluvio ClI an application written on top of the client library. The CLI is used manage, produce, and consume data to and from a Fluvio cluster using the terminal. 

For additional information, checkout:

* [Fluvio CLI](/docs/cli)

#### Future Work

Future version of Fluvio will provide additional programming language bindings, such as: 

* <a href="https://golang.org/" target="_blank">Go</a>
* <a href="https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/Introduction/Introduction.html" target="_blank">Objective-C</a>
* <a href="https://docs.oracle.com/javase/tutorial/" target="_blank">Java</a>
* <a href="https://docs.python.org/3/reference/" target="_blank">Python</a>
* <a href="https://www.cplusplus.com/" target="_blank">C/C++</a>.



## Client API

Fluvio client library has three core APIs: Admin, Producer and Consumer.

### Admin API

**Admin API** is the management interface for the Fluvio cluster. The API can perform the following operations:

* _create/delete/update_ objects such as: `create topic`.
* inspect _configuration_, such as: `list spus`.
* inspect _status_, such as: partitions status - `Online`, `Offline`, or `LeaderOffline`.

#### Configuration Objects

Configuration object models follow a similar paradigm. Each object has the following components:

* **Name** - unique identifier of the object
* **Spec** - the configuration specification (aka. desired state)
* **Status** - the actual provisioning status (aka. actual state)
* **Owner** - provides a parent/child relationship (for resource removal)

Fluvio operator configures the object _Spec_, whereas the Cluster updates the object _Status_. Conclusively,  _Status_ is a read-only element from the operator standpoint. 

Fluvio has the following configuration objects:

* **SPUs** - streaming processing unit (custom or managed)
* **SPGs** - groups of managed SPUs
* **topics** - data streaming configuration element
* **partitions** - provisioned data streaming element of a topic
    * partitions are children of topics

Each configuration object goes through its own lifecycle. Object status track the state as it progresses through various lifecycle stages. 

-> Some configuration objects such as **Partition** and **Managed** SPU are managed objects that are created as part of the parent's lifecycle and they `cannot be directly modified` by the operator.

For detailed schema definition and object life cycles, checkout the [Reference Guide](../references/#object-lifecycle).

#### Object Outputs

Each configuration object can outputted in different data formats, such as _json_, or _yaml_. Additional data formats are available and can be exposed if required.

Filters can be applied when fetching a configuration object, such as: `list spus name`, `list partition name`, etc.


### Producer API

**Producer API** is responsible for sending records to data streams. A data record is a `key/value` object, where `key` is an optional field. `Key` and `value` fields can be of arbitrary format.

For example, a producer with a `key` mapped to countries would use the following API:

```rust
let timber_resources: HashMap<&str, i32> = 
    [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
    .into_iter().collect();   

producer.send(timber_resources).await;
```

#### Producer Behavior

Producers can send records _one at a time_ or _batched_. Producer API is multi-threaded, which enables applications to implement multiple instances of the API to any number of data streams in parallel.


### Consumer API

**Consumer API** is responsible for receiving records from data streams. Records can be retrieved _one at a time_ or in _continuously_ from any position in the data stream.

For example, a consumer reading `key/value` records (_one at a time_) from offset 100, use the following API:

```rust
let records = consumer.fetch(100).await;  

for record in records { 
    for (k, v) in record { 
        println!("{}, {}", k, v);
    } 
}
```

Records are retrieved in _binary format_ and it is up to the Application developer to provide a conversion into their custom type.


#### Consumer Behavior

Consumers are also multi-threaded which allows them to read records from multiple data streams simultaneously. Each connection can specify different retrieval properties:

* **Delivery** - retrieve records based on their replication status
    * COMMITTED: fetch records only after they have been replicated and committed by all `Live Replicas`.
    * IMMEDIATE: fetch records as soon as they are received by the `Replica Leader`.
* **Max Bytes** - batch records up to a maximum number of bytes
    * Default buffer size is 1Mb.


### Fault Tolerance

The client is designed to survive SPU failures.  Data streams are replicated across multiple SPUs. When a connection is established, the client looks-up the SPU that hosts the replica leader. This SPU will services all replica producers and consumers. If the SPU goes offline, one of the followers becomes the leader and the new SPU services the consumers/producers. The client **automatically switches over** to the new SPU.


## Client Workflow

Client library provides provides profiles that hides the complexity of the connection configuration (target cluster, TLS configuration, etc.)


The client library has language bindings and a CLI. 


Client and full multi-threaded that can produce or consume concurrently to and from different topics.


Workflow
* use profile to connect to cluster
* use Admin API to retrieve objects
* use consumer or producer to exchange data stream



#### Related Topics
-------------------
* [SC Architecture](../SC)
* [SPU Architecture](../spu)
* [Replication](../replication)
* [Kubernetes Integration](../k8-integration)
* [Deployment Models](../deployments)