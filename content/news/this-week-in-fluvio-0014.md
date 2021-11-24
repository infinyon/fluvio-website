---
title: "This Week in Fluvio #14"
date: 2021-11-10
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

## No new release

We didn't have a new release this week.

### Coming soon: SmartConnectors

Last week we teased SmartModules. One of the things we've been able to do with SmartModules is integrate them with our Connectors. We're calling our data Connectors + SmartModules: **SmartConnectors**.

Using SmartConnectors is easy if you're already using SmartModules. You only need to add a reference to the SmartModule in your source or sink Connector config.

Let's look at an example. We're going to look at a SmartConnector from the perspective of the config

```yaml
# config.yaml
version: v1
name: my-test-mqtt
type: mqtt
topic: public-mqtt
create_topic: true
direction: source
parameters:
  mqtt-url: "broker.hivemq.com"
  mqtt-topic: "testtopic/#"
  map: "example-parse-mqtt-map"
```

In this example, we're using the `my-test-mqtt` connector we introduced in [a previous TWiF] to get a live bytestream from an MQTT broker and store it in a topic. But before we store it, we want to parse and transform the raw bytestream into our own types with a SmartModule.

[a previous TWiF]: {{< ref "/news/this-week-in-fluvio-0010.md#new-managed-connector">}}

Here's the SmartModule Map we're going to use to transform the connector output.

```rust
use fluvio_smartmodule::{smartmodule, Result, Record, RecordData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MqttEvent {
    mqtt_topic: String,
    payload: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MyMessage {
    key1: String,
    key2: String,
}

#[smartmodule(map)]
pub fn map(record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    // Parse the MQTT event
    let mqtt_event: MqttEvent = serde_json::from_slice(record.value.as_ref())?;

    // Try to parse the payload bytes 
    // If we can, return it
    let (value, value_record) = if let Ok(my_message) = serde_json::from_slice::<MyMessage>(&mqtt_event.payload) {
        // Return the parsed payload
        let parsed_value = serde_json::to_vec(&my_message)?;
        let parsed_record = RecordData::from(parsed_value);

        (record.key.clone(), parsed_record)
    } else {
        // otherwise return the default value of our type 
        let default_value = serde_json::to_vec(&MyMessage::default())?;
        let default_record = RecordData::from(default_value);

        (record.key.clone(), default_record)
    };

    Ok((value, value_record))
}
```


Compile and load the SmartModule WASM module
```shell
$ cargo build --release --target wasm32-unknown-unknown
```

Then create the SmartModule like this. Our connector is expecting a SmartModule map named `example-parse-mqtt-map`
```shell
$ fluvio smartmodule create example-parse-mqtt-map --wasm-file ./target/wasm32-unknown-unknown/release/module.wasm
```

We just need to make sure the SmartModule we're referencing has been created. Let's check with `fluvio smartmodule list`.

```shell
$ fluvio smartmodule list
 NAME                    STATUS             SIZE 
 example-parse-mqtt-map  SmartModuleStatus  164590
```

Now we'll create our connector the usual way

```shell
$ fluvio connector create --config ./path/to/config.yaml
$ fluvio connector list
 NAME          STATUS 
 my-test-mqtt  Running
```

So lastly, we'll look at the data in the topic. Just for comparison, I'll show data that would have been produced both without the SmartModule and with the SmartModule.

Example topic data without using our SmartModule
```json
{"mqtt_topic":"testtopic/a_home/temp","payload":[51,56]}
{"mqtt_topic":"testtopic/a_home/menu/reg","payload":[50]}
{"mqtt_topic":"testtopic/a_home/menu/rele1","payload":[49]}
{"mqtt_topic":"testtopic/a_home/menu/rele2","payload":[49]}
{"mqtt_topic":"testtopic/a_home/menu/pwm1","payload":[51,48]}
{"mqtt_topic":"testtopic/a_home/menu/pwm2","payload":[51,48]}
{"mqtt_topic":"testtopic/a_home/menu/tc","payload":[49,53]}
{"mqtt_topic":"testtopic/a_home/menu/tzad","payload":[52,53]}
{"mqtt_topic":"testtopic/fluvio","payload":[123,34,107,101,121,49,34,58,34,83,109,97,114,116,67,111,110,110,101,99,116,111,114,32,69,120,97,109,112,108,101,34,44,34,107,101,121,50,34,58,34,72,101,108,108,111,32,119,111,114,108,100,33,34,125]}
```

Example topic data using our SmartModule
```json
{"key1":"","key2":""}
{"key1":"","key2":""}
{"key1":"","key2":""}
{"key1":"","key2":""}
{"key1":"","key2":""}
{"key1":"","key2":""}
{"key1":"","key2":""}
{"key1":"","key2":""}
{"key1":"SmartConnector Example","key2":"Hello world!"}
```

And we see clear differences. With the inclusion of the SmartModule, we've transformed our external data to our needs.

SmartConnectors are coming soon, so keep an eye out for a future release.

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions