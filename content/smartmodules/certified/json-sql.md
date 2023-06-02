---
title: JSON to SQL Mapping
menu: Json-Sql
weight: 30
---

This is certified by InfinyOn [map-type]({{<ref "../transform/map.md" >}}) SmartModule that converts arbitrary JSON records into [SQL model](https://github.com/infinyon/fluvio-connectors/blob/main/rust-connectors/models/fluvio-model-sql), which is a self-describing representation of SQL INSERT statements. This SmartModule is intended to be used in [SQL Sink Connector][sql-sink-connector], to execute a command in a SQL database.

### Mapping
The mapping between incoming JSON records and the resulting SQL record is defined in the configuration of the SmartModule in the `mapping` init parameter. Let's look at the example:

%copy%
```yaml
- uses: infinyon/json-sql@0.1.0
  with:
    mapping:
      table: "target_table"
      map-columns:
        "device_id":
          json-key: "device.device_id"
          value:
            type: "int"
            required: true
        "device_type":
          json-key: "device.type"
          value:
            type: "text"
            default: "mobile"
        "record":
          json-key: "$"
          value:
            type: "jsonb"
            required: true
```

Here, we create insert statements to `target_table` database table. Each statement has three columns:
* **device_id** with `int` type.  The value for this column will looked-up at the following hierarchy `$.device.device_id` of the input record. If it is not present, the error will be thrown, as the mapping states, it is a required field.
* **device_type** - text field with `mobile` marked as default value. Not required. Will be taken from `$.device.type` hierarchy. 
* **record** - `jsonb` column that contains the whole input record.


With the given mapping, the **Json-Sql** SmartModule will convert the input:

```json
{
  "device": {
    "device_id": 1
  }
}
```

into the following output:

```json
{
  "Insert": {
    "table": "target_table",
    "values": [
      {
        "column": "record",
        "raw_value": "{\"device\":{\"device_id\":1}}",
        "type": "Json"
      },
      {
        "column": "device_type",
        "raw_value": "mobile",
        "type": "Text"
      },
      {
        "column": "device_id",
        "raw_value": "1",
        "type": "Int"
      }
    ]
  }
}
```

which is equivalent to the following SQL statement:

```sql
INSERT INTO 'target_table' (record, device_type, device_id) VALUES ('{"device":{"device_id":1}}', 'mobile', 1)
```

#### Data types
The list of supported types and corresponding types from [SQL model](https://github.com/infinyon/fluvio-connectors/blob/main/rust-connectors/models/fluvio-model-sql):

| Mapping                                     | Model           |
|---------------------------------------------|-----------------|
| int8, bigint                                | BigInt          |
| int4, int, integer                          | Int             |
| int2, smallint                              | SmallInt        |
| bool, boolean                               | Bool            |
| bytes, bytea                                | Bytes           |
| text                                        | Text            |
| float4, float, real                         | Float           |
| float8, "double precision", doubleprecision | DoublePrecision |
| decimal, numeric                            | Numeric         |
| date                                        | Date            |
| time                                        | Time            |
| timestamp                                   | Timestamp       |
| json, jsonb                                 | Json            |
| uuid                                        | Uuid            |


### Usage example
To see **Json-Sql** SmartModule in action, we will use [SMDK]({{<ref "../smdk/build-test.md" >}}) tool. 

First, we need to download it to our cluster from [SmartModule Hub]({{<ref "/smartmodules/hub/overview">}}):

%copy first-line%
```shell
$ fluvio hub download infinyon/json-sql@0.1.0
```

Second, we need to create a file `transform.yaml` with the mapping used before:

%copy%
```yaml
# transform.yaml
transforms:
- uses: infinyon/json-sql@0.1.0
  with:
    mapping:
      table: "target_table"
      map-columns:
        "device_id":
          json-key: "device.device_id"
          value:
            type: "int"
            required: true
        "device_type":
          json-key: "device.type"
          value:
            type: "text"
            default: "mobile"
        "record":
          json-key: "$"
          value:
            type: "jsonb"
            required: true

```

Now let's call `smdk test` to see the result:


%copy first-line%
```shell
$ smdk test --text '{"device":{"device_id":1}}' --transforms-file ./transforms.yaml
1 records outputed
{"Insert":{"table":"target_table","values":[{"column":"record","raw_value":"{\"device\":{\"device_id\":1}}","type":"Json"},{"column":"type","raw_value":"mobile","type":"Text"},{"column":"device_id","raw_value":"1","type":"Int"}]}}
```

As mentioned at the beginning of this page, the outputed records can be consumed by [SQL Sink Connector][sql-sink-connector] to be executed on SQL database. By convention, SQL Sink connector expects JSON inputs.

For additional examples checkout the tutorials:
* [Build HTTP to SQL Pipeline]({{<ref "../../docs/tutorials/data-pipeline.md" >}})
* [Build MQTT to SQL Pipeline]({{<ref "../../docs/tutorials/mqtt-to-sql.md" >}})

[sql-sink-connector]: {{<ref "../../connectors-old/outbound/sql.md" >}}
