---
title: FilterMap
weight: 40
toc: true
---

-> **Note**: Currently, `filter-map` SmartModules are not supported in connectors.

SmartModule FilterMaps are used to both transform _and_ potentially filter records from a stream at the same time. This can be useful for a number of
scenarios including working with data with nullable fields, or working with subsets of event data. In these cases, FilterMap allows us discard irrelevant data - such as records with null fields or event types that we don't care about - while also performing meaningful work with relevant data - such as reformatting fields we've extracted or events we've gathered.



FilterMap functions work by returning an `Option` of a new record. To discard a record from the stream, return `None`. Otherwise, transform
the record according to your needs and return it as `Some(record)`.

<img src="/smartmodules/images/smartmodule-filtermap.svg" alt="SmartModule FilterMap" justify="center" height="190">

Let's dive in and see how to use this in practice. You can find the full code for this doc's example [in the fluvio-smartmodule-examples repository][1].

##### Prerequisites

This section assumes that SMDK is [installed].


## Getting Practical: Transform `Order Ready` events in a mobile shopping App

For this example, we'll write a simplified version of the [online grocery-notifications example we blogged about][2]. In this simplified example, we'll consider a stream that has two event types: Account Created and Order Ready. We'll use FilterMap to discard `account_created` events, and to transform `order_ready` events into text messages that should be sent to the user.

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

### Create a SmartModule Project

Run `smdk generate` with the name of the filter and choose the  "filter" options:

%copy first-line%
```bash
$ smdk generate filter-map
project-group => 'john'
fluvio-smartmodule-cargo-dependency => '"0.3.0"'
🔧   Destination: ~/smdk/filter-map ...
🔧   Generating template ...
✔ 🤷   Will your SmartModule use init parameters? · false
✔ 🤷   Which type of SmartModule would you like? · filter-map
Ignoring: /var/folders/5q/jwc86771549058kmbkbqjcdc0000gn/T/.tmp5Yi3lU/cargo-generate.toml
[1/5]   Done: Cargo.toml
[2/5]   Done: README.md
[3/5]   Done: SmartModule.toml
[4/5]   Done: src/lib.rs
[5/5]   Done: src
🔧   Moving generated files into: `~/smdk/filter-map`...
💡   Initializing a fresh Git repository
✨   Done! New project created ~/smdk/filter-map
```

The generator created a sample code for us, let's go ahead and update it.

### The Code: Writing our FilterMap

We'll want to `cd` into the project directory for the rest of the commands to work:

%copy first-line%
```bash
$ cd filter-map
```

Now, let's jump right into the code. Copy and paste the following block into the `src/lib.rs` file in the new SmartModule project:

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

Let's break down what's happening here. First, we have a `GroceryEvent` enum which represents the different types of input record we are expecting. We're using `serde` to automatically serialize and deserialize our JSON into this enum type.

Next, we check what type of event we received. If it's an `order_ready` event, we transform it, picking the phone number and account preferred name and creating a record that contains a text-friendly message that should be sent to the user. If it is any other event type, we filter it out by returning `Ok(None)`.

### Build the SmartModule

Let's make sure our code compiles. If eveything works as expected, there is a `.wasm` file generated in the target directory.

%copy first-line%
```bash
$ smdk build
...
Compiling filter-map v0.1.0 (~/smdk/filter-map)
Finished release-lto [optimized] target(s) in 11.97s
```

Your SmartModule WASM binary is now ready for use.

### Test with SMDK

Let's test our work using the command line `test` facility.

Test `filter` component to ensure non-mathing records are eliminated:

%copy first-line%
```bash
$ smdk test --text='{"type":"account_created","account_id":"1","username":"bill9876","preferred_name":"Bill","phone_number":"1-800-234-5678"}'
loading module at: ~/smdk/filter-map/target/wasm32-unknown-unknown/release-lto/filter_map.wasm
0 records outputed
```

Test `map` part to ensure matching records are transformed:

%copy first-line%
```bash
$ smdk test --text='{"type":"order_ready","account_id":"1","sms_number":"1-800-234-5678","sms_name":"Bill"}'
loading module at: ~/smdk/filter-map/target/wasm32-unknown-unknown/release-lto/filter_map.wasm
1 records outputed
{"message":"Hello Bill, your groceries have been collected and are ready to pick up!","number":"1-800-234-5678"}
```

### Test on Cluster

Let's create a new Fluvio topic to produce the sample records we want to consume with our SmartModule:

%copy first-line%
```bash
$ fluvio topic create filter-map
topic "filter-map" created
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

Let's double check it's all there.

%copy first-line%
```bash
$ fluvio consume filter-map -dB
Consuming records from the beginning of topic 'filter-map'
{"type":"account_created","account_id":"1","username":"bill9876","preferred_name":"Bill","phone_number":"1-800-234-5678"}
{"type":"order_ready","account_id":"1","sms_number":"1-800-234-5678","sms_name":"Bill"}
{"type":"account_created","account_id":"2","username":"mary","preferred_name":"Mary","phone_number":"1-222-222-2222"}
{"type":"order_ready","account_id":"2","sms_number":"1-222-222-2222","sms_name":"Mary"}
```

#### Load SmartModule to Fluvio

The SmartModule can be loaded to local Fluvio Cluster or [InfinyOn Cloud], as determined by the [`current profile`]. In this example, the profile points to InfinyOn Cloud.

%copy first-line%
```bash
$ smdk load
Loading package at: ~/smdk/filter-map
Found SmartModule package: filter-map
loading module at: ~/smdk/filter-map/target/wasm32-unknown-unknown/release-lto/filter_map.wasm
Trying connection to fluvio router.infinyon.cloud:9003
Creating SmartModule: filter-map
```

Rust `fluvio smartmodule list` to ensure your SmartModule has been uploaded:

%copy first-line%
```bash
$ fluvio smartmodule list
  SMARTMODULE                   SIZE     
  john/filter-map@0.1.0         166.6 KB
```

SmartModules that have been uploaded on the cluster can be used by other areas of the system (consumers, producers, connectors, etc):

%copy first-line%
```bash
$ fluvio consume filter-map -dB --smartmodule=john/filter-map@0.1.0
Consuming records from the beginning of topic 'filter-map'
{"message":"Hello Bill, your groceries have been collected and are ready to pick up!","number":"1-800-234-5678"}
{"message":"Hello Mary, your groceries have been collected and are ready to pick up!","number":"1-222-222-2222"}
```

Congratulations! :tada: Eveything worked as expected!

## Publish to SmartModule Hub

Let's [publish] this SmartModule to [SmartModule Hub] to make accessible to others.

%copy first-line%
```bash
$ smdk publish
Creating package john/filter-map@0.1.0
.. fill out info in hub/package-meta.yaml
Package hub/filter-map-0.1.0.ipkg created
Package uploaded!
```

Let's double check that the SmartModule is available for download:

%copy first-line%
```bash
$ fluvio hub list
  SMARTMODULE                    
  john/filter-map@0.1.0         
```

Congratulations! :tada: Your SmartModule is now available for download in the SmartModule Hub.

## Read next

- [Explore map use-cases](https://www.infinyon.com/blog/2021/08/smartstream-map-use-cases/)
- [Writing a JSON filter]({{< ref "filter" >}})
- [Writing an aggregate to sum numbers]({{< ref "aggregate" >}})


[installed]: {{< ref "smartmodules/smdk/install" >}}
[publish]: {{< ref "smartmodules/smdk/publish" >}}
[InfinyOn Cloud]: https://infinyon.cloud
[`current profile`]: {{< ref "cli/client/profile" >}}
[SmartModule Hub]: {{< ref "smartmodules/hub/overview" >}}

[1]: https://github.com/infinyon/fluvio-smartmodule-examples/blob/master/grocery-notifications/src/lib.rs
[2]: https://www.infinyon.com/blog/2021/11/filter-map/
[downloaded the Fluvio CLI]: https://www.fluvio.io/download/
[using ArrayMap to break apart paginated API requests]: https://infinyon.com/blog/2021/10/smartstream-array-map-reddit/
[full source code for this example on GitHub]: https://github.com/infinyon/fluvio/blob/095d8f0cbbcc79ebc71cea464cd653ffde7af4e0/crates/fluvio-smartstream/examples/array_map_json_array/src/lib.rs
