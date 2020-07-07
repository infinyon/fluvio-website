---
title: Streaming Controller (SC) Architecture
menu: SC
toc: true
weight: 20
---

**Streaming Controller (SC)** is the central coordinator and the **authoritative** entity of the cluster. It manages configuration changes, provisions SPUs, performs replica assignment, coordinates communication with external clients, and sends periodic reconciliation updates.

{{< image src="architecture/sc-architecture.svg" alt="SC Coordinator" justify="center" width="530" type="scaled-90">}}

The SC leverages a **Key-Value (KV) store** to persist cluster object configurations.


#### Key-Value Store

**Fluvio** is designed to work seamlessly with **<a href="https://kubernetes.io" target="_blank">Kubernetes</a>** and **<a href="https://etcd.io" target="_blank">etcd</a>** **KV** store. The **KV interface** is store agnostic and can be extended to support alternative implementations such <a href="https://consul.io" target="_blank">Consul</a>, <a href="https://zookeeper.apache.org" target="_blank">Zookeeper</a>, or in-memory stores.

#### Default Ports

SCs have a **public** and a **private** server that are attached to the following ports:

* **Public Port**: 9003
* **Private Port**: 9004


## Core Objects

There are four core objects in a Fluvio cluster: **SPU**, **SPU-group**, **Topic**, and **Partition**. The objects follow **Kubernetes paradigm** with two fields that govern the configuration: the spec and the status. **Spec** expresses a desired state and the **status** describes the current state. 


### SPUs

**SPUs spec** has a unique ID, a type, an optional rack, and endpoint identifier for the API servers. **SPU Id** is shared across all SPU types and it must be **globally unique**.

```yaml
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
```

**SPU status** has a resolution field that monitors changes in **connectivity** from the SC's point of view.

```yaml
status:
    resolution: online
```

There are two types of SPUs: managed and custom. **Managed** SPUs are provisioned and maintained by Fluvio, whereas **custom SPUs** are provisioned and managed out of band. Fluvio has the ability to support multiple **managed and custom** SPUs simultaneously. **SPUs** can be deployed in a virtually unlimited topologies across **availability zones** and **geo-locations**.

#### Custom SPUs

Custom SPUs are designed for **Edge** devices, **IOT** devices or **custom environments** where the infrastructure is managed through deployment tools such as <a href="https://puppet.com" target="_blank">Puppet</a>, <a href="https://www.chef.io" target="_blank">Chef</a>, or <a href="https://www.ansible.com" target="_blank">Ansible</a>. This feature is currently experimental.

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

{{< image src="architecture/spu-groups.svg" alt="SpuGroups" justify="center" width="740" type="scaled-98">}}


**Replica** specifies the number of SPUs in a group and it can be dynamically changed: 

* if higher, new SPUs are **provisioned**
* if lower, SPUs with the higher ids are **terminated**.

 **MinId** is the Id of the first SPU in the replica range. 
 
 **Template** defines configuration parameters passed to all SPUs in the group. While there are many configuration parameters in the template section, the most relevant one in the **storage/size**. If no **size** is specified, it default to 1 gigabyte. 

##### SPU-group Spec

```yaml
spec:
  replicas: 2
  minId: 11
  template:
    storage:
        size: 20Gi
```


##### SPU-group Status

```yaml
status:
    resolution: Reserved
```

SPU-group status has 3 **resolutions**: Init, Invalid, and Reserved. If the group is marked invalid, a **reason** field describes the error.

Checkout the [SPU-groups CLI](/docs/cli/spu-groups) for additional information.


### Topics

**Topics** define configuration parameters for data streams. A topic may have one or more partition and a replication factor. **Partitions** split the data into independent slices that can be managed by different SPUs. **Replication factor** defines the number of copies of data across SPUs. Replication factor must be is an a integer from 1 to the total number of SPUs.

**Partitions** consists of replicas, where each replica can be a leader or a follower. **Leaders** maintain the primary data set and **followers** store a copy of the data. Leaders and followers must map to independent **SPUs**.

##### Topic Spec

```yaml
spec:
  partitions: 6
  replicationFactor: 3
```

A topic with *6 partitions* and a *replication factor of 3* on a new cluster generates the following distribution:

{{< image src="architecture/partition-assignment.svg" alt="Partition Assignment" justify="center" width="560" type="scaled-75">}}

The algorithm that computes partition/replica distribution is described in the [Replication Assignment](../replication) section. 

Fluvio also supports **manual** partition/replica distribution through a **replica assignment file**. The file format is described in the [Topics CLI](/docs/cli/topics) section.

##### Topic Status

```yaml
status:
  resolution: Provisioned
  replicaMap: 
    - 0: [0, 1, 2]
    - 1: [1, 2, 0]
    - 2: [2, 1, 0]
    - 3: [0, 1, 2]
    - 4: [1, 2, 0]
    - 5: [2, 1, 0]
```

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

**Partition** are configuration objects managed by the system. When a new topic is created, the **SC** performs [replication assignment](../replication) to generate the partitions:

1. **generate a partition map** and store in the topic status,
2. **create a partition object** for each row in the partition map,
3. **assign each partition** to the SPU leader.

Topics and partitions are linked through a **parent-child** relationship. If a topic is deleted, all child partitions are automatically removed.

{{< image src="architecture/topic-2-partitions.svg" alt="Topic 2 Assignment" justify="center" width="680" type="scaled-90">}}

**SC** is responsible for the configuration in the **Partition Spec** and the **SPU** leader is responsible for the **Partition Status**.

##### Partition Spec

```yaml
spec:
  initialLeader: 101
  replicas: [101, 102]
```

The **SC** defines **replica assignment** and the SPU **initial leader**. After initial allocation, the **SC** notifies SPU **leader** and **followers** of the new partition.

##### Partition Status

```yaml
status:
  leader: 101
  lrs: [101, 102]
  ...
```

**SPU leader** is responsible for managing **Live Replicas (lrs)** and other data streaming related parameters.

Replica management, election, and all other status fields are documented in the [SPU Architecture](../spu) section.


## Workflows

**SC** design is an event driven architecture that **captures cluster changes** and keeps the SPUs and the Key-Value store **synchronized**. 

{{< image src="architecture/sc-workflows.svg" alt="SC Controller" justify="center" width="800" type="scaled-98">}}

The SC uses a **common workflow** to process all event types:

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

Metadata dispatcher maintains a **Local Store** of read-only objects types that mirror the KV store. Objects in the local store can only be updated by KV Store requests. The local store is utilized by **Controllers** to **transform** events into the actions.

## Controllers

SPU, Topic, and Partition Controllers run independently and manage the workflows for their designated objects. 


### SPU Controller

SPU Controller listens for SPU events from KV store and events from Connection Manager.

{{< image src="architecture/spu-controller.svg" alt="SPU Controller" justify="center" width="440" type="scaled-60">}}

* **Add SPU**
    
    SPU controller creates an **action** to add SPU to the Connection Manager.

* **Modify SPU**
    
    SPU controller creates an **action** to update SPU in the Connection Manager.

* **Delete SPU**

    SPU controller creates an **action** to delete SPU from the Connection Manager.

* **Online/Offline**

    When connection status changes, the controller creates an **action** to update SPU **resolution status** in the KV store. 


### Topic Controller

Topic Controller listens for Topic and SPU events from KV store.

{{< image src="architecture/topic-controller.svg" alt="Topic Controller" justify="center" width="440" type="scaled-60">}}

* **Add Topic**

    Topic controller creates an **action** to update Topic **status** resolution to **Init** in the KV store.

* **Modify Topic**

    For topics with status resolution **Init** or **Invalid**, the controller validates partition and replication configuration parameters. Upon validation, the controller:

    * Params _OK_ - creates an **action** to update Topic **status** resolution to **Pending** in KV store.
    * Params _Invalid_ - creates an **action** to update Topic **status** resolution to **Invalid** in KV store.

    For topics with status resolution **Pending** or **InsufficientResources**, the controller checks if the number SPUs meets the replication factor. Upon validation, the controller:

    * SPUs _Ok_ - generates a Replica Map and creates the following **actions** for the KV Store:
    
        * an **action** to update **status** resolution to **Provisioned** and replicaMap to **Replica Map**.
        * an **action** to create a new **Partition** for each entry in the Replica Map.

    * _Not enough SPUs_ - creates an **action** to update Topic **status** resolution to **InsufficientResources** in KV store.

* **Add SPU**

    Topic controller selects all topics with **status** resolution in **Pending** or **InsufficientResources** and  generates a new Replica Map. 

    * for each topic with a new Replica Map, the controller creates **2 actions** for the KV Store:

        * an **action** to update **status** resolution to **Provisioned** and replicaMap to **Replica Map**.
        * an **action** to create a new **Partition** for each entry in the Replica Map.
    

### Partition Controller

Partition Controller listens for Partition and SPU events from KV store and events from Connection Manager.

{{< image src="architecture/partition-controller.svg" alt="Partition Controller" justify="center" width="440" type="scaled-60">}}

* **Add Partition**

    Partition controller creates the following actions:
    
    * **action** to add Partition to the Connection Manager.
    * **action** to update Partition **status** resolution to **Offline** in KV store.

* **Modify Partition**

    Partition controller creates an **action** to update **Partition spec** in the Connection Manager.

* **Delete Partition**

    Partition controller creates an **action** to delete **Partition spec** from the Connection Manager.

* **Modify SPU**

    Partition controller checks if SPU status changed from **Online -> Offline** and it retrieves all Partitions that with the SPU is the leader.
    
    * for each partition, the controller computes a **new leader** candidates. Upon completion, the controller:
    
        * leader computed: creates an **action** to update leader of Partition Status in Connection Manager.
        * no suitable leader found: creates **action** to update Partition **status** resolution to **Offline** in KV store.

    Partition controller checks if SPU status changed from **Offline -> Online** and it retrieves all Partitions with **status** resolution **Offline**. 
    
    * for each partition where this SPU is not the leader, the controller checks if the SPU eligible to become leader.
    
        * SPU eligible: the controller creates an **action** to update Partition **status** leader to the SPU id in KV store.
        * SPU not suitable to be leader: the controllers leaves leave partition unchanged.

* **Change LRS**

    Partitions controller receives **Live Replicas (LRS)** updates form **Connection Manager**. The matching Partition is updated in the KV store with the following action:
    
    * Update Partition status: **action** to update Partition **status** resolution to **Online**, **leader** to LRS.leader and **replica** to LRS.replica.


## Connection Manager

**Connection Manager (CM)** is an **SC Server** module responsible for the connections between **SC** and **SPUs**. The **CM** only **accepts** connections from **registered** SPUs.

#### Connection Setup

A connection is established in the following sequence:

{{< image src="architecture/connection-setup.svg" alt="Connection Manager" justify="center" width="780" type="scaled-99">}}

* **SPU Controller** sends **add SPU spec** to **CM**.
* **Partition Controller** sends **add Partitions** to **CM**
* **CM** saves **SPU** and **Partitions** in local cache.
* **SPU** requests a connection authorization.
* **CM** authorizes registered SPUs and rejects all others.
* **CM** sends authorization **accepted** to SPU.
* **CM** saves connection stream in local cache.
* **CM** notifies all relevant **Controllers** to change SPU status to **online**
* **CM** sends **SPU Spec** and **Partition Specs** relevant to the SPU.
* **CM** receives continuous **LRS updates** from all SPUs in the cluster.

After the connection is established, both endpoints can initiate requests.

#### Connection Failure

If the connection drops is due to network failures or SPU going offline, the **CM** takes the following remediation steps:

* **CM** removes connection stream from local cache.
* **CM** notifies all relevant controllers that SPU status is **offline**.

When the SPU come back online it initiates a new connection as described in the **Connection Setup** section.

#### Live Replica (LRS) Updates

**Live Replicas (LRS)** are continuous updates sent by **leader SPUs** to the **CM** to report changes in replica status. The **CM** forwards the requests to relevant **Controllers** for processing.


#### Related Topics
-------------------
* [SPU Architecture](../spu)
* [Replication](../replication)
* [Kubernetes Integration](../k8-integration)
* [Deployment Models](../deployments)
