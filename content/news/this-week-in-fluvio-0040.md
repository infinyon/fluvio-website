---
title: "This Week in Fluvio #40"
date: 2022-07-27
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

This Week in Fluvio is our weekly newsletter for development updates to [Fluvio open source].

Please subscribe to This Week in Fluvio to receive new posts in your inbox
{{< subscribe-button >}}


{{< banner >}}

---

## New release
* [Fluvio v0.9.32](https://github.com/infinyon/fluvio/releases/tag/v0.9.32)

## New features
* Support async response in multiplexed socket. ([#2488](https://github.com/infinyon/fluvio/pull/2488))
* Rename `--smartmodule` option in `fluvio consume` to `--smart-module`. `--smartmodule is still an alias for backward compatibility. ([#2485](https://github.com/infinyon/fluvio/issues/2485))

## Performance improvements
* Drop write lock before async IO operations. ([#2490](https://github.com/infinyon/fluvio/pull/2490))
* Add `Clone` trait to `DefaultProduceRequest`. ([#2501](https://github.com/infinyon/fluvio/pull/2501))
* Add `AtMostOnce` and `AtLeastOnce` delivery semantics. ([#2503](https://github.com/infinyon/fluvio/pull/2503))

## Bug fixes
* Restrict usage of `--initial`, `--extra-params` and `--join-topic` in `fluvio consume`. Those options only should be accepted when using specific smartmodules. ([#2476](https://github.com/infinyon/fluvio/pull/2476))
* Keep serving incoming requests even if socket closed to write. ([#2484](https://github.com/infinyon/fluvio/pull/2484))

## Developer experience improvements
* Measure latency for stats using macro. ([#2483](https://github.com/infinyon/fluvio/pull/2483))

## Upcoming events
* The date of our webinar has changed. It is now at Tue, Aug 16, 2022 10:00 AM - 10:30 AM PDT 
    * [Enhance your Kafka Infrastructure with Fluvio](https://register.gotowebinar.com/register/7829882206451748624)

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
