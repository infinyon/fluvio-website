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

**Topics** define configuration parameters for data streams. A topic may have one or more partition and a replication factor. **Partitions** split the data into independent slices that can be managed by different SPUs. **Replication factor** defines the number of copies of data across SPUs. Replication factor must be is an a integer from 1 to the total number of SPUs.

**Partitions** consists of replicas, where each replica can be a leader or a follower. **Leaders** maintain the primary data set and **followers** store a copy of the data. Leaders and followers must map to independent **SPUs**.

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

**Resolution** reflects the status of topic:

* Init - topic is initializing.
* Pending - configuration is valid, topic is provisioning SPUs for replica map.
* InsufficientResources - replica map cannot be created due to lack of SPUs. 
* InvalidConfig - invalid configuration (for manual replica assignment).
* Provisioned - topic is successfully allocated.

If an errors occurs **reason** field describes the cause of the error.

**Replica Map** defines the partition/replica distribution. The first number is the **partition index** and the array is a **list the SPUs** with the leader in first position. 

In this example, **4: [1, 2, 0]** defines:

* a partition index 4
* a list of SPUs
    * SPU 1 is the leader.
    * SPU 0 and SPU 2 are followers.


### Partitions

**Partition** are system managed configuration objects. When a new topic is provisioned, the **SC** performs the following operations:

1. **generates a partition map** and store in the topic status,
2. **creates a partition object** for each row in the partition map,
3. **assigns each partition** to the SPU leader.

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

**SPU leader** is responsible for managing **Live Replicas (lrs)** and other data streaming related parameters.

Replica management, election, and all other status fields are documented in the [SPU]({{< relref "spu" >}}) section.


## Object Workflows

**SC** design an event driven architecture that **captures cluster changes** and keeps the SPUs and the Key-Value Store **synchronized**. 

{{< image src="sc-workflows.svg" alt="SC Controller" justify="center" width="800" type="scaled-98">}}

The SC uses a **common workflow** to process events:

1. Readers
    * capture incoming events
    * invoke Metadata Dispatcher
2. Metadata Dispatcher
    * saves events metadata in **local store** 
    * invokes a Controller
3. Controllers
    * each **controller** applies object centric business logic
    * generate actions
4. Action Dispatcher
    * distributes actions
5. Updaters 
    * formats actions
    * sends update to external entity

###### Local Store

Metadata dispatcher maintains a **Local Store** of read-only objects types that mirror the KV store. Objects in the local store can only be updated by KV Store requests. The local store is utilized the **Object Controllers** as they translated events into the actions.

###### Controllers

SPU, Topic, and Partition Controllers run independently and manage the workflows for their designated objects. 

#### SPU Controller

SPU Controller listens for SPU events:

* **Add SPU**
    
    SPU controller asks Connection Manager to add SPU to the connection list. If the SPU is online, the controller asks the KV Store to change SPU status to **online**.

* **Delete SPU**

    SPU controller removes the connection to the SPU.


#### Topic Controller

Topic Controller listens for Topic and SPU events:

* **Add Topic**

... 

* **Add Delete**

...

* **Add SPU**

... 

* **Modify SPU**

...


#### Partition Controller

Partition Controller listens for Partition and SPU events as well as SPU notifications:

* **Add Partition**

...

* **Mod Partition**

...

* **Del Partition**

...

* **Mod SPU**

...

* **SPU Change LRS**

...


#### Chained Operations

Chained operations, such as topic - partition workflows, will traverse the systems multiple times by sequentially applying changes to KV store objects. These objects in turns notify Metadata Dispatcher through a new event. 


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