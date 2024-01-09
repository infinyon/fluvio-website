---
title: SMDK - Load to Cluster
menu: Load
weight: 60
toc: false
---

SMDK load give developers the ability to upload their own SmartModules to a local or [InfinyOn Cloud] clusters. 

This section assumes that `my-filter` project has been [built].

### Load - Operation

From your `my-filter` directory and use the `load` command:

%copy first-line%
```bash
$ smdk load
Loading package at: ~/smdk/my-filter
Found SmartModule package: my-filter
loading module at: ~/smdk/my-filter/target/wasm32-unknown-unknown/release-lto/my_filter.wasm
Trying connection to fluvio router.infinyon.cloud:9003
Creating SmartModule: my-filter
```

The SmartMoudule is uploaded to the cluster that your [`current profile`]. If you want to upload the SmartModule to multiple clusters, just switch the profile and run the `load` command again.

#### Inspect Result

Ensure that your SmartModule has been uploaded by running the following command :

%copy first-line%
```bash
$ fluvio smartmodule list
  SMARTMODULE                  SIZE     
  aj/my-filter@0.1.0            85.7 KB
```

SmartModules can be used in `Consumers` and `Connectors`. Future releases will expand this capability to other system components.

### Test with Consumers

SmartModules are applied to consumers with the `--smartmodule` argument. Let's setup a topic and produce some random text.

Create a topic:

%copy first-line%
```bash
$ fluvio topic create test
topic "test" created
```

Produce random data:

%copy first-line%
```bash
$ fluvio produce test 
> cats
Ok!
> dogs
Ok!
> start
Ok!
> stop
Ok!
> ^C
```

Consume using `my-filter`:

%copy first-line%
```bash
$ fluvio consume test -dB --smartmodule aj/my-filter@0.1.0
Consuming records from the beginning of topic 'test'
cats
start
```

The `-B` tells the consumer to read from the beginning of the stream, and `-d` disables continuous reading.


### Test with Cloud Connectors

SmartModule are applied to InfinyOn Cloud Connectors using `fluvio cloud connector` command. In this example we use the `http-source` connector with `transforms` to invoke our filter.

Copy the following connector configuration into a`connector.yml` file:

%copy%
```yaml
version: 0.3.0
name: cat-facts
type: http-source
topic: cats
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10s
transforms:
  - uses: acme/my-filter@0.1.0
    invoke: filter
```

Create a connector using the configuration file.

%copy first-line%
```bash
$ fluvio cloud connector create --config connector.yaml
connector "cat-facts" (http-source) created
```

The conector creates a topic called `cats` and filters out all records without letter `a`. Note, this exammple shows a general use case rather than a practical use case (as virtually all entries will have the letter 'a').


### Steps

1. [Generate a SmartModule]({{< ref "generate" >}})
2. [Build and Test]({{< ref "build-test" >}})
3. **[Load to your Cluster]({{< ref "load" >}})**
4. [Publish to SmartModule Hub]({{< ref "publish" >}})

[InfinyOn Cloud]: https://infinyon.cloud
[built]: {{< ref "build-test/#build---operation" >}}
[`current profile`]: {{< ref "cli/client/profile" >}}