---
title: "This Week in Fluvio #11"
date: 2021-10-21
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## New Release - Fluvio v0.9.11

### Producer Auto-reconnect
Prior to this release, if a client encountered any network hiccups that caused a disconnection during a Producer session, the Producer would experience an error.

Now the Producer will try to reconnect automatically.

### SmartStreams w/ inputs
Using SmartStreams is a little more flexible now.

Previously, a SmartStream filter only supported logic with inputs that were hardcoded and compiled before using. This meant that we needed a separate filter per pattern we wanted to match on.

However we've added the capability to pass in user inputs at the time of execution. So we can have a single filter covering multiple patterns based on how we use it.

This is an example filter that takes in a parameter. Our named parameter is `key`, and the default value is the string value of the letter `a`. Keep following, as we see how this default influences the behavior of the filter.


```rust
use fluvio_smartstream::{smartstream, SmartOpt, Record, Result};

#[derive(SmartOpt)]
pub struct FilterOpt {
    key: String
}

impl Default for FilterOpt {
    fn default() -> Self {
        Self {
            key: "a".to_string()
        }
    }
}

#[smartstream(filter, params)]
pub fn filter(record: &Record, opt: &FilterOpt) -> Result<bool> {
    let string = std::str::from_utf8(record.value.as_ref())?;
    Ok(string.contains(&opt.key))
}
```

Continuing this example, we are producing a list of fruits to our topic `fruits`

Producer:

```shell
$ echo "grape" | fluvio produce fruits
$ echo "strawberry" | fluvio produce fruits
$ echo "plum" | fluvio produce fruits
$ echo "raspberry" | fluvio produce fruits
$ echo "mango" | fluvio produce fruits
```

Here, a Consumer listens to the `fruits` topic using the filter without providing a value for `key`. The expected behavior is to fallback to the default value of `key` we defined.

We see that this matches every fruit except for `plum`, because it doesn't have the letter `a` in it.

```shell
$ fluvio consume --filter fluvio_wasm_filter_with_parameters.wasm fruits
Consuming records from the end of topic 'fruits'. This will wait for new records
grape
strawberry
raspberry
mango
```

This is another Consumer also listening to the `fruits` topic. This time passing in a new value for `key` with the `--extra-params` (`-e` for short) to override the default of `a`.

Instead of fruits with `a`, we want any fruit with the word `berry`. We see that now the output only includes `strawberry` and `raspberry`, as expected.

```shell
$ fluvio consume --filter fluvio_wasm_filter_with_parameters.wasm fruits --extra-params key=berry
Consuming records from the end of topic 'fruits'. This will wait for new records
strawberry
raspberry
```

### SmartStreams ArrayMap 
This is a new type of SmartStream API that will make it easier to chunk up large datasets into smaller pieces. 

Here's an example that shows what an `array_map` SmartStream looks like:

```rust
use fluvio_smartstream::{smartstream, Record, RecordData, Result};

#[smartstream(array_map)]
pub fn array_map(record: &Record) -> Result<Vec<(Option<RecordData>, RecordData)>> {
    // Deserialize a JSON array with any kind of values inside
    let array: Vec<serde_json::Value> = serde_json::from_slice(record.value.as_ref())?;

    // Convert each JSON value from the array back into a JSON string
    let strings: Vec<String> = array
        .into_iter()
        .map(|value| serde_json::to_string(&value))
        .collect::<core::result::Result<_, _>>()?;

    // Create one record from each JSON string to send
    let records: Vec<(Option<RecordData>, RecordData)> = strings
        .into_iter()
        .map(|s| (None, RecordData::from(s)))
        .collect();
    Ok(records)
}
```

Let's say we have a topic `array-map-array`, and we produce data in the form of a JSON array

```shell
$ echo '["Apple", "Banana", "Cranberry"]' | fluvio produce array-map-array
```

If we have a consumer listening to this topic and with the ArrayMap SmartStream, we get output that looks like this:

```shell
$ fluvio consume array-map-array --array-map fluvio_wasm_array_map_array.wasm
Consuming records from the end of topic 'array-map-array'. This will wait for new records
"Apple"
"Banana"
"Cranberry"
```

Docs for ArrayMap are [available here]({{< ref "/smartmodules/transform/array-map" >}}). For a more hands-on explanation with a real-world example, please read our [blog post](https://infinyon.com/blog/2021/10/smartstream-array-map-reddit/) demonstrating the capabilities of `#[smartstream(array_map)]`.


---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
[connectors]: /connectors