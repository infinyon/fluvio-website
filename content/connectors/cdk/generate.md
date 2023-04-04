---
title: Generate 
weight: 30
---

CDK generate helps developers build a sample SmartConnector project by answering a few simple questions.
Prerequisites

This section assumes that CDK is installed.
 
Use `cdk generate` to create a new connector project:

{{% inline-embed file="embeds/cli/example/cdk-generate-example.md" %}}

The generator created Rust project ready to compile:


{{% inline-embed file="embeds/cdk/tree-output.md" %}}

This is a simple SmartConnector `my-connector`:

{{% inline-embed file="embeds/cdk/my-connector-code.md" %}}

**Note** the `Connector.toml` file. This file contains Connector parameters required to load the file in the Cluster and publish it to SmartConnector Hub. 

{{% inline-embed file="embeds/cdk/my-connector-connector-toml.md" %}}

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