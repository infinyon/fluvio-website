---
title: SMDK - Generate a SmartModule
menu: Generate
weight: 30
toc: false
---

SMDK generate helps develpoers build a sample SmartModule project by answering a few simple quuestions. 

### Generate - Operation

SMDK generate commands runds a wizard and builds a sample project is a subdirectory. Let's get started:

%copy first-line%
```bash
$ smdk generate my-filter
Generating new SmartModule project: my-filter
fluvio-smartmodule-cargo-dependency => '"0.2.5"'
🔧   Destination: ~/smdk/my-filter ...
🔧   Generating template ...
✔ 🤷   Which type of SmartModule would you like? · filter
✔ 🤷   Will your SmartModule use init parameters? · false
Ignoring: /var/folders/5q/jwc86771549058kmbkbqjcdc0000gn/T/.tmpwdNuM4/cargo-generate.toml
[1/5]   Done: Cargo.toml
[2/5]   Done: README.md
[3/5]   Done: SmartModule.toml
[4/5]   Done: src/lib.rs
[5/5]   Done: src
🔧   Moving generated files into: `~/smdk/my-filter`...
💡   Initializing a fresh Git repository
✨   Done! New project created ~/smdk/my-filter
```

The generator created Rust project ready to compile:

```bash
$ tree 
.
├── Cargo.toml
├── README.md
├── SmartModule.toml
└── src
    └── lib.rs
```


Note the `SmartModule.toml` file. This file contains SmartModule parameters required to load the file in the Cluster or publish to SmartModule Hub. 

%copy first-line%
```bash
$ cat SmartModule.toml
[package]
name = "my-filter"
group = "aj"
version = "0.1.0"
apiVersion = "0.1.0"
description = ""
license = "Apache-2.0"

[[params]]
name = "input"
description = "input description"
```

#### Sections

* `package` is used to build the SmartModule FQDN (`aj/my-filter@0.1.0`) and the description to publish to SmartModule Hub. Note that the `group` is equivalent to the package owner in the Hub. 
* `params` defines the command line parameters by the SmartModule internal logic.

The project is ready to build and test. Checkout the next section for instructions.

### Next Steps

3. [Build and Test]
4. [Load to your Cluster]
5. [Publish to SmartMoudle Hub]


[Build and Test]: {{< ref "build-test" >}}
[Load to your Cluster]: {{< ref "load" >}}
[Publish to SmartMoudle Hub]: {{< ref "publish" >}}