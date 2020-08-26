---
title: Replica Assignment
toc: true
weight: 40
---

**Replica assignment** algorithm is triggered by topic creation and it is responsible for a building a **balanced distribution** of replicas across the SPUs in a Fluvio cluster. **Replicas** from different SPUs are grouped in **replica sets**, where each replica saves a copy of a data stream. Each replica set has a leader and one or more followers that are distributed across available SPUs.

For additional information on replica sets, checkout [replica election](../replica-election).


## Replica Assignment Algorithm

Fluvio replica assignment algorithm ensures that the replica leader and followers are evenly distributed across available SPUs. If you'd rather deploy your own replication algorithm, use [Manual Replica Assignment](#manual-replica-assignment) instead.


### Computed Replica Assignment (CRA)

Fluvio uses computed replica assignment algorithm to generate a **replica map** anytime a new topic is created. The algorithm takes into account the following parameters and configuration objects:

* a list of SPUs 
* the number of partitions
* the replication factor
* ignore rack assignment flag


#### Algorithm

The algorithm uses a **round-robin**, **gap-enabled** distribution assignment. 

In a cluster with **4** SPUs, a topic created with:

|   Replicas   |   Partitions  |
|:------------:|:-------------:|
| **3**        | **15**        |

The algorithm generates the following replica distribution:
 
```text
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
```

Next, the indexed distribution is collapsed in the following **replica map**:

```text
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
```

For **balanced distribution** the algorithm **chains multiple calls sequentially**. The algorithm starts the next run **at the index** where the last run completed. That index is mapped to partition 0. 

In this example, if the last replica assignment completed at index 8, the next run starts at index 9 and partition 0 is assigned replica: [4, 1, 2].


### Computed Replica Assignment with Rack Enabled

**SPUs** may be assigned rack labels to distribute replicas across racks. **CRA with rack enabled** algorithm will fail unless  **all SPUs** in the cluster have **rack** defined.

Rack label earmark SPUs with a locations such as a physical rack name, or a cloud availability zone. The algorithm ensures partitions are distributed **across racks**. To achieve a  **balanced distribution**, you must have the same number of SPUs for each rack.

#### Algorithm

The algorithm is designed to work with SPUs in **any rack assignment**. If the rack assignment is unbalanced, the algorithm fills gaps with a round-robin allocation in the SPU matrix.

The algorithm has the following 3 stages:

* Stage 1: Create a rack centric SPU matrix
* Stage 2: Convert SPU matrix to an SPU sequence
* Stage 3: Generate replica map

##### Example 1 - Balanced Rack Distribution

On a cluster with **12** SPUs evenly distributed across **4** racks, a topic created with:

|  Replicas    |  Partitions   |
|:------------:|:-------------:|
| **4**        | **12**        |


The **3-stage** algorithm generates the following distribution:


```text
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
----------------------------------------------------------------
Partition |   Replicas           rack-a  rack-b  rack-c  rack-d
----------------------------------------------------------------
       0  |   [ 0, 4, 8, 9]      [1]     [ ] 1   [ ] 1   [ ] 1
       1  |   [ 4, 8, 9, 1]      [1] 1   [1] 1   [ ] 2   [ ] 2
       2  |   [ 8, 9, 1, 5]      [1] 2   [1] 2   [1] 2   [ ] 3
       3  |   [ 9, 1, 5, 6]      [1] 3   [1] 3   [1] 3   [1] 3
       4  |   [ 1, 5, 6,10]      [2] 3   [1] 4   [1] 4   [1] 4
       5  |   [ 5, 6,10, 2]      [2] 4   [2] 4   [1] 5   [1] 5
       6  |   [ 6,10, 2, 3]      [2] 5   [2] 5   [2] 5   [1] 6
       7  |   [10, 2, 3, 7]      [2] 6   [2] 6   [2] 6   [2] 6
       8  |   [ 2, 3, 7,11]      [3] 6   [2] 7   [2] 7   [2] 7
       9  |   [ 3, 7,11, 0]      [3] 7   [3] 7   [2] 8   [2] 8
      10  |   [ 7,11, 0, 4]      [3] 8   [3] 8   [3] 8   [2] 9
      11  |   [11, 0, 4, 8]      [3] 9   [3] 9   [3] 9   [3] 9
----------------------------------------------------------------
                    Leaders        3       3       3       3
                    Followers      9       9       9       9
----------------------------------------------------------------
```

Replicas are evenly distributed across racks and SPUs.

##### Example 2 - Unbalanced Rack Distribution

On a cluster with **6** SPUs unevenly distributed across **3** racks: 

* rack-a: **0**
* rack-b: **1, 2**
* rack-c: **3, 4, 5**

For a topic created with:

|  Replicas    |  Partitions   |
|:------------:|:-------------:|
| **3**        |  **6**        |

The **3-stage** algorithm generates the following distribution:

```text
Stage 1: SPU Matrix allocation (sorted by size)
------------------------------------------
    rack-c:  3, 4, 5
    rack-b:  1, 2, _
    rack-a:  0, _, _ 

Stage 2: SPU sequence (read in diagonal)
------------------------------------------
    3, 2, 0, 4, 1, 5

Stage 3: Replica Map
-------------------------------------------------------
Partition |   Replicas          rack-c  rack-b  rack-a
-------------------------------------------------------
       0  |   [3, 2, 0]         [1]     [ ] 1   [ ] 1
       1  |   [2, 0, 4]         [1] 1   [1] 1   [ ] 2
       2  |   [0, 4, 1]         [1] 2   [1] 2   [1] 2
       3  |   [4, 1, 5]         [2] 3   [1] 3   [1] 2
       4  |   [1, 5, 3]         [2] 5   [2] 3   [1] 2
       5  |   [5, 3, 2]         [3] 6   [2] 4   [1] 2
-------------------------------------------------------
                    Leaders       3       2       1 
                    Followers     6       4       2
-------------------------------------------------------
```

Replicas are evenly distributed across SPUs. Racks with a higher number of SPUs handle more replicas. If a power failure occurs on a large rack, leader redistribution may overwhelm the SPUs on the smaller racks.

### Manual Replica Assignment

**MRA** is provisioned through a **replica assignment file**. The file defines a **replica map** that is semantically similar to the **replicaMap** defined in [Topic Status](../sc#topic-status"). In fact, the **replica map** defined as defined in the file is assigned to this field.

The following command creates a topic from a **replica assignment file**:

```bash
$ fluvio topic create --topic custom-topic --replica-assignment ./my-assignment
```

_Validate-only_ flag is available to verify a replica assignment file without applying any changes.

#### Replica Assignment File

**Replica assignment file** defines a **replica map** in JSON format. A replica map with 2 partitions and 3 replicas is defined as follows:

```json
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
```

The **replica map** definition meet the following criteria:

* **id**:
    - must start with 0.
    - must be in sequence, without gaps.

* **replicas**:
    - must have at least one element.
    - all replicas must have the same number of elements.
    - all elements must be unique.
    - all elements must be positive integers.

For additional information on how to check the result of a replica assignment file, checkout [Topics CLI](/docs/cli/topics).

#### Related Topics
-------------------
* [SC Architecture](../sc)
* [SPU Architecture](../spu)
* [Topic/Partitions](../topics-partitions)
* [Replica Election](../replica-election)
* [Client Library](../client)
* [References](../references)