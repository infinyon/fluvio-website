%copy first-line%
```bash
$ cdk generate
🤷   Project Name: my-connector
🔧   Destination: ~/my-connector ...
🔧   project-name: my-connector ...
🔧   Generating template ...
✔ 🤷   Will your Connector be public? · false
🤷   Please set a group name: acme
✔ 🤷   Which type of Connector would you like [source/sink]? · source
Ignoring: /var/folders/r8/4x6_d2rn283946frzd1gc1pr0000gn/T/.tmptToFV3/cargo-generate.toml
[1/6]   Done: Cargo.toml             
[2/6]   Done: Connector.toml
[3/6]   Done: sample-config.yaml
[4/6]   Done: src/config.rs
[5/6]   Done: src/main.rs
[6/6]   Done: src
🔧   Moving generated files into: `~/my-connector`...
💡   Initializing a fresh Git repository
✨   Done! New project created ~/my-connector
```