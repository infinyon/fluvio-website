---
title: MQTT Connector
menu: MQTT
---

## Overview

MQTT is a publish/subscribe protocol that allows clients to listen to a stream
of events produced to a server. It is widely used in real-time and IoT applications
since it is lightweight and easy to use.

## Configuration Options

The MQTT connector supports the following configuration options:

- `mqtt-url`: The hostname of the MQTT server to subscribe to
- `mqtt-topic`: The topic filter to use when receiving MQTT events

As well as the following Smart Connector options:

- `filter`: The name of a SmartModule to use as a filter
- `map`: The name of a SmartModule to use as a map
- `arraymap`: The name of a SmartModule to use as an arraymap

The MQTT connector may be deployed as a Local Connector or a Managed Connector.
See the following sections to learn how to declare the configuration options
for each mode.

### As a Local Connector

To deploy MQTT as a Local Connector, we execute it via the published Docker image
for the connector. We'll first need to make sure we have a Fluvio topic ready.

%copy first-line%
```bash
$ fluvio topic create mqtt
```

Then, we can execute the connector with the following command:

%copy%
```bash
docker run -d --name="my-mqtt" \
    -v"$HOME/.fluvio/config:/home/fluvio/.fluvio/config" \
    -t infinyon/fluvio-connect-mqtt \
    -- \
    --fluvio-topic=mqtt \
    --mqtt-url=mqtt.hsl.fi \
    --mqtt-topic=/hfp/v2/journey/#
```

Here, everything before the `--` is a docker argument, and everything after
`--` is an argument to the connector.

### As a Managed Connector

To deploy MQTT as a Managed Connector, we need to create a configuration file,
typically called `connect.yml`. The config file might look like the following:

%copy%
```yaml
# connect.yml
version: v1
name: my-mqtt
type: mqtt
topic: mqtt-topic
create_topic: true
direction: source
parameters:
  mqtt-url: "mqtt.hsl.fi"
  mqtt-topic: "/hfp/v2/journey/#"
```

The way we use this configuration is by passing it to the `fluvio connector` command,
like this:

%copy first-line%
```bash
$ fluvio connector create --config=./connect.yml
```

## Data Events

There are two important pieces of information that we're interested in capturing from
every MQTT event. They are:

- The topic the event was sent to, and
- The body of the event

In MQTT, topics are more of a label for particular message types. When we specify a
`mqtt-topic` to subscribe to, we are actually defining a "filter", or a pattern that
tells MQTT which events we are interested in receiving. This means that events we receive
may not all belong to the same topic - rather, they all match the filter we provided.

Because of this, the Fluvio MQTT Connector includes both the topic name _and_ the
message body when producing events to Fluvio. It packages these two pieces of information
as two fields in a JSON object. Furthermore, since MQTT allows events to contain arbitrary
binary data, the payload is encoded as a byte buffer.

Below is a sample of what an MQTT event captured and sent to Fluvio looks like:

```json
{"mqtt_topic":"/hfp/v2/journey/ongoing/vp/bus/0022/01151/2200/1/Espoon keskus/23:02/1160105/4/60;24/29/00/00","payload":[123,34,86,80,34,58,123,34,100,101,115,105,34,58,34,50,48,48,34,44,34,100,105,114,34,58,34,49,34,44,34,111,112,101,114,34,58,50,50,44,34,118,101,104,34,58,49,49,53,49,44,34,116,115,116,34,58,34,50,48,50,49,45,49,49,45,49,56,84,50,49,58,49,51,58,51,50,46,56,56,52,90,34,44,34,116,115,105,34,58,49,54,51,55,50,55,48,48,49,50,44,34,115,112,100,34,58,49,50,46,54,55,44,34,104,100,103,34,58,51,51,55,44,34,108,97,116,34,58,54,48,46,50,48,48,49,53,55,44,34,108,111,110,103,34,58,50,52,46,57,48,48,50,49,53,44,34,97,99,99,34,58,45,48,46,49,49,44,34,100,108,34,58,45,55,53,44,34,111,100,111,34,58,110,117,108,108,44,34,100,114,115,116,34,58,110,117,108,108,44,34,111,100,97,121,34,58,34,50,48,50,49,45,49,49,45,49,56,34,44,34,106,114,110,34,58,49,53,48,44,34,108,105,110,101,34,58,49,48,52,52,44,34,115,116,97,114,116,34,58,34,50,51,58,48,50,34,44,34,108,111,99,34,58,34,71,80,83,34,44,34,115,116,111,112,34,58,110,117,108,108,44,34,114,111,117,116,101,34,58,34,50,50,48,48,34,44,34,111,99,99,117,34,58,48,125,125]}
```
