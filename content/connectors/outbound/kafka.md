---
title: Outbound Kafka Connector
menu: Kafka
---

The Kafka Connector is quite simple. It will send every record of a fluvio topic to a kafka cluster for a given kafka topic.

## Connector config `parameters`

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
%copy%

{{<code file="code-blocks/yaml/connectors/outbound-examples/outbound-kafka.yaml" lang="yaml" copy=true >}}


## Data Events

Events are sent to fluvio as raw bytes. The record are sent along to fluvio as well.