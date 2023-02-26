---
title: Connector template 
weight: 40
hidden: true
---

## Connector Template

This is an example connector template. Templates are written using `yaml` syntax.

{{<code file="embeds/templates/connector-template.yaml" lang="yaml" copy=true >}}

## Template description

### `name`
*required*

Provide a name for your connector

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

### `producer` (Common Inbound connector options)

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

### `transforms`
This is a list that configures the order of execution of SmartModules against your record data.

#### `uses`
This takes a single value, which is the name of your SmartModule

#### `with`
This takes multiple `key: value` pass to your SmartModule

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
