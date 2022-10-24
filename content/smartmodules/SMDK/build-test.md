---
title: SMDK - Build & Test a SmartModule
menu: Build & Test
weight: 40
toc: false
---

### Create a new project

Using `cargo generate`, you can answer a few prompts and generate the code for a SmartModule

%copy first-line%
```bash
$ cargo generate gh:infinyon/fluvio-smartmodule-template
```

Example:

We are creating a `filter` type of SmartModule, named `my-filter`

%copy first-line%
```bash
$ cargo generate gh:infinyon/fluvio-smartmodule-template
🤷   Project Name : my-filter
🔧   Generating template ...
✔ 🤷   Which type of SmartModule would you like? · filter
✔ 🤷   Want to use SmartModule parameters? · true
[1/7]   Done: .cargo/config.toml
[2/7]   Done: .cargo
[3/7]   Done: .gitignore
[4/7]   Done: Cargo.toml
[5/7]   Done: README.md
[6/7]   Done: src/lib.rs
[7/7]   Done: src
🔧   Moving generated files into: `/home/User/my-filter`...
💡   Initializing a fresh Git repository
✨   Done! New project created /home/User/my-filter
```

Navigate to your SmartModule directory, make your changes, then compile:

%copy first-line%
```bash
$ cargo build --release
```

Now that we have the SmartModule binary compiled let's see it in action.