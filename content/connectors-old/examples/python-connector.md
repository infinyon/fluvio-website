---
title: Python connector with docker compose
---

This example `Dockerfile` packages our [example Python Fluvio project]({{<ref "/api/official/python/examples">}}) into a connector image.

### Dockerfile

{{<code file="embeds/client-examples/python/Dockerfile" lang="Dockerfile" copy=true >}}

This `docker-compose.yml` used with `docker compose` CLI starts our previously built connector image as a [local connector]({{<ref "/connectors-old/local-connectors">}})

### docker-compose.yml

{{<code file="embeds/client-examples/python/docker-compose.yml" lang="yaml" copy=true >}}

## Run example

{{% inline-embed file="embeds/client-examples/run-docker-compose-example.md" %}}

### Example output

```
[...]
Hello World! - Time is: 2022-11-10 02:35:10.143967
[...]
 ```