---
title: Build & Test 
weight: 40
---

##### Prerequisites

This section assumes that CDK is [installed]({{< ref "install" >}}) and `my-connector` project has been [generated]({{< ref "generate" >}}).

### Build - Operation

Navigate to `my-connector` project directory and run `build`

%copy first-line%
```bash
$ cdk build
...
Compiling my-connector v0.1.0 (~/cdk/my-connector)
 Finished release [optimized] target(s) in 1m 12s
```

The build process generated a binary for your custom connector. We are now ready to test it.

## Test - Operation

If the connector builds successfully, it’s time to start an instance with `cdk test`:

The `--config <PATH>` argument is required. It is a path to the configuration file in YAML format. 

Generating the project with `cdk generate` should have created a sample config for you named `config-example.yaml`

%copy first-line%
```bash
$ cdk test --config config-example.yaml 
    Finished release [optimized] target(s) in 0.16s
Connector runs with process id: 80380
Starting my-connector source connector with CustomConfig { foo: "bar" }
```

SmartConnector output will be redirected to the current terminal output.

To stop running SmartConnector in test mode, press Ctrl+C.

### Steps

1. [Install CDK]({{< ref "install" >}})
2. [Generate a SmartConnector]({{< ref "generate" >}})
3. **[Build and Test]({{< ref "build-test" >}})**
4. [Start and Shutdown]({{< ref "start-shutdown" >}})
5. [List and Logs]({{< ref "list-log" >}})
6. [Publish to SmartConnector Hub]({{< ref "publish" >}})