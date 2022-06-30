---
title: Kafka
section: Sink
---

## Overview

The Kafka Connector is quite simple. It will send every record of a fluvio topic to a kafka cluster for a given kafka topic.

## Configuration Options

%copy%
```yaml
# kafka-sink-connector.yml
name: my-kafka-sink
type: kafka-sink
topic: fluvio-output-topic
parameters:
  kafka-url: "localhost:9092"
  kafka-topic: kafka-input-topic
  kafka-option:
    linger.ms: 10
    topic.metadata.refresh.fast.interval.ms: 10
    topic.metadata.refresh.fast.cnt: 10
```

* `kakfa-url` is required
* `kafka-topic` is optional and will default to the top level `topic` which is
the topic used on the fluvio side of things.
* `kafka-partition` is option and will default to `0` unless specified.
* `kafka-option` is an option that is specified as a dictonary. The [Configuration
properties are from the rdkafka
options](https://github.com/edenhill/librdkafka/blob/b171d8f411a981c7604a79777ce10245f05280dd/CONFIGURATION.md).

This configuration file is used together with the `fluvio connector create` command, like so:

%copy first-line%
```bash
$ fluvio connector create --config=./kafka-sink-connector.yml
```

## Data Events

Events are sent to fluvio as raw bytes. The record are sent along to fluvio as well.
