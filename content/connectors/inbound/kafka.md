---
title: Inbound Kafka Connector
menu: Kafka
connector:
  name: "infinyon/fluvio-connect-kafka-source"
  link: "https://github.com/infinyon/fluvio-connectors/tree/main/rust-connectors/sources/kafka"
---

The inbound Kafka Connector will send every record on a Kafka topic/partition to a Fluvio topic/partition.

{{<caution>}}
The Inbound Kafka connector does not currently support SSL
{{</caution>}}

## Common config values

%copy%
```yaml
type: kafka-source
```

%copy%
```yaml
version: 0.2.0
```

## Parameters

The inbound Kafka connector supports the following configuration options:

###  `kakfa-url`
*required*

The url of the Kafka instance to connect to.

### `kafka-topic`
Default: Same name as the configured `fluvio` topic

The name of the Kafka topic name to connect to.

### `kafka-partition`
Default: `0`

The Kafka partition to connect to.

### `kafka-group`
Default: `fluvio-kafka-source`

The Kafka group

#### Example connector config

{{<code file="embeds/connectors/inbound-examples/inbound-kafka.yaml" lang="yaml" copy=true >}}

## Data Events

Events are sent to Fluvio as raw bytes. The record are sent along to Fluvio as well.
