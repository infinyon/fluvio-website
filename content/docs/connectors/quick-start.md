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

[Test source connector] produces a new record every 1000ms to the topic of your choice. Use this connector to test the infrastructure and create your custom connectors.

[MQTT source connector] is a client implementation of an MQTT protocol, and it reads messages from an MQTT server and produces them to a fluvio topic.

Fluvio cluster offers a connector command-line interface (CLI) to start, stop and get the status of a container. A cluster may run many instances of the same or different connectors simultaneously.  Fluvio manages the connector infrastructure through Kubernetes. If you run a local installation of Fluvio, make sure it runs `minikube` or `k3d`.

-> Fluvio **local clusters** installed outside of Kubernetes are not supported at this time.

[fluvio-connectors repository]: https://github.com/infinyon/fluvio-connectors
[test-connector (source)]: https://github.com/infinyon/fluvio-connectors/tree/main/test-connector
[mqtt-connector (source)]: https://github.com/infinyon/fluvio-connectors/tree/main/mqtt
[Test source connector]:  https://github.com/infinyon/fluvio-connectors/tree/main/test-connector
[MQTT source connector]: https://github.com/infinyon/fluvio-connectors/tree/main/mqtt


## Connector Configuration

Fluvio uses configuration files to instantiate connectors. The configuration file has two sections:

* connector definition
* connector properties

The `connector definition` section has the following parameters:

```yaml
version: v1                     
name: unique_identifying_name
type: official_connector_type   # pick from connector catalog
direction: source_or_sink
topic: my_fluvio_topic
create_topic: true              # optional
```

The `connector properties` are fields required by the external service.

```yaml
parameters:
  connector_arg_key1: connector_arg_val1
  connector_arg_key2: connector_arg_val2
```

For a list of parameters, check connector properties in the connector catalog.


## Define a Connector

Starting a connector is a two-step process: create the `configuration file`, run the `create connector` command.


### Mqtt Example

An sample `configuration file` for an mqtt connector:

%copy%

```yaml
version: latest
name: my-test-mqtt
type: mqtt
direction: source
topic: my-mqtt
create_topic: true
parameters:
  mqtt-url: "mqtt.hsl.fi"
  mqtt-topic: "/hfp/v2/journey/#"
  fluvio-topic: my-mqtt
```

In the future, Fluvio connector catalog will describe and verify the arguments
before connector creation. For now you must ensure the configuration parameters are accurate:

* `version` is the version of the connector from connector catalog.
* `name` is the connector name as defined in the connector catalog.
* `type` is the unique identifier of the connector.
* `direction` defines if the connector is sink or source.
* `topic` is the fluvio topic the connetor will publish to.
* `parameters` is the list of parameters as defined by the connector.
  * `mqtt-url` defines the mqtt server url.
  * `mqtt-topic` defines the mqtt topic.
  * `timeout` controls reconnection logic (optional).
  * `qos` manages quality of service (optional).

For additional information, checkout mqtt connector in [github](https://github.com/infinyon/fluvio-connectors/blob/main/mqtt/src/main.rs).


### Test-connector Example

An sample `configuration file` for a test connector:

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
```

The `connector definition` section is defined above and not repeated here. The test connector specific parameters are:
* `timeout` interval for sending records (default: 1000)
* `count` the number of records to produce (defaults to i64 max).

For additional information, checkout test connector in [github](https://github.com/infinyon/fluvio-connectors/blob/main/test-connector/src/main.rs).

## Connector Commands

To show a list of available connector commands, run the following CLI:

%copy first-line%
```bash
$ fluvio cluster connector -h
```

### Create a Connector

Use the following cli command to create a connector:

%copy first-line%
```bash
$ fluvio cluster connector create --config my-connector-config.yaml
```

If the `create_topic` is configured, a topic is created. If the topic already exists, the command is ignored. If `create_topic` is not configured, and the topic does not exist, the connector returns an error.


### List all Connectors

Use the following cli command to retrieve the status of the connectors:

%copy first-line%
```bash
$ fluvio cluster connector list
-------------
 NAME               STATUS
 my-test-connector  Running
      my-test-mqtt  Running
```

### Delete a Connector

Use the following cli command to delete a connector:

%copy first-line%
```bash
$ fluvio cluster connector delete my-test-connector
```

Deleting a connector does not impact the topic. Hence, the records are available for reading after the connector is deleted. Recreating the same connector will resume publishing to the same topic.
