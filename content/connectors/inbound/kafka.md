---
title: Inbound Kafka Connector
menu: Kafka
connector:
  name: "Source"
  link: "https://github.com/infinyon/kafka-connector"
---

The inbound Kafka Connector will send every record on a Kafka topic/partition to a Fluvio topic/partition.

{{<caution>}}
The Inbound Kafka connector does not currently support SSL
{{</caution>}}

This is a connector for taking data from a Kafka topic and sending to a Fluvio topic.

## Example connector config

{{<code file="embeds/connectors/inbound-examples/inbound-kafka.yaml" lang="yaml" copy=true >}}


## Configuration

| Opt            | default               | type     | description                            |
| :---           | :---                  | :---     | :----                                  |
| url            | -                     | String   | The url for the kafka connector        |
| topic          | -                     | String   | The kafka topic                        |
| partition      | 0                     | Integer  | The kafka partition                    |
| group          | fluvio-kafka-source   | String   | The kafka consumer group               |



## Transformations
Fluvio Kafka Connectors support [Transformations](https://www.fluvio.io/docs/concepts/transformations-chain/).