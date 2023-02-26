---
title: Outbound Kafka Connector
menu: Kafka
connector:
  name: "Source"
  link: "https://github.com/infinyon/kafka-connector"
---

The Kafka Connector is quite simple. It will send every record of a fluvio topic to a kafka cluster for a given kafka topic

## Example connector config

{{<code file="embeds/connectors/outbound-examples/outbound-kafka.yaml" lang="yaml" copy=true >}}

## Example connector config with security

{{<code file="embeds/connectors/outbound-examples/outbound-kafka-ssl.yaml" lang="yaml" copy=true >}}


## Configuration

| Opt            | default               | type     | description                            |
| :---           | :---                  | :---     | :----                                  |
| url            | -                     | String   | The url for the kafka connector        |
| topic          | -                     | String   | The kafka topic                        |
| partition      | 0                     | Integer  | The kafka partition                    |
| create-topic   | false                 | Boolean  | Create or not a topic before start     |
| options        | -                     | Mapping  | The kafka client options               |
| security       | -                     | Mapping  | Optional. The kafka security config    |

## Security configuration
| Option               | default  | type     | description                            |
| :---                 | :---     | :---     | :----                                  |
| security_protocol    | ssl      | String   | The kafka security protocol            |
| ssl_key              | -        | Mapping  | The SSL key file to use                |
| ssl_cert             | -        | Mapping  | The SSL cert file to use               |
| ssl_ca               | -        | Mapping  | The SSL ca file to use                 |
