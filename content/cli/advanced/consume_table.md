---
title: Consume output table formatting
weight: 10
---

This document covers two of the the CLI Consumer's output types.
* [`--output table`]({{< ref "#table" >}}) which is a simple formatted table
* [`--output full_table`]({{< ref "#full_table" >}}) which is a text-based user interface, with features such as live row-based updates, and column customization via [`table-format`]


To demonstrate the table output, we're going to use the following input

Example initial topic input

%copy%
```json
{"key1":"a","key2":"1","key3":"Alice","id":123}
{"key1":"b","key2":"2","key3":"Bob","id":456}
{"key1":"c","key2":"3","key3":"Carol","id":789}
[{"key1":"x","key2":"10","key3":"Alice","id":123},{"key1":"y","key2":"20","key3":"Bob","id":456},{"key1":"c","key2":"30","key3":"Carol","id":789}]
```

## table

By default the top-level object keys will be used as the column names, sorted by alphabetical order. For more customizability, please use the [`full_table`] output

[`full_table`]: {{< ref "#full_table" >}}

Example command:

%copy first-line%
```shell
$ fluvio consume example-topic --output table -B
```

Example output:
```
 id  | key1 | key2 | key3 
 123 | a    | 1    | Alice
 456 | b | 2 | Bob
 789 | c | 3 | Carol
 123 | x | 10 | Alice
 456 | y | 20 | Bob
 789 | c | 30 | Carol
```


## full_table

By default the top-level object keys will be used as the column names, sorted by alphabetical order.

Example command:

%copy first-line%
```shell
$ fluvio consume example-topic --output full_table -B
```

Example output:
```
┌('c' to clear table | 'q' or ESC to exit) | Items: 6─────────────────┐
│id                key1              key2              key3           │
│123               a                 1                 Alice          │
│456               b                 2                 Bob            │
│789               c                 3                 Carol          │
│123               x                 10                Alice          │
│456               y                 20                Bob            │
│789               c                 30                Carol          │
└─────────────────────────────────────────────────────────────────────┘
```

You can scroll with
* `up`/`down` arrow keys or the mouse scroll wheel to move one row at a time
* `Page up`/`Page down` to move 5 rows up/down at a time
* `Home` to move to the top of the table
* `End` to move to the bottom of the table
* `c` to clear the table state
* `q` or `ESC` to exit the table

### Customize the `full_table` table
You may have json data that isn't most effectively displayed with the keys ordered alphabetically. Or your data is event sourced, and you only want to see the most recent data organized by one or more primary keys.

In that case, to customize the `full_table` output, you can provide the name of your [`table-format`].

[`table-format`]: {{< ref "/cli/commands/table-format.md" >}}

`fluvio consume <topic-name> --output full_table --table-format <table-format name>`

For more information about how to use `table-format` to customize your table display (including how to rename and/or rearrange columns, or how to configure primary keys for row updating)