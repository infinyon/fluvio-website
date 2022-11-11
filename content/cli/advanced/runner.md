---
title: Run
weight: 10
---

The `fluvio run` commands are used to directly invoke Fluvio's cluster
components, the [Streaming Controller (SC)] and [Streaming Processing Unit (SPU)].
For most use-cases, you'll probably want to use the [`fluvio cluster`] commands,
which are simpler to use and invoke these runner commands for you. However, if
you want to manually administrate your own cluster and have fine-grained control
over the components, then you can do so using `fluvio run`.

[Streaming Controller (SC)]: {{< ref "/docs/architecture/sc" >}}
[Streaming Processing Unit (SPU)]: {{< ref "/docs/architecture/spu" >}}
[`fluvio cluster`]: {{< ref "/cli/local/cluster" >}}

## `fluvio run sc`

Directly invoke a Streaming Controller (SC) from the command line. 
An SC hosts a public server for interacting with clients, and a
private server for interacting with SPUs and other cluster components. Most of
the CLI options for the SC involve configuring these servers. See the
[SC Architecture] for more details.

[SC Architecture]: {{< ref "/docs/architecture/sc" >}}

{{% inline-embed file="embeds/cli/help/fluvio-run-sc.md" %}}

---

## `fluvio run spu`

Directly invokes a Streaming Processing Unit (SPU) from the command line.
An SPU has a public server which is used to stream data to and from Fluvio Clients, and
a private server which is used to communicate with an SC. Most of the CLI options for
the SPU involve configuring these servers. See the [SPU Architecture] for more details.

[SPU Architecture]: {{< ref "/docs/architecture/spu" >}}

{{% inline-embed file="embeds/cli/help/fluvio-run-spu.md" %}}
