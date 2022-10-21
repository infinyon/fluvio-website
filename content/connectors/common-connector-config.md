---
title: Common connector config
menu: Common config 
---
### Common Connector Arguments

For our rust connectors we have a set of parameters that are the same for all connectors. Below are the keys and their descriptions:

* `aggregate` - Path of aggregate smartmodule used as a pre-produce step. If the value is not a path to a file, it will be used to lookup a SmartModule by name
* `aggregate-initial-value` - The initial value for the aggregate smartmodule
* `arraymap` - Path of arraymap smartmodule used as a pre-produce step. If the value is not a path to a file, it will be used to lookup a SmartModule by name
* `filter` -  Path of filter smartmodule used as a pre-produce step. If the value is not a path to a file, it will be used to lookup a SmartModule by name
* `filter-map` - Path of `filter-map` smartmodule used as a pre-produce step. If the value is not a path to a file, it will be used to lookup a SmartModule by name
* `fluvio-partition` - The fluvio partition to consume with
* `map` - Path of map smartmodule used as a pre-produce step. If the value is not a path to a file, it will be used to lookup a SmartModule by name
* `rust-log` - The rust log level. If it is not defined, `RUST_LOG` environment variable will be used. If environment variable is not defined, then INFO level will be used.


### Producer Options

Adding a top level `producer` section to a given yaml has the following key options:
* `linger` - the amount of time an inbound connector should wait before publishing
to a fluvio topic. This is of the form `1s`, `1m`, `1 ms`, etc.
* `batch-size` - the size of the batches for the inbound connector. This is of the
form `1B`, `2KB`, `3MB`, etc. This allows for more throughput into a fluvio
topic.
* `compression` - This is an enum with options of `gzip`, `snappy`, or `lz4`
for the different compression types that fluvio supports.

These fields are all optional as well as the `producer` field itself.

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
  interval: 10s
  map: "catfact-map"
producer:
  linger: 1ms
  compression: gzip
  batch-size: 1 MB
```

### Consumer Options

Adding a top level `consumer` section to a given yaml has the following options:

* `partition` - The fluvio partition that an outbound connector uses.
