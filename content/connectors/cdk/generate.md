---
title: Generate
weight: 20
---

### Generate a Connector

CDK generate helps developers build a sample Connector project by answering a few simple questions.

Use `cdk generate` to create a new connector project:

{{% inline-embed file="embeds/cli/example/cdk-generate-example.md" %}}

The generator created Rust project ready to compile:

{{% inline-embed file="embeds/cdk/tree-output.md" %}}


This a simple connector with the code in `src/main.rs`:

{{% inline-embed file="embeds/cdk/main-code.md" %}}


Connectors may also have can have configuration parameters as defined in `src/config.rs`:

{{% inline-embed file="embeds/cdk/config-code.md" %}}


 The `Connector.toml` file contains the definition of the Connector parameters required to load the file in the Cluster and publish it to Connector Hub.

{{% inline-embed file="embeds/cdk/connector-toml.md" %}}

#### Sections
* `package` is used to build the connector FQDN `acme/my-connector@0.1.0`, and the description to publish to  Hub. The `group` name is equivalent to the package owner in the Hub.
* `direction` is used to declare the direction data flows through the connector, with respect to the Fluvio cluster. An inbound connector uses `source = true`, and and outbound connectdor uses `sink = true`
* `custom.properties.foo` defines a user configuration key `foo` that can be used in the logic of the connector.

The project is ready to build and test. Checkout the [next section]({{< ref "build-test" >}}) for instructions.

### Steps

1. **[Generate a Connector]({{< ref "generate" >}})**
2. [Build and Test]({{< ref "build-test" >}})
3. [Start and Shutdown]({{< ref "start-shutdown" >}})
4. [Troubleshooting]({{< ref "troubleshooting" >}})
5. [Secrets]({{< ref "secrets" >}})
6. [Publish to Connector Hub]({{< ref "publish" >}})
7. [Start from Connector Hello]({{< ref "github-examples" >}})