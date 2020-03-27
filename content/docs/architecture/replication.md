---
title: Replication Design
menu: Replication
weight: 50
---

Fluvio **real-time replication architecture** improves **data availability** without sacrificing performance as it concurrently serves a large number of **producers** and **consumers**.

#### Topics

Fluvio's uses **Topics** to define **data distribution** criteria, such as slicing and replication.  **Topic partitions** create data slices that are managed independently by different SPUs. **Topic replication** defines the number of copies for each data slice.

{{< fluvio >}}
$ fluvio topic create --topic topic-a --partitions 1 --replication 3
{{< /fluvio >}}

 A **replication** of 3 creates **3 copies** of data where each copy is written on a different SPU:

{{< image src="leader-followers.svg" alt="Leader, Followers" justify="center" width="520" type="scaled-90">}}

The SPUs in a replication assignment have different **roles**:

* SPU-1 is the **Replica Leader**. 
* SPU-2 and SPU-3 are **Replica Followers**.

## Replication Assignment

Fluvio has two types for replication assignment: manual and computed. **Manual Replica Assignment** provides the ability to apply a **custom replication** algorithm. **Computed Replica Assignment** uses the default replication algorithm to compute replica map.

### Manual Replica Assignment (MRA)

**MRA** is provisioned through a **replica assignment file**. The file defines a **replica map** that is semantically similar to the **replicaMap** defined in [Topic Status]({{< relref "sc#topic-status" >}}). In fact, the **replica map** defined in the file is assigned to this field.

The following command creates a topic from a **replica assignment file**:

{{< fluvio >}}
$ fluvio topic create --topic custom-topic --replica-assignment ./my-assignment
{{< /fluvio >}}

_Validate-only_ flag is available to verify a replica assignment file without applying changes.

#### Replica Assignment File

**Replica assignment file** defines a **replica map** in JSON format. A replica map with 2 partitions and 3 replicas is defined as follows:

{{< code toml >}}
{ 
    "partitions": [{
            "id": 0,
            "replicas": [0, 1, 2] 
        }, 
        {
            "id": 1,
            "replicas": [1, 2, 0] 
        }
    ]
}
{{< /code >}}

The **replica map** definition meet the following criteria:

* **id**:

    - must start with 0.
    - must be in sequence, without gaps.

* **replicas**:

    - must have at least one element.
    - all replicas must have the same number of elements.
    - all elements must be unique.
    - all elements must be positive integers.

For additional information on how to check the result of a replica assignment file, see [Topic CLI]({{< relref "../cli/topics" >}}).

### Computed Replica Assignment (CRA)

**CRA** uses Fluvio's replica assignment algorithm to generate a **replica map**. The algorithm takes into account the following elements:

* a list of SPUs 
* rack name for each SPU
* the number of partitions
* the replication factor
* ignore rack assignment flag

There are **two algorithms** to compute a **replica map**:

* with rack assignment 
* without rack assignment

#### CRA without Rack Assignment

CRA without rack assignment can be invoked if **no SPUs** have rack define, or **ignore rack flag** is set. 
The algorithm is based on a **round-robin**, **gap-enabled** distribution assignment. 

For example, in an environment with **5 x SPUs**, a topic created with:

* **15 x partitions** 
* **3 x replicas** 

starting at index 0, generates the following **replica map**:
 
{{< code json >}}
------------------------------------------
|p-idx | 5 x SPUs                | gaps  |
------------------------------------------
|    0 |012                      |   0   |
|    1 | 123                     |   0   |
|    2 |  234                    |   0   |
|    3 |   340                   |   0   |
|    4 |    401                  |   0   |
|    5 |     0 23                |   1   |
|    6 |      1 34               |   1   |
|    7 |       2 40              |   1   |
|    8 |        3 01             |   1   |
|    9 |         4 12            |   1   |
|   10 |          0  34          |   2   |
|   11 |           1  40         |   2   |
|   12 |            2  01        |   2   |
|   13 |             3  12       |   2   |
|   14 |              4  23      |   2   |
|   15 |012                      |   0   |
------------------------------------------
{{< /code >}}

At index 15, the algorithm repeat.

##### Algorithm Notes

The algorithm:

* can **start at any index** and continue the for multiple topic allocations in sequence.
* produces **well balanced distribution** regardless of the number of **SPUs**.

#### CRA with Rack Assignment

CRA with rack assignment can only be called if **all SPUs** in the cluster have the rack field defined.

 


{{< links >}}
* [SC Architecture]({{<relref "sc">}})
* [SPU Architecture]({{<relref "spu">}})
* [Data Persistence]({{<relref "persistence">}})
* [Kubernetes Integration]({{<relref "k8-integration">}})
* [Deployment Models]({{<relref "deployments">}})
{{< /links >}} 