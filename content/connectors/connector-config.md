---
title: Connector configuration 
weight: 20
---

This is the template YAML connector file. To make it useful, it needs to be
populated. In the next section we will go over the different sections of the connector configuration file.

{{<code file="embeds/templates/connector-template.yaml" lang="yaml" copy="true">}}


## Connector `apiVersion` configuration

The `apiVersion` is the version of the connector API that the connector uses to parse the configuration file. The current accepted versions: `0.1.0` and `0.2.0`.

## Connector `meta` configuration

The `meta` section contains the metadata for the connector:

* The `name` is the name of the connector. e.g. `my-connector`.
* The `type` is the type of the connector. e.g. `http-source`, `http-sink`, `mqtt-source`.
  * See the [connectors]({{<ref "/connectors">}}) section for the full list of connectors supported.
* The `version` is the version of the connector. e.g. `0.2.0`.
* The `topic` is the topic that the connector will connect to. e.g. `my-topic`. The topic will be created automatically if it does not exist.
* The `secrets`(optional) is a list of secrets that the connector will use. This accepts a list of objects with the key `name`.
  * See the [secrets]({{<ref "connectors/secrets.md">}}) section for more information.
* The `producer`(optional) is the producer configuration for the connector. Currently, this is only used for `source`/`inbound` connectors. The current supported configurations are `linger`, `compression` and `batch_size`. All configurations are optional. See examples to a list of valid values for each configuration.
* The `consumer`(optional) is the consumer configuration for the connector. Currently, this is only used for `sink`/`outbound` connectors. The current supported configurations are `id`, `partition`, `max_bytes` and `offset`. All configurations are optional. See examples to a list of valid values for each configuration.

An example with all the keys filled for a `http-source` connector:

{{<code file="embeds/templates/connector-template-meta-source-filled.yaml" lang="yaml" copy="true">}}

An example with all the keys filled for a `http-sink` connector:

{{<code file="embeds/templates/connector-template-meta-sink-filled.yaml" lang="yaml" copy="true">}}


## Connector `transforms` configuration

Connectors support `transforms`. Records can be modified before they are sent to the topic. The `transforms` section is a list of `transform` objects. Each `transform` object has an `uses` and a `with` section.

* `uses` is the reference to the SmartModule used in the transform.
* `with` is the configuration for the transform
  * The section is different for each transform
  * See the [connectors]({{<ref "/connectors">}}) reference documentation for available configuration options

See the [Transformations]({{<ref "docs/concepts/transformations-chain.md">}}) section for more information.

See [Tutorials]({{<ref "docs/tutorials/data-pipeline.md">}}) for examples of using transforms.
