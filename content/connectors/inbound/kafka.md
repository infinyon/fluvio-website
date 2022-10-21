---
title: Kafka
---

## Overview

The Kafka Connector is quite simple. It will send every record on a kafka
topic/partition to a fluvio topic/partition.

## Configuration Options

%copy%
```yaml
# kafka-source-connector.yml
name: my-kafka-source
type: kafka-source
topic: fluvio-output-topic
parameters:
  kafka-url: "localhost:9092" # or something
  kafka-topic: kafka-input-topic
```

* `kakfa-url` is required
* `kafka-topic` is optional and will default to the top level `topic` which is the topic used on the fluvio side of things.
* `kafka-partition` is option and will default to `0` unless specified.


This configuration file is used together with the `fluvio connector create` command, like so:

%copy first-line%
```bash
$ fluvio connector create --config=./kafka-source-connector.yml
```

## Data Events

Events are sent to fluvio as raw bytes. The record are sent along to fluvio as well.
