---
title: "This Week in Fluvio #22"
date: 2022-02-09
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}


## New Release - Fluvio v0.9.19

### Connector versions
Now when you run `fluvio connector list`, the version of the connector is returned

Example connector config:

%copy%
```yaml
# test-connector-config.yaml 
version: 0.1.1
name: my-test-connector
type: test-connector
topic: my-test-connector-topic
create_topic: true 
direction: source
```

```bash
$ fluvio connector create --config test-connector-config.yaml
$ fluvio connector list
 NAME               VERSION  STATUS 
 my-test-connector  0.1.1    Running 
```

### SmartModule debugging support using WASI

This is for advanced users who are willing to compile Fluvio locally. Please follow the [Fluvio Developer guide](https://github.com/infinyon/fluvio/blob/master/DEVELOPER.md) to get set up for local development.


SmartModule devs can now compile Fluvio with WASI support. This provides SmartModules access to `stdout` and `stderr` for debugging purposes.

%copy first-line%
```shell
$ git clone https://github.com/infinyon/fluvio.git
```

Build the development Fluvio cluster image with WASI support enabled

%copy first-line%
```shell
$ DEBUG_SMARTMODULE=true make build_k8_image
```

Build the development Fluvio CLI.<br>
(Optional, but for this example I also create an alias to the development Fluvio CLI `flvd` to minimize confusion)

%copy first-line%
```shell
$ make build-cli
$ alias flvd=$(pwd)/target/debug/fluvio
```

Start our development Fluvio cluster with WASI support

%copy first-line%
```shell
$ flvd cluster start --develop
```

Here's our example SmartModule. It is a slight modification of [our filter example](https://github.com/infinyon/fluvio/blob/master/crates/fluvio-smartmodule/examples/filter/src/lib.rs). For debugging purposes, we print the record to stdout before checking the contents of the record and applying filtering.

%copy%
```rust
use fluvio_smartmodule::{smartmodule, Record, Result};

#[smartmodule(filter)]
pub fn filter(record: &Record) -> Result<bool> {
    // Print every record to SPU logs
    println!("DEBUG: {record:#?}");
    let string = std::str::from_utf8(record.value.as_ref())?;
    Ok(string.contains('a'))
}
```

Before you build the SmartModule, you need to add the `wasm32-wasi` target with `rustup`.

%copy first-line%
```shell
$ rustup target add wasm32-wasi

Build SmartModule using the `wasm32-wasi` target to use it against our WASI-enabled cluster.  

%copy first-line%
```shell
$ cargo build --release --target wasm32-wasi
```

Load the WASI SmartModule into the cluster as `wasi-sm`

%copy first-line%
```shell
$ flvd smart-module create wasi-sm --wasm-file ./target/wasm32-wasi/release/fluvio_wasm_filter.wasm
```

Create the testing topic `twif-22`

%copy first-line%
```shell
$ flvd topic create twif-22
topic "twif-22" created
```

For our example producer input, we'll send 2 records (A positive and negative example)

%copy first-line%
```shell
$ flvd produce twif-22
> a
Ok!
> b
Ok!
```

In the consumer output using our WASI SmartModule, only the first record prints, which is the correct behavior.

%copy first-line%
```shell
$ flvd consume twif-22 --filter wasi-sm
Consuming records from the end of topic 'twif-22'. This will wait for new records
a
⠴
```

To view our SmartModule debug output, we look at the SPU pod logs in Kubernetes. At the bottom of the log we can verify that the contents of each record was printed.

%copy first-line%
```shell
$ kubectl logs -f fluvio-spg-main-0
[...]
2022-02-15T00:45:25.502747Z  INFO accept_incoming{self=FluvioApiServer("0.0.0.0:9005")}: fluvio_service::server: Received connection, spawning request handler
DEBUG: Record {
    preamble: RecordHeader {
        attributes: 0,
        timestamp_delta: 0,
        offset_delta: 0,
    },
    key: None,
    value: a,
    headers: 0,
}
DEBUG: Record {
    preamble: RecordHeader {
        attributes: 0,
        timestamp_delta: 0,
        offset_delta: 0,
    },
    key: None,
    value: b,
    headers: 0,
}
```

## Connectors

### Postgres Source + Sink

* [Docs for Postgres Source connector]({{<ref "/connectors/sources/postgres">}})
* [Docs for Postgres Sink connector]({{<ref "/connectors/sinks/postgres">}})



### HTTP

Our http connector has new options available to format its output.

We provide HTTP response information in a JSON format.

config
```yaml
parameters:
  output_type: json
  output_parts: body 
```

This output returns the body content as a single JSON string.

```json
{"body":"{\"fact\":\"The largest breed of cat is the Ragdoll with males weighing in at 1 5 to 20 lbs. The heaviest domestic cat on record was a neutered male tabby named Himmy from Queensland, Australia who weighed 46 lbs. 1 5 oz.\",\"length\":209}"}
```

If you need all of the header information, you can request the full response 

config
```yaml
parameters:
  output_type: json
  output_parts: full
```

```json
{"version":"HTTP/1.1","status_code":200,"status_string":"OK","headers":{"server":"nginx","transfer-encoding":"chunked","date":"Sat, 05 Feb 2022 12:15:01 GMT","x-content-type-options":"nosniff","access-control-allow-origin":"*","set-cookie":["XSRF-TOKEN=eyJpdiI6IlB5T1FVNXNDR3NvMlgrQzQ3TEJ4dGc9PSIsInZhbHVlIjoiKzRuckJrTU16SG9ycFk4L0w1QjYvdWQ1MDdJZGZZNURZSW9jQkN4RnlmMEcyMUo0TWVCSjE2SFJJblVrVWtTM05QOTE1VEdpOWlaTFlWMlFISEhSS0FRWThJMGNOaWpLOGFWTXVMRklWTzZ3dDJvSUxQTW5qeDdVNTYvV1M4ek8iLCJtYWMiOiJkOWE4NDQwZTIyOTdlZDAyMmU4OGQ5YTBkOWMzNjY1YmY0NDU5Y2RhOTZlNDg0NGI1YTdjMTE0ZTRkM2U2ZGJkIiwidGFnIjoiIn0%3D; expires=Sat, 05-Feb-2022 14:15:01 GMT; path=/; samesite=lax","cat_facts_session=eyJpdiI6IkNMWWxZLzFFL21wODdmanAyWjk0cFE9PSIsInZhbHVlIjoiNG1OTEZlKzhxVW9YaXBsbzl4TCtrVjJvQjBjWmdUYkg0VWtoVEgycVhGMWduUThNekNBQ0hxVFpCaG5PdHc3NnZsbWVhUHI2NU5Yem9mU1pxbk1CcDFLYUNIbkw1M2EzN1IrNTFlbDFkbG9YMjUyRUkrQnJ3OGR6NEdscVZpRnAiLCJtYWMiOiJlMWJlNTZmYzU1MmY2M2U1YjliMjEyMjFmZTcwM2FiOWI0MzEwM2M5YmM4ZDFiZjhkZTg5NDNmNGMwMmUzOTg5IiwidGFnIjoiIn0%3D; expires=Sat, 05-Feb-2022 14:15:01 GMT; path=/; httponly; samesite=lax"],"connection":"keep-alive","vary":"Accept-Encoding","cache-control":"no-cache, private","content-type":"application/json","x-ratelimit-limit":"100","x-xss-protection":"1; mode=block","x-frame-options":"SAMEORIGIN","x-ratelimit-remaining":"98"},"body":"{\"fact\":\"The first official cat show in the UK was organised at Crystal Palace in 1871.\",\"length\":78}"}
```


---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions