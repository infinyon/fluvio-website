---
title: Custom Resource Definitions
menu: CRDs
weight: 30
---

## SPU Group
##### spugroups.fluvio.infinyon.com

| Field | Type | Description |
|-|-|-|
| spec.replicas | integer | Number of replicas |
| spec.minId | integer | Minimum ID in group |
| spec.template.spec.rack | string | Rack name |
| spec.template.spec.publicEndpoint.port | integer | Port of public endpoint |
| spec.template.spec.publicEndpoint.encryption | string | PLAINTEXT or SSL |
| spec.template.spec.privateEndpoint.port | integer | Port of private endpoint |
| spec.template.spec.privateEndpoint.encryption | string | PLAINTEXT or SSL |
| spec.template.spec.replication.inSyncReplicaMin | integer | Minimum number of in-sync replicas |
| spec.template.spec.storage.logDir | integer | Path to data directory |
| spec.template.spec.storage.size | integer | Storage size |
| spec.template.spec.env[].name | integer | Environment variable name |
| spec.template.spec.env[].value | integer | Environment variable value |
{.full-width}

## SPU
##### spus.fluvio.infinyon.com

| Field | Type | Description |
|-|-|-|
| spec.spuId | string | ID of SPU |
| spec.spuType | string | Custom or Managed |
| spec.rack | string | Rack name |
| spec.publicEndpoint.ingress[].ip | string | IP address of public endpoint |
| spec.publicEndpoint.ingress[].hostname | string | Hostname of public endpoint |
| spec.publicEndpoint.port | integer | Port of public endpoint |
| spec.publicEndpoint.encryption | string | PLAINTEXT or SSL |
| spec.privateEndpoint.host | string | Hostname of private endpoint |
| spec.privateEndpoint.port | integer | Port of private endpoint |
| spec.privateEndpoint.encryption | string | PLAINTEXT or SSL |
| status.resolution | string | SPU status |
{.full-width}


## Topic
##### topics.fluvio.infinyon.com

| Field | Type | Description |
|-|-|-|
| spec.type | string | Type of topic |
| spec.partitions | integer | Partitions count |
| spec.replicationFactor | integer | Replication count |
| spec.ignoreRackAssignment | boolean | Ignore rack assignment |
| spec.customReplicaAssignment[].partition.id | integer | Partition ID |
| spec.customReplicaAssignment[].partition.replicas | integer | Number of replicas |
| status.resolution | string | Topic status |
{.full-width}

## Partition
##### partitions.fluvio.infinyon.com

| Field | Type | Description |
|-|-|-|
| spec.leader | integer | Leader SPU ID |
| spec.replicas[].items | integer | Followers |
| status.resolution | string | Partition status |
| status.isBeingDeleted | boolean | Being deleted |
| status.lsr | integer | Live Replicas |
| status.leader.hw | integer | Leader High Watermark |
| status.leader.leo | integer | Leader End Offset |
| status.replicas | integer | Follower Offsets |
{.full-width}
