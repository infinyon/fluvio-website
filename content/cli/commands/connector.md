---
title: Connector
weight: 30
---

The `fluvio connector` family of commands is used to create and delete Managed Connectors,
as well as to view basic information about existing connectors.

## `fluvio connector create`

This command is used to create new Managed Connectors.

```
Create a new Managed Connector

fluvio connector create --config <config>

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -c, --config <config>    The name for the new Managed Connector
```

New connectors require a
configuration file (commonly called `connect.yml`) which is used to pass general
and connector-specific parameters to the connector instance.
An example connector config file might look like this:

%copy%
```yaml
# connect.yml
version: v1
name: cat-facts
type: http
topic: cat-facts
create_topic: true
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10
```

Here's a description of the available options:

- `version`: The published version of the connector
- `name`: The name given to the instance of the connector when it is created
- `type`: The type of connector. This corresponds to the name of the docker image
  that this connector is published in, e.g. `infinyon/fluvio-connect-<type>`
- `topic`: The name of the Fluvio topic that this connector will produce to
- `create_topic`: Whether to create the topic if it does not exist
- `direction`: Whether the connector is a `source` or a `sink` connector
- `parameters`: An object that contains connector-specific parameters.
  Also, connectors that support SmartModules specify their SmartModule names here:
  - `filter`: The name of a Filter SmartModule to apply
  - `map`: The name of a Map SmartModule to apply
  - `arraymap`: The name of an ArrayMap SmartModule to apply

-> **Note**: Currently, `aggregate` and `filter-map` SmartModules are not supported in connectors.

When running `fluvio connector create`, pass the path to this file using the `--config`
option.

Example usage:

%copy first-line%
```bash
$ fluvio connector create --config=./connect.yml
```

---

## `fluvio connector list`

This command show you all the existing Managed Connectors in your cluster.

```
List all Managed Connectors

fluvio connector list [OPTIONS]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -O, --output <type>    Output [default: table]  [possible values: table, yaml, json]
```

Example usage:

%copy first-line%
```bash
$ fluvio connector list
 NAME       STATUS
 cat-facts  Running
```

---

## `fluvio connector delete`

This command deletes an existing Managed Connector.

```
Delete a Managed Connector

fluvio connector delete <name>

FLAGS:
    -h, --help    Prints help information

ARGS:
    <name>    The name of the connector to delete
```

Example usage:

%copy first-line%
```bash
$ fluvio connector delete cat-facts
```