---
title: Cloud Connectors
menu: Connector
weight: 40
---

The `fluvio cloud connector` family of commands is used to create, delete, and troubleshoot Connectors in InfinyOn Cloud. The connectors can be inbound or outbound, as provisioned in the configuration file.

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


New connectors require a configuration file (commonly called `connect.yml`) which is used to pass general and connector-specific parameters to the connector instance.

An example connector config file might look like this:

%copy%
```yaml
# connect.yml
version: latest
name: cat-facts
type: http-source
topic: cat-facts
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10s
```

Here's a description of the available options:

- `version`: The published version of the connector
- `name`: The name given to the instance of the connector when it is created
- `type`: The type of connector. This corresponds to the name of the docker image
  that this connector is published in, e.g. `infinyon/fluvio-connect-<type>`
- `topic`: The name of the Fluvio topic that this connector will produce to.
- `direction`: Whether the connector is a `source` or a `sink` connector
- `parameters`: An object that contains connector-specific parameters.
  Also, connectors that support SmartModules specify their SmartModule names here:
  - `filter`: The name of a Filter SmartModule to apply
  - `map`: The name of a Map SmartModule to apply
  - `arraymap`: The name of an ArrayMap SmartModule to apply

-> **Note**: Currently, `aggregate` and `filter-map` SmartModules are not supported in connectors.

-> **Note**: The Fluvio topic set in `topic` will be automatically created if the Fluvio `topic` does not exist.

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
version: latest
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

Make a small change in the cats cats.yaml, for example change `interval: 20s`, the update:

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