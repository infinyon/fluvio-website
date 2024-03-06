---
title: "This Week in Fluvio #59"
date: 2024-03-06
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

---

We have moved away from a weekly updates. Fluvio open source project has grown significantly in terms of code and components. Our release cadence is divided into Fluvio Core, InfinyOn Cloud, Clients and SDKs. Moving forward our goal is to update the community with the relevant releases.

Today is such an occassion.

We released Fluvio 0.11.5 this week.

## New release
We are pleased share that Fluvio **v0.11.5** is now available!

Thank you to new contributors to the fluvio project:

- [k0i](https://github.com/k0i)
- [AF](https://github.com/jdafont)
- [ODudek](https://github.com/ODudek)
- [shylock](https://github.com/Shylock-Hg)
- [fraidev](https://github.com/fraidev)
- [Urbit-pilled](https://github.com/urbit-pilled)

To update you can run `fvm update`

```
$ fvm update
info: Updating fluvio stable to version 0.11.4. Current version is 0.11.5.
info: Downloading (1/5): fluvio@0.11.5
info: Downloading (2/5): fluvio-cloud@0.2.18
info: Downloading (3/5): fluvio-run@0.11.5
info: Downloading (4/5): cdk@0.11.5
info: Downloading (5/5): smdk@0.11.5
done: Installed fluvio version 0.11.5
done: Now using fluvio version 0.11.5

```

If you don't have Fluvio in your machine run:

```
curl -fsS https://hub.infinyon.cloud/install/install.sh | bash
```

If you are enjoying Fluvio please share with your friends!

## New features

We made the self hosted experience easier with the following:

- Hub access to public smartmodules and connectors no longer requires a cloud login
    - cdk, smdk, and fluvio hub commands should allow access to list and download public components
- Consumers connected to a topic will receive a notification and shut down if the connected topic is deleted
- Improvements to support async in our fluvio python client

## Upcoming features
InfinyOn Stateful Service Development Kit is 2 releases away from a beta release.

We have released 6 developer preview iterations and shared with 50 to 100 developers. If you'd like access to the private beta, please fill out [this form](https://infinyon.com/request/ss-early-access/).

## Bug fixes
This release includes a number of bug fixes, documentation improvements, and improved error messaging.

See the [CHANGELOG](https://github.com/infinyon/fluvio/blob/v0.11.5/CHANGELOG.md) for details

## New blog post
[Marvin Hansen](https://github.com/marvin-hansen) wrote this amazing blog after building with Fluvio. [Real-time Streaming Analytics with Fluvio, DeepCausality, and Rust](https://infinyon.com/blog/2024/02/fluvio-deep-causality-rs/)

## Good First Issues

All the best. Here are some issues that you could contribute to:

- [fvm switch fails on some systems with running local cluster](https://github.com/infinyon/fluvio/issues/3765)
- [MQTT Connector: Prefix auto generated Client ID to prevent connection drops](https://github.com/infinyon/fluvio/issues/3825)
- [Add new command fluvio cluster resume](https://github.com/infinyon/fluvio/issues/3810) (intermediate/expert difficulty)


---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
