---
title: Outbound Kafka Connector
menu: Kafka
connector:
  name: "infinyon/fluvio-connect-kafka-sink"
  link: "https://github.com/infinyon/fluvio-connectors/tree/main/rust-connectors/sinks/kafka"
---

The Kafka Connector is quite simple. It will send every record of a fluvio topic to a kafka cluster for a given kafka topic.

## Common config values

%copy%
```yaml
type: kafka-sink
```

%copy%
```yaml
version: 0.3.2
```

## Parameters

### `kakfa-url`
*required*

The url of the Kafka instance to connect to.

### `kafka-topic`
Default: Same name as the configured `fluvio` topic

The name of the Kafka topic name to connect to.


### `kafka-partition`
Default: `0`

The Kafka partition to connect to.

### `kafka-option`
*optional*

option that is specified as a dictonary.

See: The [Configuration
properties are from the rdkafka
options](https://github.com/edenhill/librdkafka/blob/b171d8f411a981c7604a79777ce10245f05280dd/CONFIGURATION.md).

### `security`

#### `security-protocol`
*optional*

The kafka security protocol.

Currently only supports `SSL`.

## Secrets

### `FLUVIO_KAFKA_CLIENT_KEY`
*optional*

The SSL key pem text.

### `FLUVIO_KAFKA_CLIENT_CERT`
*optional*

The SSL cert pem text

### `FLUVIO_KAFKA_CLIENT_CA`
*optional*

The SSL CA pem text

#### Example connector config

{{<code file="embeds/connectors/outbound-examples/outbound-kafka.yaml" lang="yaml" copy=true >}}


## Data Events

Events are sent to Fluvio as raw bytes. The event records are sent along to Fluvio as well.
