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

#### CRA with Rack disabled

**CRA with rack disabled** can be used if one of the two conditions are met: 

* **no racks defined** for any of the **SPUs**. 
* **ignore rack flag** is set. 

##### Algorithm

The algorithm uses a **round-robin**, **gap-enabled** distribution assignment. 

The cluster has:

*  SPUs : **5**

The topic configuration parameters are:

* partitions : **15**
* replicas :  **3**

If the algorithm starts at index 0, it generates the following replica distribution:
 
{{< code json >}}
------------------------------------------
|  idx | 5 x SPUs                | gaps  |
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

Which translates into the following **replica map**:

{{< code >}}
---------------------------
| Partition |  Replicas   |
---------------------------
|       0   |  [0, 1, 2]  |
|       1   |  [1, 2, 3]  |
|       2   |  [2, 3, 4]  |
|       3   |  [3, 4, 0]  |
|       4   |  [4, 0, 1]  |
|       5   |  [0, 2, 3]  |
|       6   |  [1, 3, 4]  |
|       7   |  [2, 4, 0]  |
|       8   |  [3, 0, 1]  |
|       9   |  [4, 1, 2]  |
|      10   |  [0, 3, 4]  |
|      11   |  [1, 4, 0]  |
|      12   |  [2, 0, 1]  |
|      13   |  [3, 1, 2]  |
|      14   |  [4, 2, 3]  |
|      15   |  [0, 1, 2]  |
---------------------------   
{{< /code >}}

For **balanced distribution** the algorithm **chains multiple calls sequentially**. The algorithm starts the next run **at the index** where the last run completed. That index is mapped to partition 0. 

In this example, if the last replica assignment completed at index 8, the next run starts at index 9 and partition 0 is assigned replica: [4, 1, 2].


#### CRA with Rack Enabled

**CRA with rack enabled** can be used if **all SPUs** in the cluster have **rack** defined.

Rack is used to earmark SPUs a locations such as a server rack, or a cloud availability zones. The algorithm ensures partitions are distributed **across racks**. It is important to have an equal number of SPUs in each rack to get a **balanced distribution**. 

##### Algorithm

The algorithm is designed to work with SPUs in **any rack assignment**. If the rack assignment is unbalanced, the algorithm fills gaps with a round-robin allocation in the SPU matrix.

The algorithm has the following 3 stages:

* Stage 1: Create a rack centric SPU matrix
* Stage 2: Convert SPU matrix to an SPU sequence
* Stage 3: Generate replica map

###### Example 1 - Balanced Rack Distribution

The cluster has **4** racks with **11** SPUs:

* rack-a: **0, 1, 2**
* rack-b: **3, 4, 5**
* rack-c: **6, 7, 8**
* rack-d: **9, 10, 11**

The topic configuration parameters are:

* partitions : **12**
* replicas :  **4**

{{< code json >}}
Stage 1: SPU Matrix allocation
------------------------------------------
    rack-a:  0, 1,  2
    rack-b:  3, 4,  5
    rack-c:  6, 7,  8
    rack-d:  9, 10, 11

Stage 2: SPU sequence (read in diagonal)
------------------------------------------
    0, 4, 8, 9, 1, 5, 6, 10, 2, 3, 7, 11

Stage 3: Replica Map
--------------------------------------------------------------
Partition |   Replicas         rack-a  rack-b  rack-c  rack-d
--------------------------------------------------------------
       0  |   [ 0, 4, 8, 9]      [1]    [ ]1   [ ]1    [ ]1
       1  |   [ 4, 8, 9, 1]      [1]1   [1]1   [ ]2    [ ]2
       2  |   [ 8, 9, 1, 5]      [1]2   [1]2   [1]2    [ ]3
       3  |   [ 9, 1, 5, 6]      [1]3   [1]3   [1]3    [1]3
       4  |   [ 1, 5, 6,10]      [2]3   [1]4   [1]4    [1]4
       5  |   [ 5, 6,10, 2]      [2]4   [2]4   [1]5    [1]5
       6  |   [ 6,10, 2, 3]      [2]5   [2]5   [2]5    [1]6
       7  |   [10, 2, 3, 7]      [2]6   [2]6   [2]6    [2]6
       8  |   [ 2, 3, 7,11]      [3]6   [2]7   [2]7    [2]7
       9  |   [ 3, 7,11, 0]      [3]7   [3]7   [2]8    [2]8
      10  |   [ 7,11, 0, 4]      [3]8   [3]8   [3]8    [2]9
      11  |   [11, 0, 4, 8]      [3]9   [3]9   [3]9    [3]9
--------------------------------------------------------------
                    Leaders        3      3      3      3
                    Followers      9      9      9      9
--------------------------------------------------------------
{{< /code >}}

The algorithm yields an even distribution with the same number of leader and followers across all racks.

###### Example 2 - Unbalanced Rack Distribution

The cluster has **3** racks with **6** SPUs:

* rack-a: **0**
* rack-b: **1, 2**
* rack-c: **3, 4, 5**

The topic configuration parameters are:

* partitions : **6**
* replicas :  **3**

{{< code json >}}
Stage 1: SPU Matrix allocation (sorted by rack with most SPUs)
------------------------------------------
    rack-c:  3, 4, 5
    rack-b:  1, 2, _
    rack-a:  0, _,  _ 

Stage 2: SPU sequence (read in diagonal)
------------------------------------------
    3, 2, 0, 4, 1, 5

Stage 3: Replica Map
-------------------------------------------------------
Partition |   Replicas         rack-c  rack-b  rack-a
-------------------------------------------------------
       0  |   [3, 2, 0]         [1]    [ ]1    [ ]1
       1  |   [2, 0, 4]         [1]1   [1]1    [ ]2
       2  |   [0, 4, 1]         [1]2   [1]2    [1]2
       3  |   [4, 1, 5]         [2]3   [1]3    [1]2
       4  |   [1, 5, 3]         [2]5   [2]3    [1]2
       5  |   [5, 3, 2]         [3]6   [2]4    [1]2
-------------------------------------------------------
                    Leaders       3      2      1 
                    Followers     6      4      2
-------------------------------------------------------
{{< /code >}}

Each SPU gets a **fair share** of leaders and followers. 

The racks with higher number of SPUs handle **more replicas**. In the event of a power failure on rack-c, SPUs on racks a and b may **get overwhelmed** as the leaders are redistributed.

{{< links >}}
* [SC Architecture]({{<relref "sc">}})
* [SPU Architecture]({{<relref "spu">}})
* [Data Persistence]({{<relref "persistence">}})
* [Kubernetes Integration]({{<relref "k8-integration">}})
* [Deployment Models]({{<relref "deployments">}})
{{< /links >}} 