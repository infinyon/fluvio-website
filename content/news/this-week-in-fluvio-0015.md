---
title: "This Week in Fluvio #15"
date: 2021-11-17
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## New Release - Fluvio v0.9.13

This is a big release

### Apple M1 support

This is the first release with official Apple M1 support. If you have an Apple M1 machine and you'd like to try out Fluvio, please read our [Getting Started] page for MacOS.

[Getting Started]: {{< ref "/docs/get-started/mac.md">}}

### SmartModules

We previewed this feature [a couple weeks ago] and now it's here!

[a couple weeks ago]: {{< ref "/news/this-week-in-fluvio-0013.md#coming-soon-smartmodules">}}

All SmartModule types (filter, map, array-map, filter-map, aggregate, joins) can saved into your Fluvio cluster and you can use them just by referring to them by name.

Example creating of SmartModule: 
```shell
$ fluvio smartmodule create my-filter --wasm-file ./path/to/my-filter.wasm
```

Example usage: 
```shell
$ fluvio consume my-topic --filter <name>
$ fluvio consume my-topic --map <name>
$ fluvio consume my-topic --array-map <name>
$ fluvio consume my-topic --filter-map <name>
$ fluvio consume my-topic --aggregate <name>
$ fluvio consume my-topic --join <name>
```

You can still use SmartModules the original way, by providing a path to your wasm file. But if you're using the SmartModule a lot, we think persistent SmartModules will be more convenient to use.

### SmartConnectors 

This feature was teased [last week], but now it is ready to be tried out.

[last week]: {{< ref "/news/this-week-in-fluvio-0014.md">}}

Check out the new [Connector Developer guide] for more information about how to create your own connectors.

[Connector Developer guide]: {{< ref "/connectors/developer-guide/overview.md">}}

### FilterMap SmartModule 

The FilterMap SmartModule enables you to do filtering and reshaping your data at the same time.

#### Example FilterMap code

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

### Join SmartModule 

The Join SmartModule uses the stream you are consuming and the value at the end of another topic and allows you to return a new value 

#### Example Join code

In this example, we have 2 topics, `left-topic` and `right-topic`, and our example Join SmartModule.

``` rust
use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};

#[smartmodule(join)]
pub fn join(left_record: &Record, right_record: &Record) -> Result<(Option<RecordData>, RecordData)> {
    let left_value: i32 = std::str::from_utf8(left_record.value.as_ref())?.parse()?;
    let right_value: i32 = std::str::from_utf8(right_record.value.as_ref())?.parse()?;
    let value = left_value + right_value;

    Ok((None, value.to_string().into()))
}
```

Example input for topic `right-topic`
```bash
$ fluvio produce right-topic
> 2
Ok!
```

Now, when new records come in to `left-topic`, we will add the new record to the newest record at `right-topic`.
```bash
$ fluvio produce left-topic
> 7
Ok!
> 11
Ok!
> 19
Ok!
```

Example consume output
```bash
$ fluvio consume left-topic --join example-join --join-topic right-topic
Consuming records from the end of topic 'left-topic'. This will wait for new records
9
13
21
```

And if we change the top value on `right-topic`
```bash
$ fluvio produce right-topic
> -5
Ok!
```

Then produce new values to `left-topic`
```bash
$ fluvio produce left-topic
> 3
Ok!
> 10
Ok!
> 27 
Ok!
```

The resulting consume output will reflect the new values to `left-topic` adding itself to the new negative value at `right-topic`

```bash
$ fluvio consume left-topic --join example-join --join-topic right-topic
Consuming records from the end of topic 'left-topic'. This will wait for new records
9
13
21
-2
5
22
```

[Link to example code](https://github.com/infinyon/fluvio/blob/master/crates/fluvio-smartmodule/examples/join/src/lib.rs)

### Fullscreen Consumer table

This is the first version of an interactive table display for the Consumer. It expects the same json object input as `--output=table` but the output is a full screen scrollable table. The columns are alphabetized, and the first column serves as a primary key for updating the row. This offers the possibility of viewing live updating data

First you need to create a `tableformat` to define how you want your table to be displayed. This would be highly dependent on the shape of your data. For example purposes, we will be displaying event-sourced data. This is what our example data looks like:

```json
{"request_id":"123", "requester_name": "Alice", "state":"running", "run_time_minutes":"3"}
{"request_id":"456", "requester_name": "Alice", "state":"waiting", "run_time_minutes":"9"}
{"request_id":"789", "requester_name": "Bob", "state":"done", "run_time_minutes":"10"}
```

An example `tableformat`.

Here we only want to display the latest state of a request. We declare the `request_id` key as `primaryKey`, which means that any new events matching an existing `request_id` will update the row.

%copy%
```yaml
# tableformat-config.yaml
name: "current-requests"
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

Consuming from the topic using the `full-table` output, and our `tableformat`
```shell
$ fluvio consume request-events --output full-table --tableformat current-requests
```

Output:
```bash
┌('c' to clear table | 'q' or ESC to exit) | Items: 3──┐
│ID                Runtime           State             │
│123               3                 running           │
│456               9                 waiting           │
│789               10                done              │
└──────────────────────────────────────────────────────┘
```

Docs for this feature will be coming soon!

### A few bug fixes
A handful of user-facing issues were fixed

* Creating a connector that creates a topic will not fail if the topic already exists ([#1823](https://github.com/infinyon/fluvio/pull/1823))
* Ability to create Kubernetes-based clusters on MacOS was restored ([#1867](https://github.com/infinyon/fluvio/pull/1867))
* Aggregate SmartModule fixed to properly accumulate from previous values instead defaulting to the initial value. ([#1869](https://github.com/infinyon/fluvio/pull/1869)) (Additional special thanks to our Discord community for reporting this bug!) 


---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
