---
title: TableFormat 
weight: 20
---

Table Format is used to customize the behavior of the Fluvio consumer output type [`full_table`]. 

[`full_table`]: {{< ref "#full_table" >}}

With `table-format`, you can control the column labels, column ordering and control which keys are primary for displaying your live event data as row updates.

%copy first-line%
```
$ fluvio table-format -h
fluvio-table-format 0.0.0
Create a table-format display specification

USAGE:
    fluvio table-format <SUBCOMMAND>

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    create    Create a new TableFormat display
    delete    Delete a TableFormat display
    list      List all TableFormat display
    help      Prints this message or the help of the given subcommand(s)
```

This is the schema for the Table Format yaml config used by `fluvio table-format create`

You only need to give your Table Format a name, and an input format (currently only JSON is supported)

### TableFormat Config schema

This is a definition of the TableFormat config schema. Below are the descriptions of each field of the config file.

Check out the [examples](#examples) section below to see a few different config files and their resulting table views.

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
            primaryKey:
                type: boolean 
```

#### Field descriptions
##### name
Required

This is the name of your Table Format. You'll see this name when you run `fluvio table-format list`, and you'll use this name with `fluvio consume topic-name --table-format <name>`

##### inputFormat
Required

The only supported option for this field is `"JSON"`

##### columns
optional array - The default column display will be the top-level keys (ordered alphabetically).

Each element references a key from input json object.

The ordering of each element is important, as it will be the order columns will be rendered.

##### keyPath
This is the only required column field. This should be a top-level key. If the key path doesn't exist, the column will print with no data.

##### headerLabel
optional - default uses key name. Override the label of the column.

##### primaryKey
optional - default false. If specified to true, rendering updates to the table will compare the values of primary keys to define a set. When new data matches an existing set, it's row will be updated. Otherwise it will append a new row to the table.

#### Examples

For the following examples, we'll start off with our topic data arriving in this order.

```json
{"key1":"a","key2":"1","key3":"Alice","id":123}
{"key1":"b","key2":"2","key3":"Bob","id":456}
{"key1":"c","key2":"3","key3":"Carol","id":789}
[{"key1":"x","key2":"10","key3":"Alice","id":123},{"key1":"y","key2":"20","key3":"Bob","id":456},{"key1":"c","key2":"30","key3":"Carol","id":789}]
```

##### Example 0

**No table-format**

Using the [`full_table`] output without using a table-format print each key into a column in alphabetical order from left to right.

%copy first-line%
```shell
$ fluvio consume event-data -B --output full_table
```

Output:

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


##### Example 1

**Display a subset of data**

In this example, we only want to display data for only 2 of the keys. The ordering of the columns will be `key1` first, then `key2`.

Config:

%copy%
```yaml
# exampleformat1.yaml
name: "exampleformat1"
inputFormat: "JSON"
columns:
  - keyPath: "key1"
  - keyPath: "key2"
```

Create the `table-format`:

%copy first-line%
```shell
$ fluvio table-format create --config exampleformat1.yaml
```

Display your table:

%copy first-line%
```shell
$ fluvio consume event-data -B --output full_table --table-format exampleformat1
```

Output:

```
┌('c' to clear table | 'q' or ESC to exit) | Items: 6─────────────────┐
│key1                               key2                              │
│a                                  1                                 │
│b                                  2                                 │
│c                                  3                                 │
│x                                  10                                │
│y                                  20                                │
│c                                  30                                │
└─────────────────────────────────────────────────────────────────────┘
```

##### Example 2

**Reorder columns**

In this example, we rearrange the order so that the columns will be ordered: `id`, `key3`, `key1`,  `key2`

Config:

%copy%
```yaml
# exampleformat2.yaml
name: "exampleformat2"
inputFormat: "JSON"
columns:
  - keyPath: "id"
  - keyPath: "key3"
  - keyPath: "key1"
  - keyPath: "key2"
```

Create the `table-format`:

%copy first-line%
```shell
$ fluvio table-format create --config exampleformat2.yaml
```

Display your table:

%copy first-line%
```shell
$ fluvio consume event-data -B --output full_table --table-format exampleformat2
```

Output:

```
┌('c' to clear table | 'q' or ESC to exit) | Items: 6─────────────────┐
│id                key3              key1              key2           │
│123               Alice             a                 1              │
│456               Bob               b                 2              │
│789               Carol             c                 3              │
│123               Alice             x                 10             │
│456               Bob               y                 20             │
│789               Carol             c                 30             │
└─────────────────────────────────────────────────────────────────────┘
```

##### Example 3

**Rename columns**

In this example, we're rearranging the order of the columns, and changing the column header to something more meaningful for our data.

Config:

%copy%
```yaml
# exampleformat3.yaml
name: "exampleformat3"
inputFormat: "JSON"
columns:
  - keyPath: "id"
    headerLabel: "ID"
  - keyPath: "key3"
    headerLabel: "Name"
  - keyPath: "key2"
    headerLabel: "Number"
  - keyPath: "key1"
    headerLabel: "Letter"
```

Create the `table-format`:

%copy first-line%
```shell
$ fluvio table-format create --config exampleformat3.yaml
```

Display your table:

%copy first-line%
```shell
$ fluvio consume event-data -B --output full_table --table-format exampleformat3
```

Output:

```
┌('c' to clear table | 'q' or ESC to exit) | Items: 6─────────────────┐
│ID                Name              Number            Letter         │
│123               Alice             1                 a              │
│456               Bob               2                 b              │
│789               Carol             3                 c              │
│123               Alice             10                x              │
│456               Bob               20                y              │
│789               Carol             30                c              │
└─────────────────────────────────────────────────────────────────────┘
```

##### Example 4: Choose primary key for row-updates

For event-sourced data, it may be beneficial to display the most up-to-date data by updating the row with current values. To do this, we select a primary key within the data.

When new data arrives, if the values at the primary key match, we replace the row with the more recent data.

Config:

%copy%
```yaml
# exampleformat4.yaml
name: "exampleformat4"
inputFormat: "JSON"
columns:
  - keyPath: "id"
    headerLabel: "ID"
    primaryKey: true
  - keyPath: "key3"
    headerLabel: "Name"
  - keyPath: "key2"
  - keyPath: "key1"
```

Command:

%copy first-line%
```shell
$ fluvio table-format create --config exampleformat4.yaml
```

Display your table:

%copy first-line%
```shell
$ fluvio consume event-data -B --output full_table --table-format exampleformat4
```

Output:

```
┌('c' to clear table | 'q' or ESC to exit) | Items: 3─────────────────┐
│ID                Name              key2              key1           │
│123               Alice             10                x              │
│456               Bob               20                y              │
│789               Carol             30                c              │
└─────────────────────────────────────────────────────────────────────┘
```