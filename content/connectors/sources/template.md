---
title: Template Connector
menu: "Source: Template"
section: Source
toc: false
hidden: true
---

The TEMPLATE connector is a Source, which reads from an external data
provider and produces data to Fluvio.

## Overview

- Why is such a connector relevant?
- Why do you need it?

## Configuration Options

- What configuration options are required for this connector?
- What is an example configuration file with the most minimal working configuration?
- Are there any common options that are not required but good to know about?
- After highlighting the required and common options, give a complete list of options
- Be sure to point out the default values for any options with defaults.

Example:

In the prometheus connector, the user MUST provide a list of prometheus endpoints,
they MUST provide the name of a Fluvio Topic to produce to, and they MAY provide a time interval for scraping.

Show an example configuration, such as:

```yaml
# config.yml
version: v1
name: my-prometheus
type: prometheus
topic: prometheus-topic
direction: source
parameters:
endpoints:
- http://localhost:9090/metrics
interval_millis: 3600
```

## Data Events

- When this connector produces to a topic, what do the events look like?
- Give one or a few examples of pieces of data as they would appear in the topic.
- If there is any relevant documentation for the data format, link to it.
- If the data has a clear choice of Record Key, explain how the key is selected.
- Give an example of a record and show what the corresponding key looks like.

## Example Use-Case

- If this connector has an obvious or common use-case, describe how a user can utilize this connector to achieve it.
- For example, the Airbyte Postgres connector explains what CDC is, why it is useful, and how the user can get started-  with that use-case.
- Walk through the entire scenario from start to finish. Include all setup using the most commonly-available tools possible.
- E.g. describe how to set up a Postgres database using Docker. Be sure to include exact versions and supply any custom configurations so the setup is reproducible.

## Versions

- A table listing all versions of this connector in reverse chronological order (latest on top)
- Version + highlights (link to ..?)