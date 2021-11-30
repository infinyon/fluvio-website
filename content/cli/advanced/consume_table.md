---
title: CLI Consumer Table formatting
weight: 10
---

This document covers two of the the CLI Consumer's output types.
* `--output table` which is a simple formatted table
* `--output full_table` which is a text-based user interface, with features such as live row-based updates, and column customization


To demonstrate the table output, we're going to use the following input

Example topic input

%copy%
```

```


## table

By default the top-level object keys will be used as the column names, sorted by alphabetical order. For more customizability, please use the `full_table` output

`fluvio consume <topic-name> --output table`

Example output:
```
```


## full_table

By default the top-level object keys will be used as the column names, sorted by alphabetical order. For more customizability, please use the `full_table` output

`fluvio consume <topic-name> --output full_table`

You can scroll with
* `up`/`down` arrow keys or the mouse scroll wheel to move one row at a time
* `Page up`/`Page down` to move 5 rows up/down at a time
* `Home` to move to the top of the table
* `End` to move to the bottom of the table
* `c` to clear the table state
* `q` or `ESC` to exit the table

### Customize the `full_table` table
To customize the `full_table` output, you can provide the name of your [`tableformat`]():


`fluvio consume <topic-name> --output full_table --tableformat <tableformat name>`