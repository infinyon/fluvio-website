---
title: Outbound DynamoDB Connector
menu: DynamoDB
connector:
  name: "infinyon/fluvio-connect-dynamodb-sink"
  link: "https://github.com/infinyon/fluvio-connectors/tree/main/rust-connectors/sinks/dynamodb"
---

The DynamoDB Outbound Connector reads events from a Fluvio topic, then deserializes as json and inserts those key value pairs based on the columns in the config.

## Common config values

%copy%
```yaml
type: dynamodb-sink
```

```yaml
version: 0.2.0
```

## Parameters

### `table-name`
*required*

The name of the [dynamodb table name]

### `column-names`
*required*

A comma separated list of the keys. **The first key is the partition key**

### `column-types`
*required*

A comma separated list of the options representing [attribute types of the keys].

These keys are used for table creation. The incoming json is converted to the [Dynamodb data types] on insert.

| Option | Attribute Type |
|:------:|:--------------:|
|    N   |     Number     |
|    S   |     String     |
|    B   |     Binary     |

## Secrets

### `AWS_REGION`
*required*

The [AWS Region]

### `AWS_ACCESS_KEY_ID`
*required*

The [AWS Access Key Id]

### `AWS_SECRET_ACCESS_KEY`
*required*

The [AWS Secret Access Key]

#### Example connector config

{{<code file="code-blocks/yaml/connectors/outbound-examples/outbound-dynamodb.yaml" lang="yaml" copy=true >}}

[Dynamodb data types]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DynamoDBMapper.DataTypes.html
[attribute types of the keys]: https://docs.aws.amazon.com/cli/latest/reference/dynamodb/create-table.html#options
[AWS Secret Access Key]: https://docs.aws.amazon.com/general/latest/gr/aws-access-keys-best-practices.html
[AWS Access Key Id]: https://docs.aws.amazon.com/general/latest/gr/aws-access-keys-best-practices.html
[AWS Region]: https://aws.amazon.com/about-aws/global-infrastructure/regions_az/
[dynamodb table name]: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html

## Data Events

{{<caution>}}
Input events are assumed to be JSON. If the data is not JSON deserializable,
the record is ignored.
{{</caution>}}
