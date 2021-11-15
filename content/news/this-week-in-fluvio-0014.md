---
title: "This Week in Fluvio #14"
date: 2021-11-10
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

## No new release

We didn't have a new release this week.


### Coming soon: SmartConnectors

Last week we teased SmartModules. One of the things we've been able to do with SmartModules is integrate them with our Connectors. We're calling our data Connectors + SmartModules: SmartConnectors.

Using SmartConnectors is easy if you're already using SmartModules. You only need to add a reference to the SmartModule in your source or sink Connector config.

Let's look at an example. We're going to look at a SmartConnector from the perspective of the config

```yaml
Bring an example connector w/ a SmartModule being used
```

In this example, we're using the <named> connector to get <some live data set> and store it in a topic. But before we store it, we want to transform the data with a SmartModule.

We just need to make sure the SmartModule we're referencing has been created. Let's check with `fluvio smartmodule list`.

```shell
$ fluvio smartmodule list
<output>
```

Now we'll create our connector the usual way

```shell
$ fluvio connector create --config ./path/to/config.yaml
<outout>

$ fluvio connector list
<output, show running connector>
```

So lastly, we'll look at the data in the topic. Just for comparison, I'll show data that would have been produced both without the SmartModule and with the SmartModule.

```shell
Without SmartModule
```

```shell
With SmartModule
```

And we see clear differences. With the inclusion of the SmartModule, we've transformed our external data to our needs.

SmartConnectors are coming soon, so keep an eye out for a future release.

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
[connectors]: /connectors