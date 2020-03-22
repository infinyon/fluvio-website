---
title: Streaming Controller (SC) Architecture
menu: SC
weight: 20
---

**Streaming Controller (SC)** is the central coordinator and the **authoritative** entity of the cluster. It manages configuration changes, provisions SPUs, performs replica assignment, coordinates communication with external clients, and sends periodic reconciliation updates.

{{< image src="sc-architecture.svg" alt="SC Coordinator" justify="center" width="530" type="scaled-90">}}

The SC leverages a **Key-Value (KV) store** to persist cluster object configurations.


#### Key-Value Store

**Fluvio** is designed to work seamlessly with **{{< target-blank title="Kubernetes" url="https://kubernetes.io" >}}** and **{{< target-blank title="etcd" url="https://etcd.io" >}}** **KV** store. The **KV interface** is store agnostic and can be extended to support alternative implementations such {{< target-blank title="Consul" url="https://consul.io" >}}, {{< target-blank title="Zookeeper" url="https://zookeeper.apache.org" >}}, or in-memory stores.


## Fluvio Objects

There are four object types in a Fluvio cluster: **SPUs**, **SPU-Groups**, **topics**, and **partitions**. The objects follow the Kubernetes paradigm with two nested fields that govern the object’s configuration: the spec and the status. The **spec** expresses desired state and the **status** describes current state. 

### SPUs

The spec describes **SPU configuration** parameters and the state captures the **online/offline** status. The [Connection Manager]({{< relref "#connection-manager" >}}) monitors changes in **connectivity** between the SC and SPUs, and it keeps the status updated.

{{< code yaml >}}
spec:
  spuId: 100
  spuType: "custom"
  rack: "Zone1"
  publicEndpoint:
    port: 9005
    ingress:
      - hostname: localhost 
    encryption: TLS
  privateEndpoint: 
    port: 9006
    host: localhost
    encryption: TLS
status:
    resolution: online
{{< /code >}}

There are two types of SPUs: managed and custom. **Managed** SPUs are provisioned and maintained by Fluvio, whereas **custom SPUs** are provisioned and managed out of band. Fluvio has the ability to support **managed and custom** SPUs at the same time. 

All SPUs are identified by the **SPU Id** which must be unique across the cluster.

#### Managed SPUs

Fluvio created **custom operators** and a **helm chart** to integrate **managed SPUs** natively with Kubernetes. Fluvio helm charts have replica sets which ensures a specific number of SPUs are provisioned and maintained by Kubernetes. While Managed SPUs display accurate **state information**, they cannot be modified outside of the helm chart. For additional information, checkout [Kubernetes Integration]({{< relref "k8-integration" >}}) section.

#### Custom SPUs

Custom SPUs are designed for on **Edge** devices, **IOT** devices or environments where it is not feasible or desirable to deploy Kubernetes. While SCs continue to use Kubernetes for configuration management, the SPUs can be deployed in any environment and **managed remotely**. Installing a custom SPU is a 3 step process:

1. Provision a new SPU through the CLI or Kubernetes commands.
2. Start a Custom SPU and point it to the SC.
3. Check the Custom SPU status to see if it has successfully connected to the SC.

Aside from the differences in provisioning described above, the SC manages all SPUs types uniformly.


### SPU Groups

The SPU group ... 

#### Topics


#### Partitions



Fluvio uses **controllers** to actively manage every object’s **state** to match a desired **spec**. This workflow is captured in the object **life cycle** section below.


* K8 paradigm
    * manage through SC API or directly through K8

## Object Life Cycles

* Objects lifecycle: SPUs, SPU-Group, Topic, Partitions.
    * Create, update, delete Patch ... describe workflow
            * receive from KV store... distribute to SPUs
        * Primary purpose of the SC is to push metadata to SPUs
        * Getting status from SPUs and push back to KV store.

## Controllers

* Controllers (implementation detail)
    * Each object has a controller which manages object life cycle
    * Each controller caches data for object types
    * Every increment changes form KV store... it computes the new action... that are propagated through the systems...
        * some actions update the state
        * some actions create or modify other objects
        * actions to distribute data to SPUs
    ```Principles```:
        * Cache is a read-only component and it is only modified by KV store.
    * Each controller is independent from each other.

* Local Cache (Store)

## Connection Manager

* SPU Connection Manager
    * SC never initiates Communication
    * Duplex Channel
    * Sends Spec to SPU & Receives status from SPU
        * Replica topology
    * Leader/follower election
    * Detects 
        * SPU online/offline

## Replica Manager

* Replica Distribution
    * link... 

## Reconciliation

    * all components are designed to be recoverable.
    * compares last know state with the current state and reconcile differences.
    * reconciliation is done out of band with no impact to traffic.
* it can start in any order
* reconciliation
* pushes instructions to the SPU

Augmenting the **SC** dynamically

Restarting the **SC**    

## High Availability

* HA for SC
    * one active SC.. can be multiple backups (upcoming)
    * Availability of the SC has no impact on SPU streaming services.
        * Traffic continues... on all existing connections until SC comes online.
    * SPU 

## Kubernetes Integration

* Kubernetes
    * use Kubernetes API to communicate with KV store
    * at startup we read KubeCtl configuration file to detect Kubernetes cluster host.
    * Link to K9 section (namespaces)

## API Servers

    * External
        * API server, default port...
    * Internal

## Configuration Parameters

* Configuration startup command
    * private & public port, K8...
* default parameters ... storage directory, replication, etc.


## System Initialization

At startup, the **SC** connects to the **KV** store and provisions its internal **global store**. SC global store caches all configuration data: SPUs, topic/partition, and replica assignments.


{{< links >}}
* [SPU Architecture]({{<relref "spu">}})
* [Topic/Partitions]({{<relref "topic-partition">}})
* [Replication]({{<relref "replication">}})
* [Data Persistence]({{<relref "persistence">}})
* [Kubernetes Integration]({{<relref "k8-integration">}})
* [Deployment Models]({{<relref "deployments">}})
{{< /links >}} 