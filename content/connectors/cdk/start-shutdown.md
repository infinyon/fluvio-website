---
title: Start & Shutdown
weight: 40
---

This section assumes `my-connector` project has been [generated]({{< ref "generate" >}}).

### Connector start

[Testing]({{<ref "/connectors/cdk/build-test#test---operation">}}) your connector runs the process in the foreground. Use `cdk deploy start` to run your connector in the background: 

%copy first-line%
```bash
$ cdk deploy start --config sample-config.yaml
Log file: /private/tmp/my-connector/my-connector.log
Connector runs with process id: 88589
```

### List running connectors

CDK offers a convenience function to list running connectors:

%copy first-line%
```bash
$ cdk deploy list
 NAME                            STATUS  
 my-my-connector-test-connector  Running 
```

You can use the connector name to shut it down.

### Connector shutdown

Stop a running your connector with `cdk deploy shutdown` 

%copy first-line%
```bash
$ cdk deploy shutdown --name my-my-connector-test-connector
Shutting down connector: my-my-connector-test-connector 
pid: 56421
 ```

In the [next section]({{< ref "troubleshooting" >}}) we'll take a look at the logs for troubleshooting.


### Steps

1. [Generate a Connector]({{< ref "generate" >}})
2. [Build and Test]({{< ref "build-test" >}})
3. **[Start and Shutdown]({{< ref "start-shutdown" >}})**
4. [Troubleshooting]({{< ref "troubleshooting" >}})
5. [Secrets]({{< ref "secrets" >}})
6. [Publish to Connector Hub]({{< ref "publish" >}})
7. [Use Examples in Github]({{< ref "github-examples" >}})


[InfinyOn Cloud]: https://infinyon.cloud