---
title: "This Week in Fluvio #10"
date: 2021-10-15
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## No new release

We didn't have a new release this week.

Instead the team met up in person for the first time!

<img src="/news/images/0010/team-at-alcatraz.jpg" style="width:600px" />

> This is us trying to not look overheated after the audio tour at Alactraz.

## New managed connector
### MQTT
We have a new connector for the MQTT protocol available.

#### Example config

%copy%

```yaml
# config.yaml
api_version: v1
name: my-test-mqtt
type: mqtt
topic: public-mqtt
create_topic: true
direction: source
parameters:
  mqtt-url: "broker.hivemq.com"
  mqtt-topic: "testtopic/#"
  fluvio-topic: "public-mqtt"
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

The test connector produces to a topic `public-mqtt`, where each record is an mqtt record corresponding to our configured mqtt topic.

The `testtopic/fluvio` payload, for example, says "hello world"

%copy first-line%

```bash
$ fluvio consume public-mqtt --tail -d
Consuming records starting 10 from the end of topic 'public-mqtt'
{"mqtt_topic":"testtopic/fluvio","payload":[104,101,108,108,111,32,119,111,114,108,100]}
{"mqtt_topic":"testtopic/a_home/temp","payload":[50,54]}
{"mqtt_topic":"testtopic/a_home/menu/reg","payload":[49]}
{"mqtt_topic":"testtopic/a_home/menu/rele1","payload":[48]}
{"mqtt_topic":"testtopic/a_home/menu/rele2","payload":[48]}
{"mqtt_topic":"testtopic/a_home/menu/pwm1","payload":[51,48]}
{"mqtt_topic":"testtopic/a_home/menu/pwm2","payload":[51,48]}
{"mqtt_topic":"testtopic/a_home/menu/tc","payload":[49,48,48]}
{"mqtt_topic":"testtopic/a_home/menu/tzad","payload":[50,49,52,55,52,56,51,54,52,55]}
```

> In order to keep this connector generic, the payload is encoded as bytes.
> Rest assured that we'll provide more documentation for best practice in the future.

To stop the connector, you need to delete it:

%copy first-line%

```bash
$ fluvio cluster connector delete my-test-mqtt
```

## Features coming soon

### Table

Tables will enable materialized views with your structured JSON/YAML/TOML data. You will be able to select and format specific keys for display into a table.

This feature is not yet ready to use, but you may notice this command available CLI.

%copy%
```shell
$ fluvio table
```

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
[connectors]: /connectors