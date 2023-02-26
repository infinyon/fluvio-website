---
title: Inbound MQTT Connector
menu: MQTT
connector:
  name: "Source"
  link: "https://github.com/infinyon/mqtt-connector"
---

MQTT is a publish/subscribe protocol that allows clients to listen to a stream
of events produced to a server.

It is widely used in real-time and IoT applications
since it is lightweight and easy to use.

-> Supports MQTT V3.1.1 and V5 protocols

## Example config

{{<code file="embeds/connectors/inbound-examples/inbound-mqtt.yaml" lang="yaml" copy=true >}}

## Configuration
| Option              | default  | type           | description                                                                                                                                          |
|:--------------------|:---------|:---------      |:-----------------------------------------------------------------------------------------------------------------------------------------------------|
| timeout             | 60s      | Duration       | mqtt broker connect timeout in seconds and nanoseconds                                                                                               |
| url                 | -        | SecretString   | MQTT url which includes schema, domain, port and credentials such as username and password.                                                          |
| topic               | -        | String         | mqtt topic to subscribe and source events from                                                                                                       |
| client_id           | UUID V4  | String         | mqtt client ID                                                                                                                                       |
| payload_output_type | binary   | String         | controls how the output of `payload` field is produced                                                                                               |


### Transformations
Fluvio MQTT Source Connector supports [Transformations](https://www.fluvio.io/docs/concepts/transformations-chain/). Records can be modified before sending to Fluvio topic.

The previous example can be extended to add extra transformations to outgoing records:

{{<code file="embeds/connectors/inbound-examples/inbound-mqtt-transformation.yaml" lang="yaml" copy=true >}}

The object `device` in the resulting record will be "unwrapped" and the addition field `source` with value `mqtt-connector`
will be added.

Read more about [JSON to JSON transformations]({{<ref "/smartmodules/certified/jolt">}}).
