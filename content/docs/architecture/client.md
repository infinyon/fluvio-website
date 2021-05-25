---
title: Client Library
weight: 90
---

A Fluvio client communicates with a Fluvio cluster to manage streams and to emit or receive events. The client uses a purpose-built communication protocol that is optimized for maximum performance, scalability, and low latency. Websocket is currently in the works, and future versions will provide adaptors to other protocols, such as: HTTP, gRPC, protobuf and more.

<img src="../images/fluvio-client.svg"
     alt="Fluvio Client"
     style="justify: center; max-width: 500px" />

All communication between the clients and the servers is encrypted in <a href="https://en.wikipedia.org/wiki/Transport_Layer_Security" target="_blank">TLS</a> for maximum privacy and security.

## Native Language Bindings

The Fluvio client library is written in Rust and can be natively embedded into other programming languages. 

### Node Binding

The first language binding implementation is for `Node.js`. The Node bindings allow web developers to add real-time data streaming to applications in a matter of minutes. No servers to configure, no databases to maintain, just connect your App to [Fluvio Cloud], and you have the <ins>first building block</ins> on the path to a **real-time distributed App**.

[Fluvio Cloud]: {{< ref "/docs/get-started/cloud" >}}

* API Reference
    * <a href="https://infinyon.github.io/fluvio-client-node/" target="_blank">Node API Reference</a>

* Tutorials
    * [Hello, World!](https://www.infinyon.com/tutorials/node/hello-world/) 

For additional details on Node binding implementation, checkout <a href="https://github.com/infinyon/node-bindgen" target="_blank">node-bindgen</a> library.


### Rust API

The Rust API is native to the platform and does not require any language binding. Check out the following references to add real-time data streaming to your Rust application:

* API Reference
    * <a href="https://docs.rs/fluvio/" target="_blank">Rust API Reference</a>

* Demo Apps
    * [Rust Tutorials](https://www.infinyon.com/tutorials/rust/hello-world/) 


### Fluvio CLI

The Fluvio ClI an application written on top of the client library. The CLI can manage a Fluvio cluster as well as produce and consume data using the terminal. 

For additional information, checkout:

* [Fluvio CLI]({{< ref "/cli" >}})

#### Future Work

Future versions of Fluvio will provide additional programming language bindings, such as: 

* <a href="https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/Introduction/Introduction.html" target="_blank">Objective-C</a>
* <a href="https://www.cplusplus.com/" target="_blank">C/C++</a>


## Client API

Fluvio client library has three core APIs: Producer, Consumer, and Admin.


### Producer API

The **Producer API** is responsible for sending records to data streams. A data record is a `key/value` object, where `key` is an optional field. `Key` and `value` fields can be of arbitrary format.

For example, a producer with a `key` mapped to countries would use the following API:

```rust
let timber_resources: HashMap<&str, i32> = 
    [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
    .into_iter().collect();   

producer.send(timber_resources).await;
```

#### Producer Behavior

Producers can send records _one at a time_ or in _batches_. The producer API is multi-threaded, which enables applications to stream data in parallel.


### Consumer API

The **Consumer API** is responsible for receiving records from data streams. Records can be retrieved _one at a time_ or _continuously_ from any position in the data stream.

For example, a consumer reading `key/value` records (_one at a time_) from offset 100, would use the following API:

```rust
let records = consumer.fetch(100).await;  

for record in records { 
    for (k, v) in record { 
        println!("{}, {}", k, v);
    } 
}
```

Records are transmitted in _binary format_ and it is up to the Application developer to provide a conversion into their custom type.


### Admin API

The **Admin API** is the management interface for the Fluvio cluster. The API can perform the following operations:

* _create/delete/update_ objects such as: `create topic`.
* inspect _configuration_, such as: `list spus`.
* inspect _status_, such as: partitions status - `Online`, `Offline`, or `LeaderOffline`.


#### Configuration Objects

Configuration object models follow a similar paradigm. Each object has the following components:

* **Name** - unique identifier of the object
* **Spec** - the configuration specification (aka. desired state)
* **Status** - the actual provisioning status (aka. actual state)
* **Owner** - provides a parent/child relationship (for resource removal)

A Fluvio administrator configures the object _Spec_, and the cluster updates the object _Status_ to match. The _Status_ is a read-only element from the administrator's perspective. 

Fluvio has the following configuration objects:

* **SPUs** - streaming processing unit (custom or managed)
* **SPGs** - groups of managed SPUs
* **topics** - data streaming configuration element
* **partitions** - provisioned data streaming element of a topic
    * partitions are children of topics

Each configuration object goes through its own lifecycle. Object status track the state as it progresses through various lifecycle stages. 

-> Some configuration objects such as **Partition** and **Managed** SPU are managed objects that are created as part of the parent's lifecycle, and they `cannot be directly modified` by the operator.

For detailed schema definition and object life cycles, checkout the [Architecture Overview].

[Architecture Overview]: {{< ref "/docs/architecture/overview" >}}


#### Object Outputs

Each configuration object can converted to different data formats, such as _json_, or _yaml_. Additional data formats are available and can be exposed if required.

Configuration objects may be fetched using filters such as `object name`. 


#### Consumer Behavior

Consumers are also multi-threaded which allows each consumer to read records from multiple data streams simultaneously. Each connection can specify different retrieval properties:

* **Consistency Model** - retrieve records based on their committed state across _replicas_:
    * COMMITTED: fetch only the records that have been replicated `n` times (where `n` defined by _min-live-replicas_)
    * UNCOMMITTED: fetch records that have been stored by _replica leader_. When using UNCOMMITTED read strategy, it is possible to lose records that have already been seen by the consumers. Hence, it should only be used when sporadic message loss is acceptable.
* **Max Bytes** - the maximum number of bytes sent in single message.
    * When a consumer fetches multiple records, the **SPU** batches the result into buffers up to the maximum number of bytes.
    * Default batch size is **1Mb**.


### Fault Tolerance

The Fluvio client can survive SPU failures. All data streams are replicated across multiple SPUs to prevent data loss. 

When a data stream is created, one of the SPUs is elected as leader and the others become followers. Fluvio clients look-up the SPU leaders to produce or consume records.

<img src="../images/prod-cons-before-failover.svg"
     alt="Producer/Consumer"
     style="justify: center; max-width: 475px" />


If the SPU leader becomes unreachable, an election is triggered and one of the SPU followers becomes the leader. The client detects the SPU leader failure and **automatically switches over** to the new leader.

<img src="../images/prod-cons-after-failover.svg"
     alt="Producer/Consumer Failover"
     style="justify: center; max-width: 475px" />

For additional information on the election algorithm, checkout [Election Design].

[Election Design]: {{< ref "/docs/architecture/replica-election" >}}

## Client Profiles

The client library utilizes profiles to hide the complexity associated with the connection configuration. Furthermore, profiles allows the client library to manage multiple Fluvio clusters from the same client instance. Simply switch the profile and all subsequent operations are applied to a different cluster.

For additional information on Profile management, checkout [Fluvio Profiles] section.

[Fluvio Profiles]: {{< ref "/cli/commands/profile" >}}

## Client Workflow

All client operations follow a similar pattern.

1. Create a profile (one time operation).
2. Connect to the cluster, using the profile created above.
3. Use the [Admin API]({{< ref "#admin-api" >}}) to configure or retrieve objects (optional).
4. Produce or Consume records:
    * Use the [Producer API] to send records to a data stream.
    * Use the [Consumer API] to retrieve records from a data stream.

The Fluvio Client library is multi-threaded, and it can simultaneously connect to _multiple clusters_, and concurrently _produce and consume_ one or more data streams.

[Admin API]: {{< ref "#admin-api" >}}
[Producer API]: {{< ref "#producer-api" >}}
[Consumer API]: {{< ref "#consumer-api" >}}