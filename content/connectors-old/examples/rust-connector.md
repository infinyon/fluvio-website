---
title: Rust connector with docker compose
---

This example `Dockerfile` packages [example Rust Fluvio project]({{<ref "/api/official/rust/examples">}}) into a connector image.

### Dockerfile

{{<code file="embeds/client-examples/rust/Dockerfile" lang="Dockerfile" copy=true >}}

This `docker-compose.yml` used with `docker compose` CLI starts our previously built connector image as a [local connector]({{<ref "/connectors-old/local-connectors">}})

### docker-compose.yml

{{<code file="embeds/client-examples/rust/docker-compose.yml" lang="yaml" copy=true >}}

## Run example

{{% inline-embed file="embeds/client-examples/run-docker-compose-example.md" %}}

### Example output

```
[...]
Hello World! - Time is Thu, 10 Nov 2022 02:31:51 +0000
[...]
 ```