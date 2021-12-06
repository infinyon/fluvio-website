---
title: Filter
weight: 20
toc: false
---

The simplest type of SmartModule is a filter, which can examine each record in a stream and decide whether to accept or reject it. All accepted records are delivered down the pipeline, and rejected records are discarded. SmartModule applied in consumers or sink connectors filter records after they are stored in a topic, and will not impact persistence - it simply means that records filtered out are not delivered to the consumer. However, SmartModule filters applied to source connectors discard packets before they are stored in the topic and should be used with care. 

<img src="/docs/smartmodules/images/smartmodule-filter.svg" alt="SmartModule Filter" justify="center" height="200">

### Getting Practical: Filter Records by JSON fields

In this example, we're going to filter records based on the contents of their JSON fields. Since SmartModules are written using arbitrary Rust code, we can also pull in other crates as dependencies. If you want to jump ahead and see the finished code, [check out our JSON filter example].

[check out our JSON filter example]: https://github.com/infinyon/fluvio/tree/master/crates/fluvio-smartmodule/examples/filter_json

#### Create a new Project

Let's use `cargo-generate` again to set up our new SmartModule project. We'll want
to give the project a name and choose the "filter" option.

%copy first-line%
```bash
$ cargo install cargo-generate # In case you didn't install it before
```

%copy first-line%
```bash
$ cargo generate --git https://github.com/infinyon/fluvio-smartmodule-template
🤷   Project Name : json-filter
🔧   Creating project called `json-filter`...
✔ 🤷   Which type of SmartModule would you like? · filter
[1/5]   Done: .cargo/config.toml
[2/5]   Done: .gitignore
[3/5]   Done: Cargo.toml
[4/5]   Done: README.md
[5/5]   Done: src/lib.rs
✨   Done! New project created json-filter
```

Alright, now that we have a setup, let's talk about what we're going to
be filtering.

#### The Data: Server Logs

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

#### The Code: Writing our Filter

Let's look at the starter code that we got when we created our Filter template.

%copy first-line%
```bash
cd json-filter && cat ./src/lib.rs
```

```rust
// src/lib.rs
use fluvio_smartmodule::{smartmodule, Record, Result};

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

Let's make sure our code compiles. We'll use release mode in order to get
the smallest and fastest binary possible. We should be able to see the
`.wasm` file appear in the target directory.

%copy first-line%
```bash
$ cargo build --release
   Compiling json-filter v0.1.0 (/home/user/json-filter)
    Finished release [optimized] target(s) in 2.33s
```

Your WASM binary is now ready for use.

-> If this is your first time building WASM, you may need to run **% rustup target add wasm32-unknown-unknown**
 
%copy first-line%
```bash    
$ ls -la target/wasm32-unknown-unknown/release
.rwxr-xr-x  135Ki user 19 May 13:29   json_filter.wasm
```

#### Test Drive: Producing and Consuming the Data

Now that we've written our filter, let's play with some data and make sure we
get the results we expect! We'll start by creating a new topic where we'll
produce our data.

%copy first-line%
```bash
$ fluvio topic create server-logs
topic "server-logs" created
```

In order to see the impact of our SmartModule filter, let's open two terminals,
with each running a consumer that watches our `server-logs` topic. One of these
will be a plain consumer that consumes _all_ the records, and the other one will
use our filter, so we should only see non-debug logs.

To run the plain consumer, use the following command:

%copy first-line%
```bash
$ fluvio consume server-logs -B
```

In the other terminal, run a consumer with the SmartModule filter using this command:

%copy first-line%
```bash
$ fluvio consume server-logs -B --filter="target/wasm32-unknown-unknown/release/json_filter.wasm"
```

Finally, we can take our `server.log` file and use `fluvio produce` to send each
line of the file as one record to our topic. In a third terminal, run the following
command to produce the server logs to our topic:

%copy first-line%
```bash
$ fluvio produce server-logs -f server.log
```

In the plain consumer, we should see all the records get passed through:

%copy first-line%
```bash
$ fluvio consume server-logs -B
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

But in the consumer with our SmartModule, we'll no longer see any of the records
whose log level was debug!

%copy first-line%
```bash
$ fluvio consume server-logs -B --filter="target/wasm32-unknown-unknown/release/json_filter.wasm"
{"level":"info","message":"Server listening on 0.0.0.0:8000"}
{"level":"info","message":"Accepted incoming connection"}
{"level":"warn","message":"Client dropped connnection"}
{"level":"info","message":"Accepted incoming connection"}
{"level":"error","message":"Unable to connect to database"}
```

### Register the SmartModule with Fluvio

After building a SmartModule as a WASM binary, it may be registered with Fluvio using the `fluvio smart-module` command:

%copy first-line%
```bash
$ fluvio smart-module create log-filter --wasm-file target/wasm32-unknown-unknown/release/json_filter.wasm
```

Use `fluvio smart-module list` command to see the available SmartModules:

%copy first-line%
```bash
$ fluvio smart-module list
 NAME        STATUS             SIZE
 log-filter  SmartModuleStatus  161916
```

Once the SmartModule is created, it can be used by other areas of the system (consumers, producers, connectors, etc):

%copy first-line%
```bash
$ fluvio consume server-logs -B --filter=log-filter
```


### Read next

- [Explore filter use-cases](https://www.infinyon.com/blog/2021/06/smartstream-filters/)
- [Writing a map to transform records]({{< ref "/docs/smartmodules/map" >}})
- [Writing an aggregate to sum numbers]({{< ref "/docs/smartmodules/aggregate" >}})
