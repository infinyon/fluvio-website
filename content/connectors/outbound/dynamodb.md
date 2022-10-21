---
title: Outbound DynamoDB Connector
menu: DynamoDB
---

The Dynamodb Outbound Connector reads events from a
fluvio topic, deserializes them as json and inserts those key value pairs based
on the columns in the config.

## Connector config `parameters`

### `table-name`

The name of the [dynamodb table name]

### `column-names`

A comma separated list of the keys. **The first key is the partition key**

### `column-types`

A comma separated list of the [attribute types of the keys].
These are `N` for number, `S` for String, and `B` for Binary.

These keys are used for table creation. The incoming json is converted to the
[Dynamodb data types] on insert.


## Connector config `secrets`

### `AWS_REGION`

The [AWS Region]

### `AWS_ACCESS_KEY_ID`

The [AWS Access Key Id]

### `AWS_SECRET_ACCESS_KEY`

The [AWS Secret Access Key]

#### Example connector config 
%copy%

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
