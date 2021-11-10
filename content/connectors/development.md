---
title: Connector Development
menu: Connector
weight: 50
---
Writing a new connector is meant to be easy. To write a connector, you must
follow a few conventions. You may write your connector in any language which
has a fluvio client library.

## Adding a new connector

A new connector

### `metadata` subcommand
A connector should have a `metadata` command which prints a json schema of the
commandline arguments. This is the command we use to build our connectors
library and validate arguments passed to a connector.
