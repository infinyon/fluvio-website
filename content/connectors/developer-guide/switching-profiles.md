---
title: Connectors and Fluvio profiles
hidden: true
---
### About Switching Profiles

When you launch a Local connector, it will use the active profile at the time of startup
in order to determine which Fluvio cluster to connect to. If you switch active profiles,
running connectors will remain connected to the same cluster, they will not automatically
switch over. This is a good thing, because it prevents you from accidentally mixing up your
data sources and destinations by just changing profiles.

However, if you _do_ want to change the Fluvio cluster that a Local connector is using,
you'll need to stop the connector, switch profiles, then re-start the connector:

```bash
$ docker kill my-http
$ fluvio profile switch other-profile
$ docker run -d --name="my-http" \
    -v"$HOME/.fluvio/config:/home/fluvio/.fluvio/config" \
    -t infinyon/fluvio-connect-http:latest \
    -- \
    --endpoint="https://catfact.ninja/fact" \
    --fluvio-topic="cat-facts" \
    --interval=10s
```

