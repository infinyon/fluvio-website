---
title: Managed connectors with InfinyOn Cloud 
menu: Managed connectors 
---

## Managed Connectors

Managed Connectors are deployed within the Fluvio cluster, and are available when
using Fluvio via [InfinyOn Cloud] or when running in Kubernetes (e.g. via `fluvio cluster start`).
When using InfinyOn Cloud, Managed Connectors are recommended over Local Connectors.

When we launch a Managed Connector, we must define a configuration file (usually called
`connect.yml`), which gives all the required options for the connector instance.
Here's a sample `connect.yml` that will launch an HTTP connector that fetches cat facts
from a JSON API.

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
```

We can run this Managed Connector with the following command:

%copy first-line%
```bash
$ fluvio connector create --config=./connect.yml
```

Once the connector is created, we should be able to check that it exists using
the `list` command.

%copy first-line%
```bash
$ fluvio connector list
 NAME       STATUS
 cat-facts  Running
```

Now that our connector is up, we can check on the traffic coming into our topic!
Use the `fluvio consume` command to look at the `cat-facts` topic:

%copy first-line%
```bash
$ fluvio consume cat-facts -B
{"fact":"A cat almost never meows at another cat, mostly just humans. Cats typically will spit, purr, and hiss at other cats.","length":116}
{"fact":"In one stride, a cheetah can cover 23 to 26 feet (7 to 8 meters).","length":65}
{"fact":"Phoenician cargo ships are thought to have brought the first domesticated cats to Europe in about 900 BC.","length":105}
```

To get a connector's config, use `fluvio connector config <connector-name> [-o file-name.yaml]`.
Don't be alarmed if the output connector config yaml doesn't match identically.
Integer values may turn into Strings and values may lose their quotes if it's
not required. The `---` is [a yaml directive used to denote the start of the
document](https://yaml.org/spec/1.1/#YAML%20directive/). It's not needed but is
added by default when the connector is serialized. **The output from this command
is safe to be copied and used in a `fluvio connector create --config
./connect.yaml`**.

%copy first-line%
```bash
$ fluvio connector config cat-facts
---
name: cat-facts
type: http-source
topic: cat-facts
version: 0.3.0
parameters:
  endpoint: "https://catfact.ninja/fact"
  interval: 10s
```


To stop a connector, we can use `fluvio connector delete` and give it the name
of the connector we created, which in this case was "cat-facts".

%copy first-line%
```bash
$ fluvio connector delete cat-facts
```

