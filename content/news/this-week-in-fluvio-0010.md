---
title: "This Week in Fluvio &#x23;10"
date: 2021-10-15
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

## No new release

We didn't have a new release this week. Instead the team met up in person for the first time.

## New managed connector
### MQTT
We have a new connector for the MQTT protocol available.

#### Example config

%copy%

```yaml
# config.yaml
version: v1
name: my-test-mqtt
type: mqtt
topic: my-mqtt
create_topic: true
direction: source
parameters:
  mqtt-url: "mqtt.hsl.fi" #Figure out another mqtt source that isn't this
  mqtt-topic: "/hfp/v2/journey/#"
  fluvio-topic: "my-mqtt"
secrets:
  foo: bar

```


%copy first-line%

```bash
$ fluvio cluster connector create --config config.yaml
```

%copy first-line%

```bash
$ fluvio cluster connector list

-------------
 NAME          STATUS 
 my-test-mqtt  Running
```

The test connector produces to a topic `my-mqtt`, where each record is an mqtt record corresponding to our configured mqtt topic.

%copy first-line%

```bash
$ fluvio consume my-mqtt --tail -d
Consuming records starting 10 from the end of topic 'my-mqtt'

#
# Paste real output here pls
#
```

To stop the connector, you need to delete it:

%copy first-line%

```bash
$ fluvio cluster connector delete my-test-mqtt
```

## Features coming soon

### Smart Modules

Smart Modules will be an alternative way to use the SmartStreams feature for both consumers and producers. You will be able to upload a Smart Module prior to instantiating a consumer or producer, and reference the Smart Module.

// How much more can I say about this? What's in the codebase at the moment? Is there a CLI?

### Table

Tables will enable material views with your structured JSON/YAML/TOML data. You will be able to select specific keys for display into a tabular format.

// More? I don't have much other than the original idea in the CLI

This feature is not yet ready to use, but you'll use the CLI.

%copy%
```shell
$ fluvio table
```



Communicate with us on [Github Discussions] or join [our Discord channel] and come say hello!

---

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
[connectors]: /connectors