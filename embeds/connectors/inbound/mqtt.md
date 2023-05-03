# Fluvio MQTT Connector
Official Infinyon MQTT connector

## Source Connector
Reads record from MQTT topic and writes to Fluvio topic.

Supports MQTT V3.1.1 and V5 protocols.


See [docs](https://www.fluvio.io/connectors/inbound/mqtt/) here.
Tutorial for [MQTT to SQL Pipeline](https://www.fluvio.io/docs/tutorials/mqtt-to-sql/).

### Configuration
| Option              | default  | type           | description                                                                                                                                          |
|:--------------------|:---------|:---------      |:-----------------------------------------------------------------------------------------------------------------------------------------------------|
| timeout             | 60s      | Duration       | mqtt broker connect timeout in seconds and nanoseconds                                                                                               |
| url                 | -        | SecretString   | MQTT url which includes schema, domain, port and credentials such as username and password.                                                          |
| topic               | -        | String         | mqtt topic to subscribe and source events from                                                                                                       |
| client_id           | UUID V4  | String         | mqtt client ID                                                                                                                                       |
| payload_output_type | binary   | String         | controls how the output of `payload` field is produced                                                                                               |

`url` option with type `SecretString` can be set as raw string value:
```yaml
url: "mqtt://test.mosquitto.org/"
```
or, as a reference to a secret with the given name:
```yaml
url:
  secret:
    name: "URL_SECRET_NAME"
```


#### Record Type Output

JSON Serialized string with fields `mqtt_topic` and `payload` 

#### Payload Output Type

| Value  | Output                       |
|:-------|:-----------------------------|
| binary | Array of bytes               |
| json   | UTF-8 JSON Serialized String |

### Usage Example

This is an example of connector config file:

```yaml
# config-example.yaml
apiVersion: 0.1.0
meta:
  version: 0.2.0
  name: my-mqtt-connector
  type: mqtt-source
  topic: mqtt-topic
  create-topic: true
mqtt:
  url: "mqtt://test.mosquitto.org/"
  topic: "mqtt-to-fluvio"
  client_id: "my_mqtt"
  timeout:
    secs: 30
    nanos: 0
  payload_output_type: json
```

Run connector locally using `cdk` tool (from root directory or any sub-directory):
```bash
fluvio install cdk

cdk deploy start --config config-example.yaml

cdk deploy list # to see the status
cdk deploy log my-mqtt-connector # to see connector's logs
```

Install MQTT Client such as
```bash
# for mac , this takes while....
brew install mosquitto
```

Insert records:
```bash
mosquitto_pub -h test.mosquitto.org -t mqtt-to-fluvio -m '{"device": {"device_id":1, "name":"device1"}}'
```

The produced record in Fluvio topic will be:
```json
{
  "mqtt_topic": "mqtt-to-fluvio",
  "payload": {
    "device": {
      "device_id": 1,
      "name": "device1"
    }
  }
}
```
### Transformations
Fluvio MQTT Source Connector supports [Transformations](https://www.fluvio.io/docs/concepts/transformations-chain/). Records can be modified before sending to Fluvio topic.

The previous example can be extended to add extra transformations to outgoing records:
```yaml
# config-example.yaml
apiVersion: 0.1.0
meta:
  version: 0.2.0
  name: my-mqtt-connector
  type: mqtt-source
  topic: mqtt-topic
  create-topic: true
mqtt:
  url: "mqtt://test.mosquitto.org/"
  topic: "mqtt-to-fluvio"
  client_id: "my_mqtt"
  timeout:
    secs: 30
    nanos: 0
  payload_output_type: json
transforms:
  - uses: infinyon/jolt@0.1.0
    with:
      spec:
        - operation: shift
          spec: 
            payload:
              device: "device"
        - operation: default
          spec:
            source: "mqtt-connector"   
```
The object `device` in the resulting record will be "unwrapped" and the addition field `source` with value `mqtt-connector`
will be added.

Read more about [JSON to JSON transformations](https://www.fluvio.io/smartmodules/certified/jolt/).


