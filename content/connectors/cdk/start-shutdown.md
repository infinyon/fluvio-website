---
title: Start & Shutdown 
weight: 50
---

##### Prerequisites

This section assumes that CDK is [installed]({{< ref "install" >}}) and `my-connector` project has been [generated]({{< ref "generate" >}}).


### Connector start 

[Testing]({{<ref "/connectors/cdk/build-test#test---operation">}}) your connector runs the process in the foreground. 

When you are ready, you can run `cdk deploy start` to run your connector in the background.

%copy first-line%
```bash
$ cdk deploy start --config config-example.yaml 
Log file: /private/tmp/my-connector/my-connector.log
Connector runs with process id: 88589
```

{{<idea>}}
To start an official connector from a local `.ipkg` connector package file, you can provide it with the `--ipkg` option

Example:

%copy first-line%
```bash
$ cdk deploy start --ipkg infinyon-http-source-0.1.1.ipkg --config config-example.yaml
```
{{</idea>}}


### Connector shutdown

Conversely, when you want to stop running your connector, you can run `cdk deploy shutdown <connector name>` to stop the running process of your connector in background.

You can access the connector name from the [list of your running connectors]({{< ref "list-log" >}}), or you can find the connector `name` in your config file. 

{{<caution>}}
This command exits quietly
{{</caution>}}

1. [Install CDK]({{< ref "install" >}})
2. [Generate a SmartConnector]({{< ref "generate" >}})
3. [Build and Test]({{< ref "build-test" >}})
4. **[Start and Shutdown]({{< ref "start-shutdown" >}})**
5. [List and Logs]({{< ref "list-log" >}})
6. [Publish to SmartConnector Hub]({{< ref "publish" >}})