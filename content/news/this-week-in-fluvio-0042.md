---
title: "This Week in Fluvio #42"
date: 2022-08-10
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

This Week in Fluvio is our weekly newsletter for development updates to [Fluvio open source].

Please subscribe to This Week in Fluvio to receive new posts in your inbox
{{< subscribe-button >}}


{{< banner >}}

---



## New release
* [Fluvio v0.9.33](https://github.com/infinyon/fluvio/releases/tag/v0.9.33)

## New features
* Added `DeliverySemantic` to `fluvio-cli`. ([#2508](https://github.com/infinyon/fluvio/pull/2508))
* SmartModule package: add missing metadata ([#2532](https://github.com/infinyon/fluvio/pull/2532))

## Bug fixes
* Prevent collisions between namespaces ([#2539](https://github.com/infinyon/fluvio/pull/2539))

## Developer experience improvements
* CLI: Added ability to delete multiple connectors, smart modules and topics with one command. ([#2427](https://github.com/infinyon/fluvio/issues/2427))
* Added `--use-k8-port-forwarding` option to `fluvio cluster start`. ([#2516](https://github.com/infinyon/fluvio/pull/2516))
* Added proxy support during packages installation ([#2535](https://github.com/infinyon/fluvio/pull/2535))
* Adds feedback and debug info to 'smart-module create' ([#2513](https://github.com/infinyon/fluvio/pull/2513))

## New blog post
* [Flexible JSON transformations in Rust](https://www.infinyon.com/blog/2022/08/fluvio-jolt-intro/)
  * [Alexander](https://github.com/galibey) presents a Rust beginner-friendly solution for processing streams of dynamic JSON records using our [fluvio-jolt](https://crates.io/crates/fluvio-jolt)crate

## Upcoming events
* Register for Webinar on Tue, Aug 16: [Enhance your Kafka Infrastructure with Fluvio](https://register.gotowebinar.com/register/7829882206451748624)

## Open positions
* [Sr Rust Engineer (Frontend)](https://www.infinyon.com/careers/cloud-ui-engineer-senior-level) [Remote - Global]
* [SW Engineer (Cloud services)](https://www.infinyon.com/careers/cloud-engineer-mid-level) [Remote - Global]
* [Sr SW engineer (Connectors)](https://www.infinyon.com/careers/connectors-engineer-senior-level) [Remote - Global]
* [Sr Rust engineer (Infrastructure)](https://www.infinyon.com/careers/infrastructure-engineer-senior-level) [Remote - US]
* [Developer Advocate](https://www.infinyon.com/careers/developer-advocate-mid-senior-level) [Remote - US timezones +/- 5hr]
* [Solutions Architect](https://www.infinyon.com/careers/solutions-architect) [Remote - US timezones +/- 5hr]



---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
