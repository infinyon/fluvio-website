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


## Configuration Objects

There are four configuration objects in a Fluvio cluster: **SPU**, **SPU-group**, **Topic**, and **Partition**. The objects follow **Kubernetes paradigm** with two fields that govern the configuration: the spec and the status. **Spec** expresses a desired state and the **status** describes the current state. 


### SPUs

**SPUs spec** has a unique ID, a type, an optional rack, and endpoint identifier for the API servers. **SPU Id** is shared across all SPU types and it must be **globally unique**.

{{< code yaml >}}
spec:
  spuId: 100
  spuType: "Custom"
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
{{< /code >}}

**SPU status** has a resolution field that monitors changes in **connectivity** from the SC's point of view.

{{< code yaml >}}
status:
    resolution: online
{{< /code >}}

There are two types of SPUs: managed and custom. **Managed** SPUs are provisioned and maintained by Fluvio, whereas **custom SPUs** are provisioned and managed out of band. Fluvio has the ability to support multiple **managed and custom** SPUs simultaneously. **SPUs** can be deployed in a virtually unlimited topologies across **availability zones** and **geo-locations**.

#### Custom SPUs

Custom SPUs are designed for **Edge** devices, **IOT** devices or **custom environments** where the infrastructure is managed through deployment tools such as {{< target-blank title="Puppet" url="https://puppet.com" >}}, {{< target-blank title="Chef" url="https://www.chef.io" >}}, or {{< target-blank title="Ansible" url="https://www.ansible.com" >}}. This feature is currently experimental.

##### Install Custom SPUs

The SC requires Custom SPUs to be registered before they are allowed to join the cluster:

1. Register a new Custom SPU in the SC.
2. Configure Custom SPU a network that has connectivity with the SC.
2. Deploy the Custom SPU.
3. Check SPU status on the SC to see if connected successfully.

Aside from the differences in installation, all SPU types are treated the same.


#### Managed SPUs

**Managed SPUs** are groups of SPUs that are **scaled independently** based on user configurable replication factors. Managed SPUs are configured and owned by SPU-groups, defined below.


### SPU Groups

Fluvio **SPU-groups** define the configuration parameters used for provisioning groups of **Managed SPUs**. 

{{< image src="spu-groups.svg" alt="SpuGroups" justify="center" width="740" type="scaled-98">}}


**Replica** specifies the number of SPUs in a group and it can be dynamically changed: 

* if higher, new SPUs are **provisioned**
* if lower, SPUs with the higher ids are **terminated**.

 **MinId** is the Id of the first SPU in the replica range. 
 
 **Template** defines configuration parameters passed to all SPUs in the group. While there are many configuration parameters in the template section, the most relevant one in the **storage/size**. If no **size** is specified, it default to 1 gigabyte. 

##### SPU-group Spec

{{< code yaml >}}
spec:
  replicas: 2
  minId: 11
  template:
    storage:
        size: 20Gi
{{< /code >}}


##### SPU-group Status

{{< code yaml >}}
status:
    resolution: Reserved
{{< /code >}}

SPU-group status has 3 **resolutions**: Init, Invalid, and Reserved. If the group is marked invalid, a **reason** field describes the error.

Checkout the [SPU-groups]({{< relref "../cli/spu-groups" >}}) CLI for additional information.


### Topics

**Topics** define configuration parameters for data streams. A topic may have one or more partitions and a replication factor. **Partitions** are distributed evenly across the SPUs. **Replication factor** must be a integer from 1 to the total number of SPUs and applies to all partitions.

##### Topic Spec

{{< code yaml >}}
spec:
  partitions: 6
  replicationFactor: 3
{{< /code >}}

A topic with *6 partitions* and a *replication factor of 3* on a new cluster generates the following distribution:

{{< image src="partition-assignment.svg" alt="Partition Assignment" justify="center" width="560" type="scaled-75">}}

The algorithm that computes partition/replica distribution is described in the [Replication]({{< relref "replication" >}}) section. 

Fluvio also supports **manual** partition/replica distribution through a **replica assignment file**. The file format is described in the [Topic/Partition]({{< relref "topic-partition" >}}) section.

##### Topic Status

{{< code yaml >}}
status:
  resolution: Provisioned
  replicaMap: 
    - 0: [0, 1, 2]
    - 1: [1, 2, 0]
    - 2: [2, 1, 0]
    - 3: [0, 1, 2]
    - 4: [1, 2, 0]
    - 5: [2, 1, 0]
{{< /code >}}

The **resolution** reflects the status of topic. Provisioned topics are in working conditions, others are pending resources or have an invalid configuration:

* Init - topic is initializing.
* Pending - configuration is valid, topic is provisioning SPUs for replica map.
* InsufficientResources - replica map cannot be created due to lack of resources. 
* InvalidConfig - invalid configuration (for manual replica assignment).
* Provisioned - topic is successfully allocated.

If an errors occurs **reason** field describes the cause of the error.

**Replica Map** defines the partition/replica distribution:

* Partitions are the indexes 0 to 5.
* Replicas list the SPUs with the leader in first position. 

In this example, partition with index 4 has 3 replicas, where:

* SPU 1 is the leader.
* SPU 0 and SPU 2 are followers.


### Partitions

**Partition** are internal configuration objects generated by the **SC**. When a new topic is provisioned, the **SC** performs the following operations:

1. **generates a partition map** and store in the topic status,
2. **creates a partition object** for each row in the partition map,
3. **assign each partition** to the SPU leader.

Topics and partitions are linked through a **parent-child** relationship. If a topic is deleted, all child partitions are automatically removed.

{{< image src="topic-2-partitions.svg" alt="Topic 2 Assignment" justify="center" width="680" type="scaled-90">}}

**SC** is responsible for the configuration in the **Partition Spec** and the **SPU** leader is responsible for the **Partition Status**.

##### Partition Spec

{{< code yaml >}}
spec:
  initialLeader: 101
  replicas: [101, 102]
{{< /code >}}

The **SC** defines **replica assignment** and the SPU **initial leader**. After initial allocation, the **SC** notifies SPU **leader** and **followers** of the new partition.

##### Partition Status

{{< code yaml >}}
status:
  leader: 101
  lrs: [101, 102]
  ...
{{< /code >}}

**SPU leader** is responsible for managing the **Live Replicas (lrs)** and other data streaming related parameters.

Replica management algorithm and other status fields are documented in the [SPU]({{< relref "spu" >}}) section.


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