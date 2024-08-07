---
title: "This Week in Fluvio #64"
date: 2024-08-07
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

---
We released **Fluvio 0.11.11** last week.

## New release
Fluvio **v0.11.11** is now available!

To update you can run `fvm update`

```bash
$ fvm update

info: Updating fluvio stable to version 0.11.11. Current version is 0.11.9.
info: Downloading (1/5): fluvio@0.11.11
info: Downloading (2/5): fluvio-cloud@0.2.25
info: Downloading (3/5): fluvio-run@0.11.11
info: Downloading (4/5): cdk@0.11.11
info: Downloading (5/5): smdk@0.11.11
done: Installed fluvio version 0.11.11
done: Now using fluvio version 0.11.11

```

If you don't have Fluvio in your machine run:

```
curl -fsS https://hub.infinyon.cloud/install/install.sh | bash
```

If you are enjoying Fluvio please share with your friends!

## New features

Notable changes in this new version:

- Added new argument to register a SPU with a public server local, using `--public-server-local`  or just `-l`.  A public server local allows configuration of additional network contexts when registering spus. 
Example:  `fluvio cluster spu register --id 5001 -p 0.0.0.0:9110 -l spu:9010 --private-server spu:9011`.

- The new  `smdk clean`  command to clean your project like `cargo clean`.
- More updates for an upcoming SDF release.

## Upcoming features
InfinyOn Stateful Data Flows is going to be in `Public Beta` soon. Stateful Data Flows is a new product that will allow you to build end to end stream processing data flows on Fluvio streams.

We have released 10 developer preview iterations and shared with 50 to 100 developers. If you'd like access to the private beta, please fill out [this form](https://infinyon.com/request/ss-early-access/).

## Bug fixes
This release includes a number of new features, bug fixes, documentation improvements, and improved error messaging.

See the [CHANGELOG](https://github.com/infinyon/fluvio/blob/v0.11.11/CHANGELOG.md) for details

## Good First Issues
We love our open source community contributors. Here are some issues that you could contribute to. All the best.

- [When a topic is deleted, connected clients should have their connection closed](https://github.com/infinyon/fluvio/issues/3836)
- [Remove localhost from fluvio in favor of 127.0.0.1](https://github.com/infinyon/fluvio/issues/3866)

## New blog post
- [How to send data reliably from IOT edge devices using Fluvio Mirroring](https://infinyon.com/blog/2024/07/use-mirroring-iot/)

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
