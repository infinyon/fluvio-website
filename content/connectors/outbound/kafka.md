---
title: Outbound Kafka Connector
menu: Kafka
connector:
  name: "infinyon/fluvio-connect-kafka-sink"
  link: "https://github.com/infinyon/fluvio-connectors/tree/main/rust-connectors/sinks/kafka"
---

The Kafka Connector is quite simple. It will send every record of a fluvio topic to a kafka cluster for a given kafka topic.

{{<caution>}}
The Outbound Kafka connector does not currently support SSL
{{</caution>}}

## Common config values

%copy%
```yaml
type: kafka-sink
```

%copy%
```yaml
version: 0.2.0
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

#### Example connector config

{{<code file="embeds/connectors/outbound-examples/outbound-kafka.yaml" lang="yaml" copy=true >}}


## Data Events

Events are sent to Fluvio as raw bytes. The record are sent along to Fluvio as well.
