---
title: Cloud Connectors
menu: Connector
weight: 40
---

The `fluvio cloud connector` family of commands is used to create, delete adn troubleshoot Connectors in InfinyOn Cloud. The connectors can be inbound or outbound, as provisioned in the configuration file.

%copy first-line%
```bash
$  fluvio cloud connector -h
```

```
fluvio-cloud-connector 
View Fluvio Connector information

USAGE:
    fluvio-cloud connector <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    config    Show the connector spec
    create    Create a new Managed Connector
    delete    Delete a Managed Connector
    help      Print this message or the help of the given subcommand(s)
    list      List all Managed Connectors
    logs      View connector logs
    update    Create a new Managed Connector
```

## `fluvio cloud connector create`

This command is used to provision a new connector.

%copy first-line%
```bash
$  fluvio cloud connector create -h
```

```
Create a new Managed Connector

USAGE:
    fluvio-cloud connector create --config <CONFIG>

OPTIONS:
    -c, --config <CONFIG>    Name of connector
    -h, --help               Print help information
```

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

When running `fluvio connector create`, pass the path to this file using the `--config`
option.

Example usage:

%copy first-line%
```bash
$ fluvio cloud connector create --config=./cats.yaml
connector "cat-facts" (http-source) created
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