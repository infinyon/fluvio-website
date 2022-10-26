---
title: Filter
weight: 20
toc: true
---

The simplest type of SmartModule is a filter, which can examine each record in a stream and decide whether to accept or reject it. All accepted records are delivered down the pipeline, and rejected records are discarded. SmartModule applied in consumers or sink connectors filter records after they are stored in a topic, and will not impact persistence - it simply means that records filtered out are not delivered to the consumer. However, SmartModule filters applied to source connectors discard packets before they are stored in the topic and should be used with care. 

<img src="/smartmodules/images/smartmodule-filter.svg" alt="SmartModule Filter" justify="center" height="190">

##### Prerequisites

This section assumes that SMDK is [installed].


## Getting Practical: Filter Records by JSON fields

In this example, we're going to filter records based on the contents of their JSON fields. Since SmartModules are written using arbitrary Rust code, we can also pull in other crates as dependencies. 

### Create a SmartModule Project

Run `smdk generate` with the name of the filter and choose the  "filter" options:

%copy first-line%
```bash
$ smdk generate json-filter
project-group => 'john'
Generating new SmartModule project: json-filter
fluvio-smartmodule-cargo-dependency => '"0.2.5"'
🔧   Destination: ~/smdk/json-filter ...
🔧   Generating template ...
🤷   Please set a group name : john
✔ 🤷   Which type of SmartModule would you like? · filter
✔ 🤷   Will your SmartModule use init parameters? · false
Ignoring: /var/folders/5q/jwc86771549058kmbkbqjcdc0000gn/T/.tmpoM9gda/cargo-generate.toml
[1/5]   Done: Cargo.toml
[2/5]   Done: README.md
[3/5]   Done: SmartModule.toml
[4/5]   Done: src/lib.rs
[5/5]   Done: src
🔧   Moving generated files into: `~/smdk/json-filter`...
💡   Initializing a fresh Git repository
✨   Done! New project created ~/smdk/json-filter
```

With the SmartModule project created, let's talk about what we will be filtering.

### The Data: Server Logs

Suppose we have a web server that accepts HTTP requests from clients, does some
stuff, and then returns a response. It is common for such servers to have an
application logging system where they report various events taking place within the
server so that it may be monitored. We can imagine that this web server is exporting
logs to Fluvio via a producer, and that the logs are formatted as JSON describing
the event that occurred.

For the purposes of this exercise, let's say we have a file that we've stored our logs
into, so that we can manually produce them to a Fluvio topic and consume them back
using our JSON SmartModule. Create a file called `server.log` with the following
contents:

```bash
$ cat server.log
{"level":"info","message":"Server listening on 0.0.0.0:8000"}
{"level":"info","message":"Accepted incoming connection"}
{"level":"debug","message":"Deserializing request from client"}
{"level":"debug","message":"Client request deserialized"}
{"level":"debug","message":"Connecting to database"}
{"level":"warn","message":"Client dropped connnection"}
{"level":"info","message":"Accepted incoming connection"}
{"level":"debug","message":"Deserializing request from client"}
{"level":"debug","message":"Client request deserialized"}
{"level":"debug","message":"Connecting to database"}
{"level":"error","message":"Unable to connect to database"}
```

Each line in this file represents one event that occurred in our server. We can
see that each event is tagged with a "level" describing the significance of the
event, and a "message" with a description about what happened. This style of rating
logs with different levels is a common pattern in application logging, and we're
going to use it as the basis of our filter. Specifically, we're going to write a
filter that excludes all "debug" log, but accepts any "info", "warn", or "error"
logs. In a real-world scenario, this could dramatically help reduce the traffic
and noise in the logs if we were to consume these records into an analytics
platform for inspection.

### The Code: Writing our Filter

Let's look at the starter code that we got when we created our Filter template.

%copy first-line%
```bash
$ cd json-filter && cat ./src/lib.rs
```

```rust
use fluvio_smartmodule::{smartmodule, Result, Record};

#[smartmodule(filter)]
pub fn filter(record: &Record) -> Result<bool> {
    let string = std::str::from_utf8(record.value.as_ref())?;
    Ok(string.contains('a'))
}
```

The `Record` type contains the binary data for a single event in our topic. In our
case, this will be a UTF-8 encoded string that is also a valid JSON value. The
first step we'll need to take is to parse our Record as JSON so that we can
inspect it and determine what level the log is. We can use `serde`'s derive feature
to define types that represents our log data.

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error
}

#[derive(serde::Deserialize)]
struct StructuredLog {
    level: LogLevel,
    #[serde(rename = "message")]
    _message: String,
}
```

We're using `#[derive(serde::Deserialize)]` to implement Deserialize for our types,
which will allow us to convert our raw data into instances of `StructuredLog`.
We have also defined a `LogLevel` enum that implements `Deserialize` as well as
`Ord`, or "Ordering". When deriving `Ord` for an enum, the variants may be compared
to one another using `<` and `>`, where later-defined variants are "greater than"
earlier-defined variants. In other words, we have `LogLevel::Error > LogLevel::Debug`
and so on for each pair of LogLevels. Notice also that we have defined a field for
our logs' messages, but it is unused (which is why it is named `_message`). This
is because our filter will not care about the message in the log, just the level.
However, by including it in our `StructuredLog` definition, we can be sure that
all logs that we pass through the filter do indeed have a "message" field. In this
way, our filter is also acting as a sort of schema validator, only accepting records
that properly conform to the shape that we expect.

Now, let's write the logic for our filter. We'll start by parsing our raw data into
instances of `StructuredLog`.

```rust
use fluvio_smartmodule::{smartmodule, Record, Result};

#[smartmodule(filter)]
fn filter(record: &Record) -> Result<bool> {
    let log: StructuredLog = serde_json::from_slice(record.value.as_ref())?;
    
    todo!()
}
```

For `fluvio_smartmodule::Result`, we can bubble-up Results using the `?` operator.

Now for the final step, we want our filter to accept all records except for "debug" logs.
In other words, we actually want to keep the records that are "more important" or
"greater than" `LogLevel::Debug`. Since we have implemented `Ord` for `LogLevel`, this
will be a piece of cake! Let's look at all the code for the finished filter.

%copy%
```rust
use fluvio_smartmodule::{smartmodule, Record, Result};

#[derive(PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error
}

#[derive(serde::Deserialize)]
struct StructuredLog {
    level: LogLevel,
    #[serde(rename = "message")]
    _message: String,
}

#[smartmodule(filter)]
fn filter(record: &Record) -> Result<bool> {
    let log: StructuredLog = serde_json::from_slice(record.value.as_ref())?;

    // We keep records that are "greater than" debug
    Ok(log.level > LogLevel::Debug)
}
```

Let's make sure our code compiles. If eveything works as expected, there is a `.wasm` file generated in the target directory.

%copy first-line%
```bash
$ smdk build
...
Compiling json-filter v0.1.0 (~/smdk/json-filter)
Finished release-lto [optimized] target(s) in 12.78s
```

Your WASM binary is now ready for use.

### Test with SMDK

Now that we've written our filter, let's test using the command line. 

Test `info` log:

%copy first-line%
```bash
$ smdk test --text='{"level":"info","message":"Server listening on 0.0.0.0:8000"}'
loading module at: /Users/aj/local/projects/smartmodule/smdk/json-filter/target/wasm32-unknown-unknown/release-lto/json_filter.wasm
1 records outputed
{"level":"info","message":"Server listening on 0.0.0.0:8000"}
```

Test `debug` log:

%copy first-line%
```bash
$ smdk test --text='{"level":"debug","message":"Deserializing request from client"}'
loading module at: /Users/aj/local/projects/smartmodule/smdk/json-filter/target/wasm32-unknown-unknown/release-lto/json_filter.wasm
0 records outputed
```

Good news! :tada: it works as expected!


### Test on Cluster

Let's create a new topic to produce our source data:

%copy first-line%
```bash
$ fluvio topic create server-logs
topic "server-logs" created
```

Load `server.log` file to `server-logs` topic:

%copy first-line%
```bash
$ fluvio produce server-logs -f server.log
```

Let's double check it's all there.

%copy first-line%
```bash
$ fluvio consume server-logs -B -d
{"level":"info","message":"Server listening on 0.0.0.0:8000"}
{"level":"info","message":"Accepted incoming connection"}
{"level":"debug","message":"Deserializing request from client"}
{"level":"debug","message":"Client request deserialized"}
{"level":"debug","message":"Connecting to database"}
{"level":"warn","message":"Client dropped connnection"}
{"level":"info","message":"Accepted incoming connection"}
{"level":"debug","message":"Deserializing request from client"}
{"level":"debug","message":"Client request deserialized"}
{"level":"debug","message":"Connecting to database"}
{"level":"error","message":"Unable to connect to database"}
```

#### Load SmartModule to Fluvio

The SmartModule can be loaded to local Fluvio Cluster or [InfinyOn Cloud], as determined by the [`current profile`]. In this example, the profile points to InfinyOn Cloud.

%copy first-line%
```bash
$ smdk load
Loading package at: ~/smdk/json-filter
Found SmartModule package: json-filter
loading module at: ~/smdk/json-filter/target/wasm32-unknown-unknown/release-lto/json_filter.wasm
Trying connection to fluvio router.infinyon.cloud:9003
Creating SmartModule: json-filter
```

Rust `fluvio smartmodule list` to ensure your SmartModule has been uploaded:

%copy first-line%
```bash
$ fluvio smartmodule list
  SMARTMODULE                   SIZE     
  john/json-filter@0.1.0        140.1 KB 
```

SmartModule that have been uploaded on the cluster can be used by other areas of the system (consumers, producers, connectors, etc):

%copy first-line%
```bash
$ fluvio consume server-logs -dB --smartmodule=john/json-filter@0.1.0
Consuming records from the beginning of topic 'server-logs'
{"level":"info","message":"Server listening on 0.0.0.0:8000"}
{"level":"info","message":"Accepted incoming connection"}
{"level":"warn","message":"Client dropped connnection"}
{"level":"info","message":"Accepted incoming connection"}
{"level":"error","message":"Unable to connect to database"}
```

Congratulations! :tada: Eveything worked as expected!


## Publish to SmartModule Hub

It turns out this SmartModule was requested by other data streaming teams in the organization, so we've decided to [publish] it on [SmartMoudle Hub].

%copy first-line%
```bash
$ smdk publish
Creating package john/json-filter@0.1.0
.. fill out info in hub/package-meta.yaml
Package hub/json-filter-0.1.0.ipkg created
Package uploaded!
```

Let's double check that the SmartModule is available for download:

%copy first-line%
```bash
$ fluvio hub list
  SMARTMODULE                    
  john/json-filter@0.1.0         
```

Congratulations! :tada: Your SmartModule is now available for download in the SmartModule Hub.

## Read next

- [Explore filter use-cases](https://www.infinyon.com/blog/2021/06/smartstream-filters/)
- [Writing a map to transform records]({{< ref "/smartmodules/types/map" >}})
- [Writing an aggregate to sum numbers]({{< ref "/smartmodules/types/aggregate" >}})


[installed]: {{< ref "smartmodules/smdk/install" >}}
[publish]: {{< ref "smartmodules/smdk/publish" >}}
[InfinyOn Cloud]: https://infinyon.cloud
[`current profile`]: {{< ref "cli/client/profile" >}}
[SmartMoudle Hub]: {{< ref "smartmodules/hub/overview" >}}
