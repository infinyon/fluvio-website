---
title: Inbound PostgreSQL Connector
menu: Postgres
connector:
  name: "infinyon/fluvio-connect-postgres-source"
  link: "https://github.com/infinyon/fluvio-connectors/tree/main/rust-connectors/sources/postgres"
hidden: true
_build:
  render: never
---

The Inbound Postgres connector *reads Write-Ahead Logging (WAL) events* from a Postgres
database and produces them into a Fluvio Topic. It does not support typical sql queries.

The intentions of this connector is to support Change Data Capture (CDC) workflows.

In other words: Whenever data is inserted, updated, or deleted in the connected database,
this connector will capture an event that describes the change that took place, and emit a record
describing the event into a Fluvio Topic.

This connector uses [logical
replication](https://www.postgresql.org/docs/current/logical-replication.html) which
has some
[restrictions](https://www.postgresql.org/docs/current/logical-replication-restrictions.html).

{{<caution>}}
The Inbound Postgres connector does not currently support SSL
{{</caution>}}

{{<caution>}}
Existing tables and rows will not be copied over when the connector starts.<br>
We recomend using `pg_dump` on the Postgres DB connected the inbound connector uses
and `psql` on the Postgres DB the outbound connector inserts into.
{{</caution>}}

## Common config values

%copy%
```yaml
type: postgres-source
```

%copy%
```yaml
version: 0.2.0
```

## Parameters

The inbound Postgres connector supports the following configuration options:

### `publication`
*required*

The name of the PUBLICATION in the leader database that
  this connector should watch for CDC. A publication describes which
  tables will be included in the CDC stream.
  - Example: `fluvio`

### `slot`
*required*

The name of the logical replication slot that this connector
  should watch for CDC. The slot is used to keep track of how much of
  the replication stream has been read so far by the Fluvio connector.
  - Example: `fluvio`

### `topic`
*required*

The name of the Fluvio Topic that the connector should
  produce CDC events to.
  - Example: `postgres-topic`

### `skip_setup`
*optional*

If you'd like the connector to not to automatically create
    the `slot` and a `publication` in your postgres database.

Choices:
- `true`
- `false`

## Secrets

### `FLUVIO_PG_DATABASE_URL`
*required*

This should contain
  your username, password, database hostname, and port.
  - Example: `postgres://user:password@hostname:port/database_name`

#### Example connector config

{{<code file="embeds/connectors-old/inbound-examples/inbound-postgres.yaml" lang="yaml" copy=true >}}

## Data Events

The Fluvio Postgres connector emits events that closely represent
the format of Postgres [logical replication messages](https://www.postgresql.org/docs/10/protocol-logicalrep-message-formats.html), however, the Fluvio connector event messages are formatted in JSON.
There are nine different types of messages, each of which share some common
metadata, but then contain different inner contents.

The properties shared by all messages include the following:

- `wal_start`: The Log Sequence Number (LSN) of the replication slot _before_ this event
- `wal_end`: The LSN of the slot _after_ this event
- `timestamp`: The timestamp of the event, in microseconds since the "Postgres Epoch" (midnight on 2000-01-01)
- `message`: A JSON object containing the event-specific contents

The following are the nine types of messages, which may be identified by the `type`
field in the top-level `message` field:

- `begin`: Indicates the beginning of a transaction. All subsequent messages until a `commit`
  are part of this transaction.
- `commit`: Indicates the successful end of a transaction.
- `origin`: Denotes the name of the true upstream database. This may be relevant if the
  Postgres instance the Fluvio connector is communicating with is actually following a
  different Postgres leader. In that case, `origin` communicates the identity of the leader
  where the subsequent messages originated.
- `relation`: Whenever a data change occurs in a table that has not been seen by the connector
  before, Postgres sends a `relation` event that describes the ID, schema, table name, and column
  types of the table the data belongs to. The connector will remember the column layout from each
  relation event and use it to apply appropriate types to subsequent data messages (insert, update, delete).
- `type`: Communicates data types from the leader database.
- `insert`: Indicates that a new row has been inserted to a table. Includes the ID of the table
  as well as the contents of the new row.
- `update`: Indicates that an existing row has been updated in a table. Includes the ID of the table
  as well as the columns belonging to the row's key and the updated data in the row.
- `delete`: Indicates that an existing row has been deleted from a table. Includes the ID of the table
  and the columns belonging to the row's key that was deleted.
- `truncate`: Indicates that one or more tables has been truncated. Includes a list of table IDs
  that have been truncated.

Below are some samples of some of the different types of events:

### Begin

```json
{
  "wal_start": 24095704,
  "wal_end": 24095704,
  "timestamp": 689713875500731,
  "message": {
    "type": "begin",
    "final_lsn": 24096336,
    "timestamp": 689713834266075,
    "xid": 734
  }
}
```

### Commit

```json
{
  "wal_start": 24096440,
  "wal_end": 24096440,
  "timestamp": 689713875500960,
  "message": {
    "type": "commit",
    "flags": 0,
    "commit_lsn": 24096336,
    "end_lsn": 24096440,
    "timestamp": 689713834266075
  }
}
```

### Relation

```json
{
  "wal_start": 0,
  "wal_end": 0,
  "timestamp": 689716528859099,
  "message": {
    "type": "relation",
    "rel_id": 16385,
    "namespace": "public",
    "name": "dog",
    "replica_identity": "Default",
    "columns": [
      {
        "flags": 0,
        "name": "name",
        "type_id": 1043,
        "type_modifier": 24
      },
      {
        "flags": 0,
        "name": "species",
        "type_id": 1043,
        "type_modifier": 24
      },
      {
        "flags": 0,
        "name": "state",
        "type_id": 1043,
        "type_modifier": 24
      },
      {
        "flags": 0,
        "name": "sex",
        "type_id": 1042,
        "type_modifier": 5
      }
    ]
  }
}
```

### Insert

```json
{
  "wal_start": 24202800,
  "wal_end": 24202800,
  "timestamp": 689716528859294,
  "message": {
    "type": "insert",
    "rel_id": 16385,
    "tuple": [
      {
        "String": "Lucy"
      },
      {
        "String": "chihuahua"
      },
      {
        "String": "TX"
      },
      {
        "String": "f"
      }
    ]
  }
}
```

### Update

```json
{
  "wal_start": 24206136,
  "wal_end": 24206136,
  "timestamp": 689721572251776,
  "message": {
    "type": "update",
    "rel_id": 16385,
    "old_tuple": [
      {
        "String": "Wiggles"
      },
      {
        "String": "terrier"
      },
      {
        "String": "DC"
      },
      {
        "String": "f"
      }
    ],
    "key_tuple": null,
    "new_tuple": [
      {
        "String": "Wiggles"
      },
      {
        "String": "terrier"
      },
      {
        "String": "VA"
      },
      {
        "String": "f"
      }
    ]
  }
}
```

### Delete

```json
{
  "wal_start": 24205576,
  "wal_end": 24205576,
  "timestamp": 689721489186984,
  "message": {
    "type": "delete",
    "rel_id": 16385,
    "old_tuple": [
      {
        "String": "Piper"
      },
      {
        "String": "shih-tzu"
      },
      {
        "String": "NY"
      },
      {
        "String": "f"
      }
    ],
    "key_tuple": null
  }
}
```

