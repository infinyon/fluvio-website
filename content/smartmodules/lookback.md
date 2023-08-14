---
title: Lookback
---

The Lookback feature in Fluvio SmartModule is the way to access records from the data stream before the SmartModule starts allowing it to build its internal state depending on what the topic currently has.

If configured, the Lookback phase is guaranteed to take place after `init` method but before the first record is processed.

To activate Lookback SmartModule must fit the following criteria:  
- A function annotated with `#[smartmodule(look_back)]` macro is present in the code. This function is used to pass requested records to SmartModule:
```rust
#[smartmodule(look_back)]
pub fn look_back(record: &Record) -> Result<()> {
    ...
}
```
- `lookback` parameter is specified in transform configuration. It defines a set of lookback records that SmartModule receives. Supported ways to define the set:
    - by size (last N records existing in the topic)
    - by age (records that are younger than the specified age)
    - by size and age (max N records younger than the specified age)
```yaml
transforms:
  - uses: local/filter-with-lookback@0.1.0
    lookback:
      age: 30m # we want only records that are younger than 30 minutes
      last: 10 # we want maximum of 10 records (last)
```
---
If Fluvio topic is empty, `look_back` is never called.

-> If you start processing from an offset other than the end, you will receive records both as lookback record and as regular processing

-> Lookback is only supported for SmartModules that are run on SPU. This implies that Source Connectors don't support it.

## Examples

This section assumes that SMDK is [installed].


### Monotonically increasing values

This is an example of SmartModule that leverages Lookback functionality. It reads the last record from a topic and
only allows a record that is higher than the previous value.

{{<code file="embeds/smartmodules/lookback-example.md" lang="rust" copy=true >}}

Using this `transforms.yaml` config we define that we only need one last record:
{{<code file="embeds/smartmodules/lookback-transform-example.yaml" lang="yaml" copy=true >}}

#### Test with Fluvio Cli
First, let's generate a new SmartModule using SMDK tool:  

%copy first-line%
```bash
$ smdk generate
Using hub https://hub-dev.infinyon.cloud
🤷   Project Name: filter-with-lookback
✔ 🤷   Will your SmartModule be public? · false
✔ 🤷   Which type of SmartModule would you like? · filter
✔ 🤷   Will your SmartModule use init parameters? · false
[1/7]   Done: .gitignore
[2/7]   Done: Cargo.toml
[3/7]   Done: README.md
[4/7]   Done: SmartModule.toml
[5/7]   Done: rust-toolchain.toml                                                                                                                                                                                                               [6/7]   Done: src/lib.rs                                                                                                                                                                                                                        [7/7]   Done: src
```
Then, put the code snippet from above into `src/lib.rs` file.

Now we are ready to build and load SmartModule to the cluster:

%copy first-line%
```bash
$ smdk build
$ smdk load
```

Let's produce 3 records into our topic:

%copy first-line%
```bash
$ fluvio produce test_topic
> 1
Ok!
> 2
Ok!
> 3
Ok!
```

In another terminal, run Fluvio Consumer with `transforms.yaml` file from the example above. It will use our SmartModule with configured `lookback`:

%copy first-line%
```bash
$ fluvio consume test_topic  --transforms-file transforms.yaml
```

If you insert another record in Producer, Consumer will only output it only if it's greater than 3 (last record value when Consumer started).  


#### Test with SMDK tool
We will use `smdk test` subcommand to verify that our SmartModule works. 

By `--lookback-last 1` we specify lookback parameter to "read last 1 record".  
By `--record "N"` we specify records that will exist before the SmartModule start processing.

The following commands will pass records "2" and "3" to `look_back` and record "4" to `filter` function:

%copy first-line%
```bash
$ smdk test --text 4 --lookback-last 1 --record 2 --record 3
4
```
The output result is 4. But if we run:

%copy first-line%
```bash
$ smdk test --text 1 --lookback-last 1 --record 2 --record 3
```
the record will be filtered out as expected.

[installed]: {{< ref "smartmodules/smdk/install" >}}

