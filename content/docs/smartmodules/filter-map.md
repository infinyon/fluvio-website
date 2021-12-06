---
title: FilterMap
weight: 33
toc: false
---

SmartModule FilterMaps are used to both transform _and_ potentially filter
records from a stream at the same time. This can be useful for a number of
scenarios including working with data with nullable fields, or working with
subsets of event data. In these cases, FilterMap allows us discard irrelevant
data - such as records with null fields or event types that we don't care about -
while also performing meaningful work with relevant data - such as reformatting
fields we've extracted or events we've gathered.

FilterMap functions work by returning an `Option` of a new record. To discard a
record from the stream, return `None`. Otherwise, transform
the record according to your needs and return it as `Some(record)`.

Let's dive in and see how to use this in practice. You can find the full code
for this doc's example [in the fluvio-smartmodule-examples repository][1].

### Example project

For this example, we'll write a simplified version of the [online grocery-notifications
example we blogged about][2]. In this simplified example, we'll consider a stream that
has two event types: Account Created and Order Ready. We'll use FilterMap to discard
`account_created` events, and to transform `order_ready` events into text messages that should
be sent to the user.

Let's take a look at some sample input for this example:

```bash
{"type":"account_created","account_id":"1","username":"bill9876","preferred_name":"Bill","phone_number":"1-800-234-5678"}
{"type":"order_ready","account_id":"1","sms_number":"1-800-234-5678","sms_name":"Bill"}
{"type":"account_created","account_id":"2","username":"mary","preferred_name":"Mary","phone_number":"1-222-222-2222"}
{"type":"order_ready","account_id":"2","sms_number":"1-222-222-2222","sms_name":"Mary"}
```

We'd like to discard the `account_created` events, keep the `order_ready` events, and
transform the `order_ready` events to a nice message that should be sent to the user.
The output records in this stream should look like this:

```bash
{"number":"1-800-234-5678","message":"Hello Bill, your groceries have been collected and are ready to pick up!"}
{"number":"1-222-222-2222","message":"Hello Mary, your groceries have been collected and are ready to pick up!"}
```

### Create a new Project

We can use the `cargo-generate` tool to create a new SmartModules project that
is ready to go. If you don't already have it, you can install `cargo-generate`
using this command:

%copy first-line%
```bash
$ cargo install cargo-generate
```

Then, use the following command to create a new SmartModules FilterMap project.

%copy first-line%
```bash
$ cargo generate --git="https://github.com/infinyon/fluvio-smartmodule-template"
⚠️   Unable to load config file: ~/.cargo/cargo-generate.toml
🤷   Project Name : filter-map
🔧   Generating template ...
✔ 🤷   Which type of SmartModule would you like? · array-map
[1/7]   Done: .cargo/config.toml
[2/7]   Done: .cargo
[3/7]   Done: .gitignore
[4/7]   Done: Cargo.toml
[5/7]   Done: README.md
[6/7]   Done: src/lib.rs
[7/7]   Done: src
🔧   Moving generated files into: `filter-map`...
✨   Done! New project created filter-map
```

We'll want to `cd` into the project directory for the rest of the commands
to work:

%copy first-line%
```bash
$ cd filter-map
```

Now, let's jump right into the code. Copy and paste the following block into
the `src/lib.rs` file in the new SmartModule project:

%copy%
```rust
use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};
use serde::{Deserialize, Serialize};

/// Events that may take place in an online grocery service
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum GroceryEvent {
    AccountCreated {
        account_id: String,
        username: String,
        preferred_name: String,
        phone_number: String,
    },
    OrderReady {
        account_id: String,
        sms_number: String,
        sms_name: String,
    },
}

#[smartmodule(filter_map)]
fn filter_map(record: &Record) -> Result<Option<(Option<RecordData>, RecordData)>> {
    let event: GroceryEvent = match serde_json::from_slice(record.value.as_ref()) {
        Ok(event) => event,
        Err(_) => return Ok(None), // Skip if we fail to parse JSON
    };

    let sms_message = match event {
        GroceryEvent::OrderReady {
            sms_name,
            sms_number,
            ..
        } => serde_json::json!({
            "number": sms_number,
            "message": format!(
                "Hello {}, your groceries have been collected and are ready to pick up!",
                sms_name
            ),
        }),
        _ => return Ok(None),
    };

    let message_json = serde_json::to_string(&sms_message)?;
    Ok(Some((record.key.clone(), message_json.into())))
}
```

Let's break down what's happening here. First, we have a `GroceryEvent` enum which
represents the different types of input record we are expecting. We're using `serde`
to automatically serialize and deserialize our JSON into this enum type.

Next, we check what type of event we received. If it's an `order_ready` event, we
transform it, picking the phone number and account preferred name and creating a
record that contains a text-friendly message that should be sent to the user.
If it is any other event type, we filter it out by returning `Ok(None)`.

Let's take a look at how we can run this SmartModule with our sample data!

### Running the FilterMap

Before getting started, make sure you have [downloaded the Fluvio CLI] and followed
the getting started guide to get up and running with a Fluvio cluster. Then, if you
haven't done so already, you'll need to install the `wasm32-unknown-unknown` target
for Rust using the following command:

%copy first-line%
```bash
$ rustup target add wasm32-unknown-unknown
```

Now we'll be able to compile the FilterMap SmartModule. Let's use release mode so
we get the smallest WASM binary possible:

%copy first-line%
```bash
$ cargo build --release
```

Next, we'll need to create a new Fluvio topic to produce and consume our data using
this command:

%copy first-line%
```bash
$ fluvio topic create filter-map
topic "array-map" created
```

Now, let's put our sample data into a file and produce it to our topic.
Create `groceries.txt` with the following contents:

%copy%
```bash
{"type":"account_created","account_id":"1","username":"bill9876","preferred_name":"Bill","phone_number":"1-800-234-5678"}
{"type":"order_ready","account_id":"1","sms_number":"1-800-234-5678","sms_name":"Bill"}
{"type":"account_created","account_id":"2","username":"mary","preferred_name":"Mary","phone_number":"1-222-222-2222"}
{"type":"order_ready","account_id":"2","sms_number":"1-222-222-2222","sms_name":"Mary"}
```

Now we can produce the sample data to our topic.

%copy first-line%
```bash
$ fluvio produce filter-map -f ./groceries.txt
```

Finally, let's consume our data using our FilterMap SmartModule and see that our
output records match the format we expect, with the phone number and text message
to send:

%copy first-line%
```bash
$ fluvio consume filter-map -B --filter-map=target/wasm32-unknown-unknown/release/filter_map.wasm
Consuming records from the beginning of topic 'filter-map'
{"message":"Hello Bill, your groceries have been collected and are ready to pick up!","number":"1-800-234-5678"}
{"message":"Hello Mary, your groceries have been collected and are ready to pick up!","number":"1-222-222-2222"}
```

Congratulations, you just completed your first FilterMap example! You can find the
[full source code for this example on GitHub][1], along with the full sources for many
other SmartModules examples.

### Read next

- [Explore map use-cases](https://www.infinyon.com/blog/2021/08/smartstream-map-use-cases/)
- [Writing a JSON filter]({{< ref "/docs/smartmodules/filter" >}})
- [Writing an aggregate to sum numbers]({{< ref "/docs/smartmodules/aggregate" >}})

[1]: https://github.com/infinyon/fluvio-smartmodule-examples/blob/master/grocery-notifications-simple/src/lib.rs
[2]: https://www.infinyon.com/blog/2021/11/filter-map/
[downloaded the Fluvio CLI]: https://www.fluvio.io/download/
[using ArrayMap to break apart paginated API requests]: https://infinyon.com/blog/2021/10/smartstream-array-map-reddit/
[full source code for this example on GitHub]: https://github.com/infinyon/fluvio/blob/095d8f0cbbcc79ebc71cea464cd653ffde7af4e0/crates/fluvio-smartstream/examples/array_map_json_array/src/lib.rs
