---
title: Troubleshooting
weight: 60
---

This section assumes `my-connector` project has been [generated]({{< ref "generate" >}}).

After you [start your connectors]({{<ref "/connectors/cdk/start-shutdown#connector-start">}}), you can view their logs for troubleshooting:

### Log

To display the logs, you'll need the connector name:

%copy first-line%
```bash
$ cdk deploy log --name my-my-connector-test-connector
Starting my-connector source connector with CustomConfig { foo: "bar" }
```

#### Changing the Log Level

By default connectors will use the `info` logging level, you can change the log level by using the `deploy` command argument `--log-level`.

```bash
cdk deploy start --config sample-config.yaml --log-level debug
```

The log levels are:
- `error`
- `warn`
- `info`
- `debug`
- `trace`

In the [next section]({{< ref "secrets" >}}) we'll take a look at how to use secrets.

### Steps

1. [Generate a Connector]({{< ref "generate" >}})
2. [Build and Test]({{< ref "build-test" >}})
3. [Start and Shutdown]({{< ref "start-shutdown" >}})
4. **[Troubleshooting]({{< ref "troubleshooting" >}})**
5. [Secrets]({{< ref "secrets" >}})
6. [Publish to Connector Hub]({{< ref "publish" >}})
7. [Use Examples in Github]({{< ref "github-examples" >}})