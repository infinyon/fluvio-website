---
title: Cloud Connectors
menu: Connector
weight: 40
---

The `fluvio cloud connector` subcommands are used to manage Connectors in InfinyOn Cloud.

%copy first-line%
```bash
$  fluvio cloud connector -h
```

{{% inline-embed file="embeds/cli/help/fluvio-cloud-connector.md" %}}

---

## `fluvio cloud connector create`

This command is used to provision a new connector.

%copy first-line%
```bash
$  fluvio cloud connector create -h
```

{{% inline-embed file="embeds/cli/help/fluvio-cloud-connector-create.md" %}}

To create a connector, you need to create a YAML-based connector config file.

For more about the connector config file, see the [Cloud connectors page]({{<ref "/connectors/cloud-connectors.md" >}}) or the [connector template]({{<ref "/connectors/connector-templates.md" >}})

When running `fluvio cloud connector create`, pass the path to this file using the `--config`
option.

Example usage:

%copy first-line%
```bash
$ fluvio cloud connector create --config=./cats.yaml
connector "cat-facts" (http-source) created
```

---

## `fluvio cloud connector config`

Command to show the configuration file used to create this connector.

%copy first-line%
```bash
$  fluvio cloud connector config -h
```

{{% inline-embed file="embeds/cli/help/fluvio-cloud-connector-config.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio cloud connector config cat-facts

name: cat-facts
type: http-source
topic: cat-facts
version: 0.4.1
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10s

```

---

## `fluvio cloud connector list`

This command show you all the existing Connectors in your cluster.

%copy first-line%
```bash
$  fluvio cloud connector list -h
```

{{% inline-embed file="embeds/cli/help/fluvio-cloud-connector-list.md" %}}


---

## `fluvio cloud connector update`

Command to update and restart an existing connector.

%copy first-line%
```bash
$  fluvio cloud connector update -h
```

{{% inline-embed file="embeds/cli/help/fluvio-cloud-connector-update.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio cloud connector update --config=./cats.yaml
connector "cat-facts" (http-source) updated
```

---

## `fluvio cloud connector logs`

Command to view the logs written by the connector.

%copy first-line%
```bash
$  fluvio cloud connector logs -h
```

{{% inline-embed file="embeds/cli/help/fluvio-cloud-connector-logs.md" %}}


Example usage:

%copy first-line%
```bash
% fluvio cloud connector logs cat-facts
2022-10-21T14:55:13.508989Z  INFO http_source: Starting HTTP source connector connector_version="0.4.1" git_hash="0ad913c5ceb732881fd753874e5082777bbed91e"
2022-10-21T14:55:13.509096Z  INFO http_source: interval=10s method=GET topic=cat-facts output_parts=body output_type=text endpoint=https://catfact.ninja/fact
2022-10-21T14:55:13.510284Z  INFO fluvio::config::tls: Using verified TLS with certificates from paths domain="broad-union-b685e7fda03fefb3d5221d0a3b9c64c7.c.infinyon.cloud"
2022-10-21T14:55:13.515459Z  INFO fluvio::fluvio: Connecting to Fluvio cluster fluvio_crate_version="0.14.0" fluvio_git_hash="e96d8e2738ee39ddbb64fea37134f119f97e25bf"
2022-10-21T14:55:13.574584Z  INFO connect: fluvio::sockets: connect to socket add=fluvio-sc-public:9003
...
```

---

## `fluvio cloud connector delete`

This command deletes an existing Connector.

%copy first-line%
```bash
$  fluvio cloud connector delete -h
```

{{% inline-embed file="embeds/cli/help/fluvio-cloud-connector-delete.md" %}}

Example usage:

%copy first-line%
```bash
$ fluvio cloud connector delete cat-facts
connector "cat-facts" deleted
```
