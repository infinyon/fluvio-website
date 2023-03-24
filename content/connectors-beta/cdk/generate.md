---
title: Generate 
weight: 30
---

CDK generate helps developers build a sample SmartConnector project by answering a few simple questions.
Prerequisites

This section assumes that CDK is installed.
 
Use `cdk generate` to create a new connector project:

%copy first-line%
```bash
$ cdk generate
🤷   Project Name: my-connector
🔧   Destination: ~/my-connector ...
🔧   project-name: my-connector ...
🔧   Generating template ...
✔ 🤷   Will your Connector be public? · false
🤷   Please set a group name: aj
✔ 🤷   Which type of Connector would you like [source/sink]? · source
Ignoring: /var/folders/r8/4x6_d2rn283946frzd1gc1pr0000gn/T/.tmptToFV3/cargo-generate.toml
[1/6]   Done: Cargo.toml             
[2/6]   Done: Connector.toml
[3/6]   Done: config-example.yaml
[4/6]   Done: src/config.rs
[5/6]   Done: src/main.rs
[6/6]   Done: src
🔧   Moving generated files into: `~/my-connector`...
💡   Initializing a fresh Git repository
✨   Done! New project created ~/my-connector
```

The generator created Rust project ready to compile:

```bash
$ tree 
.
├── Cargo.toml
├── Connector.toml
├── config-example.yaml
└── src
    ├── config.rs
    └── main.rs

2 directories, 5 files
```

This is a simple SmartConnector `my-connector`:

```rust
mod config;
use config::CustomConfig;


use fluvio::{RecordKey, TopicProducer};
use fluvio_connector_common::{
    connector,
    Result
};

#[connector(source)]
async fn start(config: CustomConfig, producer: TopicProducer) -> Result<()> {
    println!("Starting my-connector source connector with {config:?}");
    for i in 1..1000 {
        let value = format!("Hello, Fluvio - {i}");
        producer.send(RecordKey::NULL, value).await?;
        producer.flush().await?;
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    Ok(())
}
```

**Note** the `Connector.toml` file. This file contains Connector parameters required to load the file in the Cluster and publish it to SmartConnector Hub. 

%copy first-line%
```bash
$ cat Connector.toml
[package]
name = "my-connector"
group = "aj"
version = "0.1.0"
apiVersion = "0.1.0"
fluvio = "0.10.0"
description = ""
license = "Apache-2.0"
visibility = "private"

[direction]
 source = true

[deployment]
binary = "my-connector"

[custom.properties.foo]
title = "Foo"
description = "Foo"
type = "string"
```

#### Sections
* `package` is used to build the SmartConnector FQDN `aj/my-connector@0.1.0`, and the description to publish to  Hub. The `group` name is equivalent to the package owner in the Hub. 
* `direction` is used to declare the direction data flows through the connector, with respect to the Fluvio cluster. An inbound connector uses `source = true`, and and outbound connectdor uses `sink = true` 
* `deployment`
* `custom.properties.foo` defines a user configuration key `foo` that can be used in the logic of the connector.

The project is ready to build and test. Checkout the next section for instructions.

### Steps

1. [Install CDK]({{< ref "install" >}})
2. **[Generate a SmartConnector]({{< ref "generate" >}})**
3. [Build and Test]({{< ref "build-test" >}})
4. [Start and Shutdown]({{< ref "start-shutdown" >}})
5. [List and Logs]({{< ref "list-log" >}})
6. [Publish to SmartConnector Hub]({{< ref "publish" >}})