---
title: SMDK - Load to Cluster
menu: Load
weight: 50
toc: false
---


## Registering SmartModules with your cluster

After building a SmartModule as a WASM binary, it may be registered with Fluvio using
the `fluvio smart-module` command, providing a name and a path to the binary.

Use [SmartModule filters]({{<ref "/smartmodules/apis/filter" >}}) to build a WASM file.

%copy first-line%
```bash
$ fluvio smart-module create my-filter --wasm-file ./target/wasm32-unknown-unknown/release/my_filter.wasm
```

After creating one or more SmartModules, one may use the `fluvio smart-module list` command
to see the available SmartModules:

%copy first-line%
```bash
$ fluvio smart-module list
   NAME       STATUS             SIZE
   my-filter  SmartModuleStatus  90442
```

## Using SmartModules

### SmartModules with Consumers

#### Using Registered SmartModules
You may use a Registered SmartModule anywhere that SmartModules may be used. To use them,
you'll need to provide the name of the SmartModule as well as its type. 

For example, if we want to apply our registered SmartModule `my-filter` while consuming from our topic `my-topic`,
provide it's name to the `--filter` argument.

%copy first-line%
```bash
$ fluvio consume my-topic -B --filter my-filter
```


### In Connectors

For our [official source and sink connectors]({{<ref "/connectors/">}}) you can apply SmartModules can be applied to any `source` or `sink` connector.

You just need to provide the type of module (`filter`, `map`, `array-map`, `filter-map`, `aggregate`) and it's registered name as a parameter.

For example, this is how you would define a `filter` type SmartModule named `my-filter` to use with our [http source connector]({{<ref "/connectors/inbound/http">}}), to apply the filter to each HTTP response before publishing to the topic:

%copy%
```yaml
# connect.yml
version: 0.3.0
name: cat-facts
type: http-source
topic: cat-facts
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 30s
  filter: my-filter 
```


### Next Steps

5. [Publish to SmartMoudle Hub]

[Publish to SmartMoudle Hub]: {{< ref "publish" >}}