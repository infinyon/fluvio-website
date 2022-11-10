---
title: Java connector with docker compose
---

This example `Dockerfile` packages our [example Java Fluvio project]({{<ref "/api/official/java/examples">}}) into a connector image.

### Dockerfile

{{<code file="embeds/client-examples/java/Dockerfile" lang="Dockerfile" copy=true >}}

This `docker-compose.yml` used with `docker compose` CLI starts our previously built connector image as a [local connector]({{<ref "/connectors/local-connectors">}})

### docker-compose.yml

{{<code file="embeds/client-examples/java/docker-compose.yml" lang="yaml" copy=true >}}

## Run example


{{% inline-embed file="embeds/client-examples/run-docker-compose-example.md" %}}