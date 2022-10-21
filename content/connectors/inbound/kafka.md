---
title: Kafka
---

The Kafka Connector is quite simple. It will send every record on a kafka
topic/partition to a fluvio topic/partition.

## Configuration Options

%copy%

{{<code file="code-blocks/yaml/connectors/inbound-examples/inbound-kafka.yaml" lang="yaml" copy=true >}}

###  `kakfa-url`
*required*

The url of the Kafka instance to connect to.

### `kafka-topic`

Defaults to the name of the `fluvio` topic

The name of the Kafka topic name to connect to.

### `kafka-partition`
Default: `0`

The Kafka partition to connect to.

## Data Events

Events are sent to fluvio as raw bytes. The record are sent along to fluvio as well.
