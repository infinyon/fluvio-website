---
title: "This Week in Fluvio #21"
date: 2022-02-02
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## New Release - Fluvio v0.9.18

This release was heavily focused on stability improvements

### Local Connector development fix

Previously, if you were [developing your own connector]({{<ref "/connectors/developer-guide/overview">}}), creating connectors would fail because during the creation of the connector pod, Kubernetes would always try to pull from Docker Hub.

You can control this behavior in your connector config through the `version` key. 

If `version` is `dev`, Kubernetes will expect an image in its *local registry* with a name matching the pattern `infinyon/fluvio-connect-<your connector name>:latest`.

Otherwise, the value of `version` will refer to the image tag to pull from Docker Hub.

e.g.
```shell
infinyon/fluvio-connect-<your connector name>:<version>
```

### Connector config parameter behavior change
Connector key parameters that include underscores in their name will covert the underscores into hyphens.

For example, in this example config

```yaml
# example-connector-config.yml
[...]
parameters:
  some_parameter: foo
```

The keys under `parameters` are used as CLI arguments to your connector config.

Prior to this release, the argument would render as `--some_parameter=foo`. 

Now the `some_parameter` key will will render as `--some-parameter=foo`

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions