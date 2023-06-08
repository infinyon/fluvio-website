---
title: Core Concepts 
---

SmartModules come equipped with pre-built intelligence and capabilities, making them readily usable even for users with limited technical expertise. By leveraging these modules, users can automate repetitive tasks, orchestrate data flows, and enable advanced functionalities without requiring deep technical knowledge or extensive coding.

By utilizing WebAssembly, SmartModules provide a secure and isolated environment for executing custom code. This ensures that the modules run safely within the Fluvio ecosystem, preventing potential security risks or conflicts with the underlying system. 

The beauty of SmartModules lies in their flexibility and adaptability. Users can configure them to suit their specific needs by adjusting parameters and settings. Naturally, producers and consumers can use SmartModules in order to shape data on both the client-side or server-side. This empowers individuals to tailor the behavior of the modules to match their unique requirements, all without writing complex code or dealing with intricate technical details.

## Using Multiple SmartModules

Multiple SmartModules can be activated in series. This process is commonly referred to as **chaining**.  The output of the previous SmartModule is used as the input of the next SmartModule. Therefore the ordering of the SmartModules in the chain is important.

## Examples

## Chaining with CLI

You can define a SmartModule chain to use with `fluvio` or `smdk` with a yaml file.


```yaml
transforms:
- uses: infinyon/jolt@0.3.0
  with:
    spec:
      - operation: shift
        spec:
          fact: "animal.fact"
          length: "length"
- uses: infinyon/regex-filter@0.1.0
  with:
    regex: "[Cc]at" 
```

You can pass this file to the CLI with the `--transforms-file` option.


#### Cloud Pipelines with SmartModule Chaining

Examples on how to use SmartModules in a pipeline:
* [Build HTTP to SQL Pipeline]
* [Build MQTT to SQL Pipeline]

[Build HTTP to SQL Pipeline]: {{<ref "/docs/tutorials/data-pipeline.md" >}}
[Build MQTT to SQL Pipeline]: {{<ref "/docs/tutorials/mqtt-to-sql.md" >}}
