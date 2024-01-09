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
apiVersion: 0.1.0
meta:
  version: 0.3.3
  name: my-sql-connector
  type: sql-sink
  topic: sql-topic
  create-topic: true
  secrets:
    - name: DB_USERNAME
    - name: DB_PASSWORD
    - name: DB_HOST
    - name: DB_PORT
    - name: DB_NAME
sql:
  url: 'postgresql://${{ secrets.DB_USERNAME }}:${{ secrets.DB_PASSWORD }}@${{ secrets.DB_HOST }}:${{ secrets.DB_PORT }}/${{ secrets.DB_NAME }}'
```

### Secrets

The connector can use secrets in order to hide sensitive information.

```yaml
apiVersion: 0.1.0
meta:
  version: 0.3.3
  name: my-sql-connector
  type: sql-sink
  topic: sql-topic
  secrets:
    - name: DATABASE_URL
sql:
  url: ${{ secrets.DATABASE_URL }}
```
## Insert Usage Example
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
apiVersion: 0.1.0
meta:
  version: 0.3.3
  name: json-sql-connector
  type: sql-sink
  topic: sql-topic
  create-topic: true
  secrets:
    - name: DATABASE_URL
sql:
  url: ${{ secrets.DATABASE_URL }}
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
cdk deploy start --config connector-config.yaml
```

To delete the connector run:

```bash
cdk deploy shutdown --name json-sql-connector

```
After you run the connector you will see records in your database table.

See more in our [Build MQTT to SQL Pipeline](https://www.fluvio.io/docs/tutorials/mqtt-to-sql/) and [Build HTTP to SQL Pipeline](https://www.fluvio.io/docs/tutorials/data-pipeline/) tutorials.

## Upsert Usage Example

Every step would be same except the connector config and the behavior of the connector after deployment.

We have a `operation` parameter which defaults to `insert` but we can pass `upsert` here to specify we want to do an upsert operation.

Upsert additionaly takes an `unique-columns` argument. `unique-columns` specifies the list indices or column names to check for uniqueness of a record.
If a record with same value in `unique-columns` exists in the database, it will be updated. If no record exists with same value, the given record will
be inserted.

Connector configuration file for upsert (assuming `device_id` is a unique column or an index in the database):

```yaml
# connector-config.yaml
apiVersion: 0.1.0
meta:
  version: 0.3.3
  name: json-sql-connector
  type: sql-sink
  topic: sql-topic
  create-topic: true
  secrets:
    - name: DATABASE_URL
sql:
  url: ${{ secrets.DATABASE_URL }}
transforms:
  - uses: infinyon/json-sql
    with:
      mapping:
        operation: "upsert"
        unique-columns:
          - "device_id"
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

See more about upsert in [our blog](https://infinyon.com/blog/2023/07/sql-upsert/).
Note: the blog doesn't use `json-sql` smartmodule and has hardcoded records for demonstration. `sql-connector` is intended to be used with `json-sql`.
