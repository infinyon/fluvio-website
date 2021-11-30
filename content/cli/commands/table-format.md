---
title: Table Format 
weight: 20
---

If you use `fluvio consume --output full_table`


```
$ fluvio tableformat -h
fluvio-tableformat 0.0.0
Create a tableformat display specification

USAGE:
    fluvio tableformat <SUBCOMMAND>

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    create    Create a new TableFormat display
    delete    Delete a TableFormat display
    list      List all TableFormat display
    help      Prints this message or the help of the given subcommand(s)
```

Schema:
```yaml
type: object
required: ["name"]
properties:
name:
    type: string
    minimum: 1
    maximum: 100
inputFormat:
    type: string
    enum:
        - JSON
columns:
    type: array
    items:
        type: object
        properties:
            headerLabel:
                type: string
            keyPath:
                type: string
            display:
                type: boolean 
            primaryKey:
                type: boolean 
```

Example config:

name: "exampleformat"
inputFormat: "JSON"
columns:
  - keyPath: "key2"
  - keyPath: "key1"
