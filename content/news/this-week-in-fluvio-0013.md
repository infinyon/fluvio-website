---
title: "This Week in Fluvio #13"
date: 2021-11-03
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## No new release

We didn't have a new release this week.

We're working on a bigger feature and it isn't quite ready. But we can talk about it.

### Coming soon: SmartModules

The SmartModules are what we are calling SmartStream code that can uploaded stored on SPUs.

Currently, using a SmartStream filter with a consumer on the CLI, you have to provide a local file path to your filter. Sometimes this can lead to very long commands.

```shell
$ fluvio consume my-topic --filter ./target/wasm32-unknown-unknown/release/path/to/my-data-filter.wasm 
```

Soon, you can store your SmartModules on the Fluvio SPUs and give it a name.

```shell
$ fluvio smartmodule create <name> --wasm-file ./target/wasm32-unknown-unknown/release/path/to/my-data-filter.wasm
```

Afterwards, you can apply your SmartModule by referring to it by its name.  

```shell
$ fluvio consume my-topic --filter <name>
```

Btw, this new functionality will be in addition to the existing behavior. You can continue using local wasm filters, but we think that if you are using the same filter frequently, you will find SmartModules to be a convenient option. Especially if you are using many devices using the same SmartModule.

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
[connectors]: /connectors