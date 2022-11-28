---
title: JSON to JSON Transformations
menu: Jolt
weight: 10
---

This is a certified by InfinyOn [map-type]({{<ref "../transform/map.md" >}}) SmartModule that transforms JSON records leveraging [Fluvio Jolt](https://github.com/infinyon/fluvio-jolt) library, which has its own DSL (Domain Specific Language) in order to facilitate its narrow job.

The transformations in **Jolt** are a set of operations that are sequentially performed over an incoming record modifying that record.

-> Records in JSON text format are only supported

There are three main types of operations:
* **Shift** - move the field from one location to another
* **Default** - specify a default value for the field, if not existed
* **Remove** - delete the field from the object

There can be any amount of operations of any type in the transformation specification.

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

The output of this transformation will be the result of operation `remove` followed by operation `shift` followed by operation `default`.

### Usage example
To demonstrate how **Jolt** SmartModule works we will use [SMDK]({{<ref "../smdk/build-test.md" >}}) tool. 

First, we need to download it to our cluster:

%copy first-line%
```shell
$ fluvio hub download infinyon/jolt@0.1.0
```

Second, we need to create a file `transform.yaml` with transformation specification that we already have:

%copy first-line%
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

Now, we can test any records against our specification:


%copy first-line%
```shell
$ smdk test --text '{}' --transforms-file ./transforms.yaml
1 records outputed
{"data":{"source":"http-connector"}}
```

%copy first-line%
```shell
$ smdk test --text '{"id":1, "name": "John Smith", "account": "1111" }' --transforms-file ./transforms.yaml
1 records outputed
{"data":{"account":"1111","name":"John Smith","source":"http-connector"}}
```

%copy first-line%
```shell
$ smdk test --text '{"id":1, "name": "John Smith", "account": "1111", "type": "custom" }' --transforms-file ./transforms.yaml
1 records outputed
{"data":{"account":"1111","name":"John Smith","source":"http-connector","type":"custom"}}
```

%copy first-line%
```shell
$ smdk test --text '{"id":1, "name": "John Smith", "source":"mqtt-connector" }' --transforms-file ./transforms.yaml
1 records outputed
{"data":{"name":"John Smith","source":"mqtt-connector"}}
```
