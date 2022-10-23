---
title: Common connector config
weight: 40
---
## Common connector arguments

### `name`
*required*

A given name for your connector

### `type`
*required*

The kind of Inbound or Outbound connector

### `version`
*required*

This version corresponds to the type of Inbound of Outbound connector.
See the individual Inbound or Outbound connector page for valid version numbers.

### `topic`
*required*

This is the name of the topic that sends or recieves records

### `rust-log`
Default: `info`

This configures the logging for your connector

Choices:
- `off`
- `info`
- `warn`
- `error`
- `debug`
- `trace`


### `consumer` (Common Outbound connector options)

#### `partition`
Default: `0`

Select the partition for Outbound connector to watch

### `producer` Common Inbound connector options

#### `linger`
Default: `100ms`

The maximum time an Inbound connector spends collecting data before sending to topic.
Can be combined with `batch-size`

#### `compression`
Default: `none`

Choices:
- `none`
- `gzip`
- `snappy`
- `lz4`

#### `batch-size`
Default: `16KB`

The maximum size of the Inbound connector producer batch size before sending to topic.
Can be combined with `linger`

### Connector specific configuration options

See individual connector for more details about the available parameters or secrets


#### `parameters`

In this section is where the unique configuration per connector will be described

```yaml
parameters:
    example: true
    message: "hello"
```

#### `secrets`
Pass sensitive configuration details to your connector

```yaml
secrets:
    my-secret: secret-value
    A_SECRET: 12345
```

## Connector Template

{{<code file="code-blocks/yaml/connector-template.yaml" lang="yaml" copy=true >}}