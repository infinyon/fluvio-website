---
title: Build & Test
weight: 30
---

This section assumes `my-connector` project has been [generated]({{< ref "generate" >}}).

### Build Connector

Inside the `my-connector` project directory run `build`:

%copy first-line%
```bash
$ cdk build
...
Compiling my-connector v0.1.0 (~/cdk/my-connector)
 Finished release [optimized] target(s) in 1m 12s
```

The build process generated a binary for your custom connector. We are now ready to test it.

### Test Connector

Staring an connector instance for testing requires a configuration file:

* `--config <PATH>` is a path to the configuration file in YAML format. The project automatically generated a sample config `sample-config.yaml` that we can use as a starting point.

Use `cdk test` to start the instance:

%copy first-line%
```bash
$ cdk test --config sample-config.yaml
    Finished release [optimized] target(s) in 0.16s
Connector runs with process id: 80380
Starting my-connector source connector with CustomConfig { foo: "bar" }
```

Connector output will be redirected to the current terminal output. To stop running Connector in test mode, press Ctrl+C.

#### Test Result

The connector produces `Hello, Fluvio` to the topic `test-my-connector-topic`. Let's check it out:

%copy first-line%
```bash
$ fluvio consume test-my-connector-topic -B
Hello, Fluvio - 1
Hello, Fluvio - 2
Hello, Fluvio - 3
Hello, Fluvio - 4
...
```

Checkout the [next section]({{< ref "start-shutdown" >}}) for instructions on how to run the connector in the background.

### Steps

1. [Generate a Connector]({{< ref "generate" >}})
2. **[Build and Test]({{< ref "build-test" >}})**
3. [Start and Shutdown]({{< ref "start-shutdown" >}})
4. [Troubleshooting]({{< ref "troubleshooting" >}})
5. [Secrets]({{< ref "secrets" >}})
6. [Publish to Connector Hub]({{< ref "publish" >}})
7. [Use Examples in Github]({{< ref "github-examples" >}})