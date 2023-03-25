---
title: Inbound MQTT Connector
menu: MQTT
connector:
  name: "infinyon/fluvio-connect-mqtt-source"
  link: "https://github.com/infinyon/fluvio-connectors/tree/main/rust-connectors/sources/mqtt"
---
{{% inline-embed file="embeds/deprecation-notice/connectors-old.txt" %}}

MQTT is a publish/subscribe protocol that allows clients to listen to a stream
of events produced to a server.

It is widely used in real-time and IoT applications
since it is lightweight and easy to use.

## Common config values

%copy%
```yaml
type: mqtt-source
```

%copy%
```yaml
version: 0.5.3
```

## Parameters

The inbound MQTT connector supports the following configuration options:

### `mqtt-url`
*required*

The hostname of the MQTT server to subscribe to

This value can be provided as a parameter or as a secret

### `mqtt-topic`
*required*

The topic filter to use when receiving MQTT events

#### Example connector config

{{<code file="embeds/connectors/inbound-examples/inbound-mqtt.yaml" lang="yaml" copy=true >}}

## Data Events

There are two important pieces of information that we're interested in capturing from
every MQTT event.

1. The topic the event was sent to
2. The body of the event

In MQTT, topics are more of a label for particular message types. When we specify a
`mqtt-topic` to subscribe to, we are actually defining a "filter", or a pattern that
tells MQTT which events we are interested in receiving.

This means that events we receive
may not all belong to the same topic - rather, they all match the filter we provided.

The result of the event message body produced to Fluvio by the inbound MQTT connector is a JSON object containing:
* MQTT topic name
* MQTT message body

Furthermore, since MQTT allows events to contain arbitrary
binary data, the payload is encoded as a byte buffer.

Below is a sample of what an MQTT event captured and sent to Fluvio looks like:

```json
{"mqtt_topic":"/hfp/v2/journey/ongoing/vp/bus/0022/01151/2200/1/Espoon keskus/23:02/1160105/4/60;24/29/00/00","payload":[123,34,86,80,34,58,123,34,100,101,115,105,34,58,34,50,48,48,34,44,34,100,105,114,34,58,34,49,34,44,34,111,112,101,114,34,58,50,50,44,34,118,101,104,34,58,49,49,53,49,44,34,116,115,116,34,58,34,50,48,50,49,45,49,49,45,49,56,84,50,49,58,49,51,58,51,50,46,56,56,52,90,34,44,34,116,115,105,34,58,49,54,51,55,50,55,48,48,49,50,44,34,115,112,100,34,58,49,50,46,54,55,44,34,104,100,103,34,58,51,51,55,44,34,108,97,116,34,58,54,48,46,50,48,48,49,53,55,44,34,108,111,110,103,34,58,50,52,46,57,48,48,50,49,53,44,34,97,99,99,34,58,45,48,46,49,49,44,34,100,108,34,58,45,55,53,44,34,111,100,111,34,58,110,117,108,108,44,34,100,114,115,116,34,58,110,117,108,108,44,34,111,100,97,121,34,58,34,50,48,50,49,45,49,49,45,49,56,34,44,34,106,114,110,34,58,49,53,48,44,34,108,105,110,101,34,58,49,48,52,52,44,34,115,116,97,114,116,34,58,34,50,51,58,48,50,34,44,34,108,111,99,34,58,34,71,80,83,34,44,34,115,116,111,112,34,58,110,117,108,108,44,34,114,111,117,116,101,34,58,34,50,50,48,48,34,44,34,111,99,99,117,34,58,48,125,125]}
```
