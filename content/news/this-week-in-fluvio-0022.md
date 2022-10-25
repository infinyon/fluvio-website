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
direction: source
```

%copy first-line%
```bash
$ fluvio connector create --config test-connector-config.yaml
```

%copy first-line%
```bash
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

%copy first-line%
```shell
$ make build-cli
```

Start our development Fluvio cluster with WASI support

%copy first-line%
```shell
$ ./target/debug/fluvio cluster start --develop
```

Here's our example SmartModule. It is a slight modification of [our filter example](https://github.com/infinyon/fluvio/blob/d63e3e2569e4d64a098e5c2189ac68e6e9cd2670/crates/fluvio-smartmodule/examples/filter/src/lib.rs). For debugging purposes, we print the record to stdout before checking the contents of the record and applying filtering.

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
```

Build SmartModule using the `wasm32-wasi` target to use it against our WASI-enabled cluster.  

%copy first-line%
```shell
$ cargo build --release --target wasm32-wasi
```

Load the WASI SmartModule into the cluster as `wasi-sm`

%copy first-line%
```shell
$ ./target/debug/fluvio smart-module create wasi-sm --wasm-file ./target/wasm32-wasi/release/fluvio_wasm_filter.wasm
```

Create the testing topic `twif-22`

%copy first-line%
```shell
$ ./target/debug/fluvio topic create twif-22
topic "twif-22" created
```

For our example producer input, we'll send 2 records to demonstrate the SmartModule output.

%copy first-line%
```shell
$ ./target/debug/fluvio produce twif-22
> a
Ok!
> b
Ok!
```

In the consumer output using our WASI SmartModule, only the first record prints, which is the correct behavior.

%copy first-line%
```shell
$ ./target/debug/fluvio consume twif-22 --filter wasi-sm
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

### Postgres

We will provide a more hands-on blog post in the future, but for now we'll summarize the release.

#### Postgres Source connector

The Fluvio source connector allows you to connect to an external Postgres database and implement [Change Data Capture (CDC)](https://en.wikipedia.org/wiki/Change_data_capture) patterns by recording all database updates into a Fluvio topic.


There is a little bit of [required configuration on the Postgres database side]({{<ref "/connectors/inbound/postgres#create-a-postgres-server-using-docker">}}), but the Postgres source connector config looks like this:

```yaml
# example-pg-source-connect.yml
version: 0.1.0
name: my-postgres-source
type: postgres-source
topic: postgres-topic
parameters:
  publication: fluvio
  slot: fluvio
secrets:
  FLUVIO_PG_DATABASE_URL: postgres://postgres:mysecretpassword@localhost:5432/postgres
```

#### Postgres Sink connector
The Postgres sink connector consumes the CDC event data from the Postgres source connector and runs the corresponding SQL against the sink connector's Postgres database.

The Postgres sink connector looks like this:

```yaml
# connect.yml
version: 0.1.0
name: my-postgres-sink
type: postgres-sink
topic: postgres-topic
parameters:
  url: postgres://postgres:mysecretpassword@localhost:5432/postgres
secrets:
  FLUVIO_PG_DATABASE_URL: postgres://postgres:mysecretpassword@localhost:5432/postgres
```

#### Postgres Connector Docs available now

A lot of work went into the release of our new Postgres connectors that we couldn't cover in depth here.

We encourage you to visit the docs, and expect a walkthrough using the Source and Sink connectors together in the future.

* [Docs for Postgres Source connector]({{<ref "/connectors/inbound/postgres">}})
* [Docs for Postgres Sink connector]({{<ref "/connectors/outbound/postgres">}})


### HTTP

Our HTTP source connector has new options available `output_type` and `output_parts` to format its output.

Example HTTP connector config

%copy%
```yaml
# connect.yml
version: 0.2.0
name: cat-facts
type: http
topic: cat-facts
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10
  output_parts: body # default
  output_type: text  # default
```

For example, our endpoint returns a JSON object in the body of the HTTP response.

%copy first-line%
```shell
$ fluvio consume cat-facts
Consuming records from the end of topic 'cat-facts'. This will wait for new records
{"fact":"In 1987 cats overtook dogs as the number one pet in America.","length":60}
```

If you want the full HTTP response, you can use `output_parts: full`

```diff
# connect.yml
[...]
- output_parts: body # default
+ output_parts: full
  output_type: text  # default
```


%copy first-line%
```shell
$ fluvio consume cat-facts
Consuming records from the end of topic 'cat-facts'. This will wait for new records
HTTP/1.1 200 OK
server: nginx
date: Wed, 16 Feb 2022 00:53:04 GMT
content-type: application/json
transfer-encoding: chunked
connection: keep-alive
vary: Accept-Encoding
cache-control: no-cache, private
x-ratelimit-limit: 100
x-ratelimit-remaining: 96
access-control-allow-origin: *
set-cookie: XSRF-TOKEN=REDACTED expires=Wed, 16-Feb-2022 02:53:04 GMT; path=/; samesite=lax
set-cookie: cat_facts_session=REDACTED expires=Wed, 16-Feb-2022 02:53:04 GMT; path=/; httponly; samesite=lax
x-frame-options: SAMEORIGIN
x-xss-protection: 1; mode=block
x-content-type-options: nosniff

{"fact":"Cats only use their meows to talk to humans, not each other. The only time they meow to communicate with other felines is when they are kittens to signal to their mother.","length":170}
```

If you plan to process the HTTP response details, it might be more useful to use `output_type: json`.

```diff
# connect.yml
[...]
  output_parts: full
- output_type: text  # default
+ output_type: json
```

```shell
$ fluvio consume cat-facts
Consuming records from the end of topic 'cat-facts'. This will wait for new records
{"status":{"version":"HTTP/1.1","code":200,"string":"OK"},"header":{"set-cookie":["XSRF-TOKEN=REDACTED expires=Wed, 16-Feb-2022 02:56:22 GMT; path=/; samesite=lax","cat_facts_session=REDACTED expires=Wed, 16-Feb-2022 02:56:22 GMT; path=/; httponly; samesite=lax"],"content-type":"application/json","x-frame-options":"SAMEORIGIN","x-content-type-options":"nosniff","x-xss-protection":"1; mode=block","vary":"Accept-Encoding","server":"nginx","x-ratelimit-remaining":"94","date":"Wed, 16 Feb 2022 00:56:22 GMT","transfer-encoding":"chunked","cache-control":"no-cache, private","x-ratelimit-limit":"100","access-control-allow-origin":"*","connection":"keep-alive"},"body":"{\"fact\":\"There are more than 500 million domestic cats in the world, with approximately 40 recognized breeds.\",\"length\":100}"}
```

[Updated docs for the HTTP Connector are available]({{<ref "/connectors/inbound/http">}})

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions