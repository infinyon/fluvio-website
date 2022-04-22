---
title: Dynamodb
section: Sink
---

## Overview

The Dynamodb Sink Connector is a sink connector which reads events from a
fluvio topic, deserializes them as json and inserts those key value pairs based
on the columns in the config.

## Configuration Options

%copy%
```yaml
# dynamodb-connector.yml
name: dynamodb-connector
type: dynamodb-sink
topic: dynamodb-test
create_topic: true
parameters:
  table-name: dynamodb-test
  column-names: col_1,col_2,col_3
  column-types: N,N,S
secrets:
  AWS_REGION: "us-west-2"
  AWS_ACCESS_KEY_ID: ""
  AWS_SECRET_ACCESS_KEY: ""
```

### Parameters

* `table-name` - The name of the [dynamodb table name]
* `column-names` - A comma separated list of the keys. **The first key is the partition key**
* `column-types` - A comma separated list of the types for the keys.


### Secrets
* `AWS_REGION` - The [AWS Region]
* `AWS_ACCESS_KEY_ID` - The [AWS Access Key Id]
* `AWS_SECRET_ACCESS_KEY` - The [AWS Secret Access Key]

[AWS Secret Access Key]: https://docs.aws.amazon.com/general/latest/gr/aws-access-keys-best-practices.html
[AWS Access Key Id]: https://docs.aws.amazon.com/general/latest/gr/aws-access-keys-best-practices.html
[AWS Region]: https://aws.amazon.com/about-aws/global-infrastructure/regions_az/
[dynamodb table name]: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html

This configuration file is used together with the `fluvio connector create` command, like so:

%copy first-line%
```bash
$ fluvio connector create --config=./dynamodb-connector.yml
```

## Data Events

Input events are assumed to be JSON. If the data is not JSON deserializable,
the record is ignored.
