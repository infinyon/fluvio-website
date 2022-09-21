---
title: Creating a Data Pipeline
menu: Creating a Data Pipeline
weight: 20
---

## Connectors and SmartModules

<img src="../images/create-pipeline.png"
     alt="execution flow of InfinyOn pipeline"
	 style="justify: center; max-width: 400px" />

This tutorial expects you to already have the Fluvio CLI installed, and InfinyOn
Cloud set up. If neither of these is the case, please follow the [previous tutorial](../cloud-setup)!

## Connectors

If you wish to automatically collect information from one source and send it to
Fluvio, Connectors are the way to go. When given the information on the
interface through the Connector configuration file, Fluvio can poll a multitude of
input types.

In this tutorial, we will be looking at the [HTTP Connector](/connectors/sources/http/) setup, connecting
to the <a href="https://catfact.ninja" target="_blank" rel="nofollow" > catfact.ninja</a> JSON database.

### Connector Config Layout

This is the template YAML connector file. To make it useful, it needs to be
populated – which we will do in the next step. See
[the documentation](/connectors/) for the parameters available for use.

{{<code file="code/yaml/connector-template.yaml" lang="yaml" copy="true">}}

Thankfully, filling it out is simple. For any connection, you need a name,
the connection type, and what topic to connect to.

For the HTTP-specific parameters you will need to specify the link it is
polling, and the interval at which it polls.

{{<code file="code/yaml/catfacts-basic-connector.yaml" lang="yaml" copy="true">}}

This creates a connector named `cat-facts`, that reads from the website
`https://catfacts.ninja/fact` every 30 seconds, and produces to the topic
`timekeeper-with-connector`.

### Testing the Connector

You can register the connector to Fluvio with `fluvio connector create <connector name> --config=<config-file.yaml>`

%copy first-line%
```bash
fluvio connector create --config=./catfact.yml
```

To see if it is running successfully, you have two options: `fluvio connector list`, and `fluvio consume`.

%copy first-line%
```bash
fluvio consume cat-facts -dT4
Consuming records starting 4 from the end of topic 'cat-facts'
{"fact":"A cat lover is called an Ailurophilia (Greek: cat+lover).","length":57}
{"fact":"British cat owners spend roughly 550 million pounds yearly on cat food.","length":71}
{"fact":"Fossil records from two million years ago show evidence of jaguars.","length":67}
{"fact":"Relative to its body size, the clouded leopard has the biggest canines of all animals\u2019 canines. Its dagger-like teeth can be as long as 1.8 inches (4.5 cm).","length":156}
```

To delete the Connector, use `fluvio connector delete <connector-name>`.

## SmartModules

SmartModules are user defined functions set to run on and modify the inputs/outputs to
a Fluvio database.

Want to filter so that only JSON records that match a specific priority tag are
recorded? A Filter SmartModule can be written to only let through records with
specific values.

You create a SmartModule by using Rust and generating it based on the SmartModule
template available [at the github repository](https://github.com/infinyon/fluvio-smartmodule-template).

### Making a SmartModule

If we want to clean up the timekeeper records and make it so that the messages
are marked separately from the JSON objects, we will need a SmartModule.
Specifically we will need a map SmartModule.

We need to go through some setup steps though.

First, check if `wasm32` is listed by `rustup` as an installed target.

%copy first-line%
```bash
$ rustup target list | grep installed
wasm32-unknown-unknown (installed)
x86_64-unknown-linux-gnu (installed)
```

-> If it is not installed, run `$ rustup target add wasm32-unknown-unknown` to install it.

Next, install `cargo-generate`, this may take a minute or two.

%copy first-line%
```bash
$ cargo install cargo-generate
```

Now you can download the template with `cargo-generate`.

-> You will have to fill in a couple of required arguments below, prefaced by the "🤷" emoji.

%copy first-line%
```bash
cargo generate --git https://github.com/infinyon/fluvio-smartmodule-template
🤷   Project Name : catfacts-map
🔧   Destination: /home/[...]/projects/[...]/catfacts-map ...
🔧   Generating template ...
✔ 🤷   Want to use SmartModule parameters? · true
✔ 🤷   Which type of SmartModule would you like? · map
[1/7]   Done: .cargo/config.toml
[2/7]   Done: .cargo
[3/7]   Done: .gitignore
[4/7]   Done: Cargo.toml
[5/7]   Done: README.md
[6/7]   Done: src/lib.rs
[7/7]   Done: src
🔧   Moving generated files into: `/home/[...]/projects/[...]/catfacts-map`...
💡   Initializing a fresh Git repository
✨   Done! New project created /home/[...]/projects/[...]/catfacts-map

```

There should now be a new directory labeled `catfacts-map` in your working directory.

Now we can edit the `catfacts-map/src/lib.rs` file to get what we need!

{{<code file="code/rust/catfacts-map/src/lib.rs" lang="rust">}}

Now that we have the SmartModule created, we need to link it to Fluvio, so that
it can be used. Otherwise you would have to remember the entire path to the
`wasm` file.

%copy first-line%
```bash
$ fluvio smart-module create catfacts-map --wasm-file="catfacts-map/target/wasm32-unknown-unknown/release/catfacts_map.wasm"
smart-module "catfacts-map" has been created.
```

You can test that it is working by adding it to the Connector config!

### Connecting to a Connector

To use a SmartModule with a Connector, add it to the Connector config.
Currently you have to specify which type of SmartModule you are using, so it
will be the form of `module-type: name-of-module`.

{{<code-highlight file="code/yaml/catfacts-map-connector.yaml" lang="yaml" lines=10 copy="true">}}

-> In the near future the config arguments be updated so that you only specify that you are using a module; Fluvio will soon be able to take care of determining which module type is being used.

Now it is almost ready, you just need to tell Fluvio to update the Connector;
or recreate it if you deleted it earlier. Use the `update` argument for `fluvio
connector` to update the Connector if it still exists.

%copy first-line%
```bash
$ fluvio connector update --config=./catfact.yml
```

If you deleted the Connector, you can easily recreate it with `fluvio connector create`.

At this point you can run `fluvio consume` again and see how things have changed.

%copy first-line%
```bash
$ fluvio consume cat-facts -dT 4
Consuming records starting 4 from the end of topic 'cat-facts'
[JSON] "The average lifespan of an outdoor-only cat is about 3 to 5 years while an indoor-only cat can live 16 years or much longer."
[JSON] "In 1987, cats overtook dogs as the number one pet in America (about 50 million cats resided in 24 million homes in 1986). About 37% of American homes today have at least one cat."
[JSON] "Cats should not be fed tuna exclusively, as it lacks taurine, an essential nutrient required for good feline health.  Make sure you have the proper Pet supplies to keep your cat happy and healthy."
[JSON] "Cat paws act as tempetature regulators, shock absorbers, hunting and grooming tools, sensors, and more"
```


And voilà! We have a database that takes inputs from the user and from a website, and displays it all prettylike for whoever is reading it!

## Check out these Other Tutorials

[Setup InfinyOn Cloud](../cloud-setup/)

## References

[Fluvio CLI Produce](/cli/commands/produce/)

[Fluvio CLI Consume](/cli/commands/consume/)

[Fluvio CLI topic](/cli/commands/topic/)

[Fluvio CLI profile](/cli/installation/profile/)

[Connectors](/connectors/)

[Smart Modules](/smartmodules/)

[Smart Module Rust API](https://docs.rs/fluvio-smartmodule/latest/fluvio_smartmodule/)
