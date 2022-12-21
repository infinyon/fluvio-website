---
title: Outbound SQL Connector 
menu: SQL 
connector:
  name: "infinyon/fluvio-connect-sql-sink"
  link: "https://github.com/infinyon/fluvio-connectors/tree/main/rust-connectors/sinks/sql"
---

-> SQL Connector is a preview with support for SmartModule chaining through `transforms`.<br>See [HTTP-to-SQL]({{<ref "/docs/tutorials/data-pipeline.md" >}}) and [MQTT-to-SQL]({{<ref "/docs/tutorials/mqtt-to-sql.md">}}) for examples of this connector in action.

The Outbound SQL connector supports PostgreSQL and SQLite databases.

## Common config values

%copy%
```yaml
type: sql-sink
```

%copy%
```yaml
version: 0.1.1
```

## Parameters

### `database_url`
*required*

The connection string for a PostgreSQL or SQLite database

### Data types

This is the a table of what input types map to in PostgreSQL and SQLite

| Model           | PostgreSQL                   | SQLite       |                                          
|:----------------|:-----------------------------|:-------------|
| Bool            | BOOL                         | BOOLEAN      |
| Char            | CHAR                         | INTEGER      |
| SmallInt        | SMALLINT, SMALLSERIAL, INT2  | INTEGER      |
| Int             | INT, SERIAL, INT4            | INTEGER      |
| BigInt          | BIGINT, BIGSERIAL, INT8      | BIGINT, INT8 |
| Float           | REAL, FLOAT4                 | REAL         |
| DoublePrecision | DOUBLE PRECISION, FLOAT8     | REAL         |
| Text            | VARCHAR, CHAR(N), TEXT, NAME | TEXT         |
| Bytes           | BYTEA                        | BLOB         |
| Numeric         | NUMERIC                      | REAL         |
| Timestamp       | TIMESTAMP                    | DATETIME     |
| Date            | DATE                         | DATE         |
| Time            | TIME                         | TIME         |
| Uuid            | UUID                         | BLOB, TEXT   |
| Json            | JSON, JSONB                  | TEXT         |

## Transforms
*required*

### Uses
This is the name of a SmartModules from the SmartModule Hub.

`<group>/<SmartModuleName>@<version>`

### Invoke
This is the action 

Choices:
- `insert`
- `map`

### With 
These are parameters to the SmartModule

## Example

Transform with [Jolt SmartModule](https://github.com/infinyon/fluvio-connectors/blob/308ca0ec6e195210a86724ff8b0a32f6897c7b93/smartmodules/jolt/)
```yaml
  - uses: infinyon/jolt@0.1.0
    invoke: insert
    with:
      spec:
        - operation: shift
          spec:
            payload:
              device: 'device'
        - operation: default
          spec:
            device:
              type: 'mobile'
```

Transform with [json-sql SmartModule](https://github.com/infinyon/fluvio-connectors/blob/308ca0ec6e195210a86724ff8b0a32f6897c7b93/smartmodules/json-sql)
```yaml
  - uses: infinyon/json-sql@0.1.0
    invoke: insert
    with:
      mapping:
        table: 'topic_message'
        map-columns:
          'device_id':
            json-key: 'device.device_id'
            value:
              type: 'int'
              default: '0'
              required: true
          'record':
            json-key: '$'
            value:
              type: 'jsonb'
              required: true
```
