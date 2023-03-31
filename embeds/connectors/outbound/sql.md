# Fluvio SQL Sink connector
The SQL Sink connector reads records from Fluvio topic, applies configured transformations, and 
sends new records to the SQL database (via `INSERT` statements). 

## Supported databases
1. PostgreSQL
2. SQLite

### Data types
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

## Transformations
The SQL Sink connector expects the data in [Fluvio SQL Model](./crates/fluvio-model-sql/README.md) in JSON format.
In order to work with different data formats or data structures, `transformations` can be applied.
The transformation is a SmartModule pulled from the SmartModule Hub. Transformations are chained according to the order
in the config. If a SmartModule requires configuration, it is passed via `with` section of `transforms` entry. 

## Configuration
| Option       | default | type   | description                                           |
|:-------------|:--------| :---   |:------------------------------------------------------|
| url          |    -    | String | SQL database conection url                            |

### Basic example:
```yaml
meta:
  version: 0.1.0
  name: my-sql-connector
  type: sql-sink
  topic: sql-topic
  create-topic: true
sql:
  url: 'postgresql://USERNAME:PASSWORD@HOST:PORT/DB_NAME'
```

## Usage Example
Let's look at the example of the connector with one transformation named [infinyon/json-sql](https://github.com/infinyon/fluvio-connectors/blob/main/smartmodules/json-sql/README.md). The transformation takes
records in JSON format and creates SQL insert operation to `topic_message` table. The value from `device.device_id`
JSON field will be put to `device_id` column and the entire json body to `record` column.

The JSON record:
```json
{
  "device": {
    "device_id": 1
  }
}
```

The SQL database (Postgres):
```
CREATE TABLE topic_message (device_id int, record json);
```

Connector configuration file:
```yaml
# connector-config.yaml
meta:
  version: latest
  name: json-sql-connector
  type: sql-sink
  topic: sql-topic
  create-topic: true
sql:
  url: 'postgresql://USERNAME:PASSWORD@HOST:PORT/DB_NAME'
transforms:
  - uses: infinyon/json-sql
    with:
      mapping:
        table: "topic_message"
        map-columns:
          "device_id":
            json-key: "device.device_id"
            value:
              type: "int"
              default: "0"
              required: true
          "record":
            json-key: "$"
            value:
              type: "jsonb"
              required: true
```

You can use Fluvio `cdk` tool to deploy the connector:
```bash
fluvio install cdk
```
and then:
```bash
cdk deploy start --config connector-config.yaml
```
To delete the connector run:
```bash
cdk deploy shutdown --config connector-config.yaml

```
After you run the connector you will see records in your database table.

See more in our [Build MQTT to SQL Pipeline](https://www.fluvio.io/docs/tutorials/mqtt-to-sql/) and [Build HTTP to SQL Pipeline](https://www.fluvio.io/docs/tutorials/data-pipeline/) tutorials.
