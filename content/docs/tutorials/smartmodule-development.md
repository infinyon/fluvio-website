---
title: SmartModule Development
weight: 20
hidden: true
---


This tutorial expects you to already have the Fluvio CLI installed, and InfinyOn
Cloud set up. If neither of these is the case, please follow the [setup tutorial]({{<ref "/docs/tutorials/cloud-setup.md">}})!

There are four main steps for this tutorial:
* Installing the SmartModule Development Kit CLI, `smdk`
* Generating a SmartModule project 
* Testing the SmartModule behavior with inputs
* Loading the SmartModule into a cluster

## SmartModules

SmartModules are user defined functions set to run on and modify the inputs/outputs to
a Fluvio database.

In this tutorial, we will create a configurable SmartModule that takes in a regular expression rule, and filters input based on whether it matches the rule.

### Installing SmartModule Development Kit CLI 

This first example will demonstrate creating, loading, and testing a SmartModule that filters based on whether the content matches a regular expression rule.

We will need a custom map SmartModule project, which we can generate with the SmartModule Development Kit CLI, `smdk`.

You can install `smdk` with the Fluvio CLI

%copy first-line%
```shell
$ fluvio install smdk
```

### Generate a new SmartModule project

With `smdk`, you can generate a new SmartModule project with the guided wizard. We're going to create a project named `regex_filter` in our current directory by running `smdk generate regex_filter`

%copy first-line%
```shell
$ smdk generate regex_filter
Generating new SmartModule project: regex_filter
fluvio-smartmodule-cargo-dependency => '0.3.0'
⚠️   Renaming project called `regex_filter` to `regex-filter`...
✔ 🤷   Which type of SmartModule would you like? · filter
🤷   Please set a group name : my-group
✔ 🤷   Will your SmartModule use init parameters? · true
✔ 🤷   Will your SmartModule be public? · false
Ignoring: /var/folders/r8/4x6_d2rn283946frzd1gc1pr0000gn/T/.tmpQXKwnh/smartmodule/cargo_template/cargo-generate.toml
[1/5]   Done: Cargo.toml
[2/5]   Done: README.md
[3/5]   Done: SmartModule.toml
[4/5]   Done: src/lib.rs
[5/5]   Done: src
🔧   Moving generated files into: `/home/user/project/regex_filter`...
💡   Initializing a fresh Git repository
✨   Done! New project created /home/user/project/regex_filter
hub: hubid my-group is set
hubid set to my-group
```

Navigate to the project directory and take a look at the `Cargo.toml` file:

%copy first-line%
```shell
$ cd regex-filter && cat Cargo.toml
```

You will need to add a couple of dependencies:

{{<code-highlight file="embeds/tutorials/sm-development/regex-filter/Cargo.toml" lang="rust" lines="14-15" copy="true">}}

And copy/paste this code in place of the auto-generated code in `lib.rs` :

{{<code file="embeds/tutorials/sm-development/regex-filter/src/lib.rs" lang="rust" copy="true">}}

Now that we have the SmartModule project created and code written, we need to build it.

### Build SmartModule

Building a SamartModule is trivial:

%copy first-line%
```bash
$ smdk build
```

### Test SmartModule

Using `smdk test`, we’ll use a simple regular expression that will return when the record contains only numbers.


#### Filter pass example

We expect the input `42` will return because there are only all numbers

%copy first-line%
```bash
$ smdk test -e regex="^[0-9]*$" --text 42
project name: "regex-filter"
loading module at: target/wasm32-unknown-unknown/release-lto/regex_filter.wasm
1 records outputed
42
```


#### Filter drop example 1 

We expect the input `abc` will drop because there are no numbers

%copy first-line%
```bash
$ smdk test -e regex="^[0-9]*$" --text abc
project name: "regex-filter"
loading module at: target/wasm32-unknown-unknown/release-lto/regex_filter.wasm
0 records outputed
```

#### Filter drop example 2

We expect the input `abc123` will also drop because there are letters mixed with numbers.

You can specify the input data in different ways, lets try passing `abc123` via `stdin` this time - the `--stdin` flag is specially useful when we're consuming data from an API and want to pipe the response to `smdk test`:

%copy first-line%
```bash
$ echo -n abc123 | smdk test -e regex="^[0-9]*$" --stdin
project name: "regex-filter"
loading module at: target/wasm32-unknown-unknown/release-lto/regex_filter.wasm
0 records outputed
```


### Load package into cluster 

In the previous steps, we used `smdk generate` to a SmartModule package. This is what the `SmartModule.toml` package metadata looks like.

{{<code file="embeds/tutorials/sm-development/regex-filter/SmartModule.toml" lang="toml">}}

%copy first-line%
```shell
$ smdk load 
Loading package at: /home/user/project/regex_filter
Found SmartModule package: regex-filter
loading module at: /home/user/project/regex_filter/target/wasm32-unknown-unknown/release-lto/regex_filter.wasm
Trying connection to fluvio router.infinyon.cloud:9003
Creating SmartModule: regex-filter
```

When you list the SmartModules in your cluster, you'll see that the `regex-filter` results match the name, version and group from the `SmartModule.toml` file.

%copy first-line%
```shell
$ fluvio sm list
  SMARTMODULE                      SIZE     
  my-group/regex-filter@0.1.0      316.4 KB 
```

With the SmartModule loaded in the cluster, we will test that our filter works with data from a topic.

This basic example we'll create a new topic, and load it with values. 

Create a file `values.txt` with the following
contents:

```txt
$ cat values.txt
42
abc
abc123
```

Create a topic `filter-test`:

%copy first-line%
```bash
$ fluvio topic create filter-test 
```

Load `values.txt` file to `filter-test` topic:

%copy first-line%
```bash
$ fluvio produce filter-test -f values.txt
```

%copy first-line%
```bash
$ fluvio consume filter-test -dB --smartmodule my-group/regex-filter@0.1.0 -e regex="^[0-9]*$"
Consuming records from the beginning of topic 'filter-test'
42
```

Or you can just use the SmartModule by name, if it is unique

%copy first-line%
```bash
$ fluvio consume filter-test -dB  --smartmodule regex-filter -e regex="^[0-9]*$" 
Consuming records from the beginning of topic 'filter-test'
42
```

You now know the development workflow for SmartModules with `smdk`. You can now generate your own project that processes data. For additional information on how to publish and share SmartModules checkout [SmartModule Hub].


## Check out these Other Tutorials

* [Setup InfinyOn Cloud]({{<ref "/docs/tutorials/cloud-setup.md">}})
* [Creating a Data Pipeline]({{<ref "/docs/tutorials/data-pipeline.md">}})

## References

* [Fluvio CLI Produce]({{<ref "/cli/client/produce.md">}})
* [Fluvio CLI Consume]({{<ref "/cli/client/consume.md">}})
* [Fluvio CLI topic]({{<ref "/cli/cluster/topic.md">}})
* [SmartModule]({{<ref "/smartmodules/">}})

[SmartModule Hub]: {{< ref "/smartmodules/hub/overview" >}}
