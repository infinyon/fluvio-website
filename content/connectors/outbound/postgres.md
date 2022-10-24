---
title: Outbound PostgreSQL Connector
menu: Postgres
connector:
  name: "infinyon/fluvio-connect-postgres-sink"
  link: "https://github.com/infinyon/fluvio-connectors/tree/main/rust-connectors/sinks/postgres"
---

The Outbound Postgres connector consumes Postgres events from a Fluvio topic.
These events are expected to be in [*Write-Ahead Logging (WAL)*](https://www.postgresql.org/docs/current/wal-intro.html) format

{{<idea>}}
This connector is intended to be used in combination with the [Inbound PostgreSQL connector]({{<ref "connectors/inbound/postgres.md">}}). The Inbound PostgreSQL connector will create WAL events and produce them to a Fluvio topic.
{{</idea>}}

{{<caution>}}
The Outbound Postgres connector does not currently support SSL
{{</caution>}}

## Common config values

%copy%
```yaml
type: postgres-sink
```

%copy%
```yaml
version: 0.2.0
```

## Secrets
### `FLUVIO_PG_DATABASE_URL`
*required*

### `url`
*required*

The login URL for your Postgres database.

This should contain
  your username, password, database hostname, and port.
  - Example: `postgres://user:password@hostname:port/database_name`
  
#### Example connector config 

{{<code file="code-blocks/yaml/connectors/outbound-examples/outbound-postgres.yaml" lang="yaml" copy=true >}}

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
In order to prevent any duplicate inserts or deletes, this connector will create a table in the configured Postgres database to keep track of its current Fluvio offset. 

Should the connector stop or unexpectedly restart, will resume from the last recorded offset. 