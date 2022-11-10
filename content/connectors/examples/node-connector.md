---
title: Node connector with docker compose
---

This example `Dockerfile` packages our [example Node Fluvio project]({{<ref "/api/official/node/examples">}}) into a connector image.

### Dockerfile

{{<code file="embeds/client-examples/node/Dockerfile" lang="Dockerfile" copy=true >}}

This `docker-compose.yml` used with `docker compose` CLI starts our previously built connector image as a [local connector]({{<ref "/connectors/local-connectors">}})

### docker-compose.yml

{{<code file="embeds/client-examples/node/docker-compose.yml" lang="yaml" copy=true >}}

## Run example


{{% inline-embed file="embeds/client-examples/run-docker-compose-example.md" %}}