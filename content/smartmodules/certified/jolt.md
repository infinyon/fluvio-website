---
title: Jolt
weight: 20
---

Jolt provides JSON to JSON transformation

This is a [map-type]({{<ref "../transform/map.md" >}}) SmartModule that transforms JSON records leveraging [Fluvio Jolt](https://github.com/infinyon/fluvio-jolt) library, which has its own DSL (Domain Specific Language) to remove the need for coding simple transformations.

The transformations in **Jolt** are a set of operations that are sequentially performed over incoming records.

-> Jolt only works on JSON text records.

There are three main types of operations:
* **Shift** - move the field from one location to another
* **Default** - specify a default value for the field, if not present
* **Remove** - delete the field from the object

There can be a mix and match of transformations applied at the same time. Let's see below:

#### Specification example
```json
[
    {
      "operation": "remove", // remove field $.id from incoming JSON object
      "spec": {
        "id": ""
      }
    },
    {
      "operation": "shift", // move everything inside $.data
      "spec": {
        "*": "data.&0",
      }
    },
    {
      "operation": "default", // if $.data.source does not exist, add it with value "http-connector"
      "spec": {
        "data": {
            "source": "http-connector"
        }
      }
    }
]
```

The Jolt SmartModule applies the operations in sequence: `remove` followed by `shift` followed by `default`.

### Usage example
To demonstrate how **Jolt** SmartModule works we will use [SMDK]({{<ref "../smdk/build-test.md" >}}) tool. 

First, we need to download it to our cluster:

%copy first-line%
```shell
$ fluvio hub download infinyon/jolt@0.1.0
```

Second, we create a file `transform.yaml` with transformation specification defined above:

%copy%
```yaml
# transform.yaml
transforms:
  - uses: infinyon/jolt@0.1.0
    with:
      spec:
        - operation: remove
          spec:
            id: ""
        - operation: shift
          spec:
            "*": "data.&0"
        - operation: default
          spec:
            data:
              source: "http-connector"
 

```

Let's use `smdk test` to see it in action:


%copy first-line%
```shell
$ smdk test --text '{}' --transforms-file ./transform.yaml
1 records outputed
{"data":{"source":"http-connector"}}
```

%copy first-line%
```shell
$ smdk test --text '{"id":1, "name": "John Smith", "account": "1111" }' --transforms-file ./transform.yaml
{"data":{"account":"1111","name":"John Smith","source":"http-connector"}}
```

%copy first-line%
```shell
$ smdk test --text '{"id":1, "name": "John Smith", "account": "1111", "type": "custom" }' --transforms-file ./transform.yaml
{"data":{"account":"1111","name":"John Smith","source":"http-connector","type":"custom"}}
```

%copy first-line%
```shell
$ smdk test --text '{"id":1, "name": "John Smith", "source":"mqtt-connector" }' --transforms-file ./transform.yaml
{"data":{"name":"John Smith","source":"mqtt-connector"}}
```


For additional examples checkout the tutorials:
* [Build HTTP to SQL Pipeline]({{<ref "../../docs/tutorials/data-pipeline.md" >}})
* [Build MQTT to SQL Pipeline]({{<ref "../../docs/tutorials/mqtt-to-sql.md" >}})


