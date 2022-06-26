---
title: Postgres
section: Sink
---

The Postgres Sink Connector is a sink connector which reads events from a
fluvio topic and inserts them into a postgres database.

## Overview

The postgres sink connector consumes replication events that a postgres
source connector emits to a fluvio topic. In the near future, it will be able
to consume events produced by other database source connectors.


## Configuration Options

Fluvio Connectors may be launched as a "Managed connector" when running Fluvio
in Kubernetes, or as a "Local connector" which may connect to Fluvio anywhere.
When using Fluvio Postgres Sink as a Managed Connector, you'll need to provide a
configuration file that looks like the following:

%copy%
```yaml
# connect.yml
version: 0.2.0
name: my-postgres-sink
type: postgres-sink
topic: postgres-topic
parameters:
  url: postgres://postgres:mysecretpassword@localhost:5432/postgres
secrets:
  FLUVIO_PG_DATABASE_URL: postgres://postgres:mysecretpassword@localhost:5432/postgres
```

This configuration file is used together with the `fluvio connector create` command, like so:

%copy first-line%
```bash
$ fluvio connector create --config=./connect.yml
```

Below are descriptions of the purpose of each parameter:

- `url` (required): The login URL for your Postgres database. This should contain
  your username, password, database hostname, and port. This key can also be specified
 via the `FLUVIO_PG_DATABASE_URL` in the `secrets` sections like above.
- `topic` (required): The name of the Fluvio Topic that the connector should
  produce CDC events to.
  - Example: `postgres-topic`

## Data Events

The Postgres Sink Connector consumes [replication events from the Postgres
Source connector](/connectors/sources/postgresql/#data-events) and runs the
appropriate SQL for the corresponding postgres sink database.

## Connector Resume

Should the connector for some reason stop or crash, when starting up again, the
connector will start the fluvio consumer stream at the `current_offset` found
in `fluvio.offset` table in the postges server. This will prevent any duplicate
inserts or deletes.
