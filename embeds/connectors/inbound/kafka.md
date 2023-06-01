# Fluvio Kafka Inbound Connector

This is a connector for taking data from a Kafka topic and sending to a Fluvio topic.

See [docs](https://www.fluvio.io/connectors/inbound/kafka/) here.

### Configuration

| Opt            | default               | type     | description                            |
| :---           | :---                  | :---     | :----                                  |
| url            | -                     | String   | The url for the kafka connector        |
| topic          | -                     | String   | The kafka topic                        |
| partition      | 0                     | Integer  | The kafka partition                    |
| group          | fluvio-kafka-source   | String   | The kafka consumer group               |

Example:
```yaml
apiVersion: 0.1.0
meta:
  version: 0.2.2
  name: my-kafka-connector
  type: kafka-source
  topic: kafka-topic
  create-topic: true
kafka:
  url: "localhost:9092"
  topic: fluvio-topic 
```

### Usage
To try out Kafka Source connector locally, you can use Fluvio CDK tool:
```bash
fluvio install cdk

cdk deploy -p kafka-source start --config crates/kafka-source/config-example.yaml
```

## Transformations
Fluvio Kafka Connectors support [Transformations](https://www.fluvio.io/docs/concepts/transformations-chain/).
