---
title: "This Week in Fluvio #15"
date: 2021-11-17
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

## New Release

### Fluvio v0.9.13

This is a big release

#### SmartModules

We previewed this feature last week and now it's here

All SmartModule filter types, (filter, map, array-map, filter-map, aggregate, joins)

(Must confirm agg and join)

#### SmartConnectors 

This feature was teased [last week], but now it is ready to be tried out.

[last week]: {{< ref "/news/this-week-in-fluvio-0014.md">}}


#### FilterMap filters
Filtering and transforming at the same time.

##### Example FilterMap code

In this example, we take in integers. 

If those integers are positive, we want to divide it by two and return. Filter out inputs if they are odd.
```rust
use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};

#[smartmodule(filter_map)]
pub fn filter_map(record: &Record) -> Result<Option<(Option<RecordData>, RecordData)>> {
    let key = record.key.clone();
    let string = String::from_utf8_lossy(record.value.as_ref()).to_string();
    let int: i32 = string.parse()?;

    if int % 2 == 0 {
        let output = int / 2;
        Ok(Some((key.clone(), RecordData::from(output.to_string()))))
    } else {
        Ok(None)
    }
}
```
[Link to example code](https://github.com/infinyon/fluvio/blob/master/crates/fluvio-smartmodule/examples/filter_map/src/lib.rs)


Example input
```bash
$ fluvio produce filter-map-topic
> 19
Ok!
> 30
Ok!
> 29
Ok!
> -60
Ok!
> 23
Ok!
> 90
Ok!
> 17
Ok!
> ^C
```

Example output
```bash
$ fluvio consume filter-map-topic --filter-map divide-even-numbers
Consuming records from the end of topic 'filter-map-topic'. This will wait for new records
15
-30
45
```

For a deeper dive into FilterMap, check out our [blog post](https://www.infinyon.com/blog/2021/11/filter-map/) which covers a use-case.

#### Fullscreen Consumer table

This is the first version of an interactive table display for the Consumer. It expects the same json object input as `--output=table` but the output is a full screen scrollable table. The columns are alphabetized, and the first column serves as a primary key for updating the row. This offers the possibility of viewing live updating data

First you need to create a `tableformat` to define how you want your table to be displayed. This would be highly dependent on the shape of your data. For example purposes, we will be displaying event-sourced data. This is what our example data looks like:

```json
{"request_id":"123", "requester_name": "Alice", "state":"running", "run_time_minutes":"3"}
{"request_id":"456", "requester_name": "Alice", "state":"waiting", "run_time_minutes":"9"}
{"request_id":"789", "requester_name": "Bob", "state":"done", "run_time_minutes":"10"}
```

An example `tableformat`.

Here we only want to display the latest state of a request. We declare the `request_id` key as `primaryKey`, which means that any new events matching an existing `request_id` will update the row.
```yaml
# tableformat-config.yaml
name: "current_requests"
input_format: "JSON"
columns:
    - headerLabel: "ID"
      keyPath: "request_id"
      primaryKey: true
    - headerLabel: "Runtime"
      keyPath: "run_time_minutes"
    - headerLabel: "State"
      keyPath: "state"
```

Create the `tableformat`
```shell
$ fluvio tableformat create --config tableformat-config.yaml
```

```shell
$ fluvio consume request-events --output full_table --tableformat current_requests 
```

<Can I add a short video here showing updating rows>

#### A few bug fixes
A handful of user-facing issues were fixed

* Creating a connector that creates a topic will not fail if the topic already exists
* Ability to create Kubernetes-based clusters on MacOS was restored
* Aggregate SmartModule fixed to properly accumulate from previous values instead defaulting to the initial value. (Special thanks to our Discord community for reporting this bug!)

#### Preview: 

Joins?

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions