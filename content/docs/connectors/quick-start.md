---
menu: Quick Start
title: Connectors Overview
weight: 10
---

Fluvio stores the source code for its connectors in the [fluvio-connectors
repository].  When a new connector is released, it is packaged in a Docker container and published on Dockerhub. In addition, the connector catalog list all available source or sink connectors. 

At the moment, Fluvio has two official connectors:
* [test-connector (source)]
* [mqtt-connector (source)]

[Test source connector] produces a new record every 10ms to the topic of your choice. Use this connector to test the infrastructure and create your custom connectors.

[MQTT source connector] is a client implementation of an MQTT protocol, and it reads messages from an MQTT server and produces them to a fluvio topic.

Fluvio cluster offers a connector command-line interface (CLI) to start, stop and get the status of a container. A cluster may run many instances of the same or different connectors simultaneously.  Fluvio manages the connector infrastructure through Kubernetes. If you run a local installation of Fluvio, make sure it runs `minikube` or `k3d`.

-> Note: Fluvio local clusters installed outside of Kubernetes are not supported.

[fluvio-connectors repository]: https://github.com/infinyon/fluvio-connectors
[test-connector (source)]: https://github.com/infinyon/fluvio-connectors/tree/main/test-connector
[mqtt-connector (source)]: https://github.com/infinyon/fluvio-connectors/tree/main/mqtt
[Test source connector]:  https://github.com/infinyon/fluvio-connectors/tree/main/test-connector
[MQTT source connector]: https://github.com/infinyon/fluvio-connectors/tree/main/mqtt

# Commands

The connector CLI commands are a subcommand to [`fluvio cluster`](/cli/commands/cluster/).

## Create a connector
Creating a connector is a bit complicated. You will need to give the connector
CLI a config yaml.

%copy first-line%
```bash
$ fluvio cluster connector create --config my-connector-config.yaml
```

### The Connector Config
All connector configs require the following:
```yaml
version: v1
name: unique_identifying_name
type: official_connector_type # currently either mqtt or test-connector
topic: my_fluvio_topic
```
with optional arguments of:
```yaml
create_topic: true
direction: source
parameters:
  connector_arg_key1: connector_arg_val1
  connector_arg_key2: connector_arg_val2
secrets:
  secret_1_key: secret_1_val
```

Additionally, a given connector will have certain `parameters` that are
required. In the future, our catalog will describe and verify the arguments
before connector creation. For now, you must know that:
* The `mqtt` connector requires parameters of `mqtt-url`, `mqtt-topic` and
`fluvio-topic` with optional parameters of `timeout` and `qos`.
* The test connector requires parameters of `topic` with optional parameters of
`timeout` and `count`.

#### Mqtt Example
%copy%
```yaml
version: v1
name: my-test-mqtt
type: mqtt
topic: my-mqtt
create_topic: true
direction: source
parameters:
  mqtt-url: "mqtt.hsl.fi"
  mqtt-topic: "/hfp/v2/journey/#"
  fluvio-topic: "my-mqtt"
```


#### test-connector example
%copy%
```yaml
version: v1
name: my-test-connector
type: test-connector
topic: my-test-connector
create_topic: true
direction: source
parameters:
  topic: my-test-connector
  timeout: 1000
```

If the `creat_topic` key is true, the tool will create a the topic specified or
fail if it already exists.

## List connectors and their statuses
%copy first-line%
```bash
$ fluvio cluster connector list
-------------
 NAME               STATUS
 my-test-connector  Running
      my-test-mqtt  Running
```
## Delete a connector
%copy first-line%
```bash
$ fluvio cluster connector delete my-test-connector
```
