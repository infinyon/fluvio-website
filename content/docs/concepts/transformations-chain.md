---
title: Transformations
weight: 100
---

In Fluvio, the data transformation is done using [SmartModules](../../../smartmodules/) - user-defined functions compiled to WebAssembly (WASM). A group of SmartModules forms a transformation chain - sequential invocations of each SmartModule according to the order defined upon creation. The output of the first invocation becomes the input of the second, the output of the second becomes the input of the third, and so on.

Both Fluvio Producer and Fluvio Consumer support **Transformation Chaining**. However, the actual code execution happens in different places. For Producer, the transformation
happens before the data is sent to the topic on the SPU, inside the Producer's process, hence utilizing the resources of the client. For Consumer, the transformation takes place on SPU before the data is sent
to the Consumers.

**Transformation Chaining** is available on the following components:
 1. Fluvio Client
 2. Fluvio CLI
 3. SmartConnectors

 ### Configuration
 As we previously noted, each Transformation is a SmartModule. More precisely, it is a SmartModule plus some custom parameters.
 Most often, **Transformation Chaining** is configured in `yaml` config file.
 
Example:

{{<code file="embeds/transforms-misc/jolt-basic.yaml" lang="yaml" copy="true">}}

we have one transformation, which is a SmartModule named [`infinyon/jolt@0.3.0`]({{<ref "../../smartmodules/certified/jolt.md" >}}). 
The name must match a SmartModule previously downloaded to the Cluster:

%copy%
```bash
fluvio sm list
  SMARTMODULE              SIZE
  infinyon/jolt@0.3.0      608.4 KB
```


Everything under `with` section in the config is treated as `key/value` pairs and passed to SmartModule as init parameters. The value in that pair is a JSON-serialized string.
In our example, the SmartModule will receive one pair:

```
"spec" : "[{\"operation\":\"default\",\"spec\":{\"source\":\"http\"}}]"
```

This is an example of how this particular SmartModule process parameters (the code is just for demonstration and may differ from real implementation):

```rust
#[smartmodule(init)]
fn init(params: SmartModuleExtraParams) -> Result<()> {
    if let Some(raw_spec) = params.get("spec") {
        // raw_spec is now: "[{\"operation\":\"default\",\"spec\":{\"source\":\"http\"}}]"
        match serde_json::from_str(raw_spec) {
            Ok(spec) => {
                SPEC.set(spec).expect("spec is already initialized");
                Ok(())
            }
            Err(err) => {
                eprintln!("unable to parse spec from params: {:?}", err);
                Err(eyre::Report::msg(
                    "could not parse the specification from `spec` param",
                ))
            }
        }
    } else {
        Err(SmartModuleInitError::MissingParam("spec".to_string()).into())
    }
}
```

The value in parameters can be strings, maps, or sequences. In this pseudo example, all values are valid:

{{<code file="embeds/transforms-misc/value-types-example.yaml" lang="yaml" copy="true">}}

#### Using Multiple SmartModules

Multiple SmartModules can be activated in series. The output of the previous SmartModule is used as the input of the next SmartModule. Therefore the ordering of the SmartModules in the chain is important.

{{<code file="embeds/transforms-misc/chain-example.yaml" lang="yaml" copy=true >}}

In this example, the `jolt` transformation is performed first, then its output is the input the `regex-filter`.