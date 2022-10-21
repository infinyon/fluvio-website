---
title: Outbound PostgreSQL Connector
menu: Postgres
---

The Postgres Outbound Connector reads events from a
fluvio topic and inserts them into a postgres database.

## Overview

The Postgres Outbound connector consumes replication events that a postgres
inbound connector emits to a fluvio topic. In the near future, it will be able
to consume events produced by other database inbound connectors.


## Connector config `parameters`

Fluvio Connectors may be launched as a "Managed connector" when running Fluvio
in Kubernetes, or as a "Local connector" which may connect to Fluvio anywhere.
When using Fluvio Postgres Outbound as a Managed Connector, you'll need to provide a
configuration file that looks like the following:


Below are descriptions of the purpose of each parameter:

### `url`
*required*

The login URL for your Postgres database.

This should contain
  your username, password, database hostname, and port.
  - Example: `postgres://user:password@hostname:port/database_name`
  
  This key can also be specified under config `secrets`.
  
  See: [`FLUVIO_PG_DATABASE_URL`]({{<ref "#fluvio_pg_database_url">}})

 ### `topic`
*required*

The name of the Fluvio Topic that the connector should
  produce CDC events to.
  - Example: `postgres-topic`

## Connector config `secrets`
### `FLUVIO_PG_DATABASE_URL`

Alternative configuration path for config parameter [`url`]({{<ref "#url">}})


#### Example connector config 
%copy%

{{<code file="code-blocks/yaml/connectors/outbound-examples/outbound-postgres.yaml" lang="yaml" copy=true >}}

## Data Events

The Postgres Outbound Connector consumes [replication events from the Postgres
 Inbound connector]({{<ref "/connectors/inbound/postgres#data-events" >}}) and runs the
appropriate SQL for the corresponding postgres outbound database.

## Connector Resume

Should the connector for some reason stop or crash, when starting up again, the
connector will start the fluvio consumer stream at the `current_offset` found
in `fluvio.offset` table in the postges server. This will prevent any duplicate
inserts or deletes.
