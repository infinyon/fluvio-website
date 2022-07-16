---
title: "This Week in Fluvio #39"
date: 2022-07-13
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

This Week in Fluvio is our weekly newsletter for development updates to [Fluvio open source].

Please subscribe to This Week in Fluvio to receive new posts in your inbox
{{< subscribe-button >}}


{{< banner >}}

---

## New release
* [Fluvio v0.9.30](https://github.com/infinyon/fluvio/releases/tag/v0.9.30)

## New features
* Add `fluvio connector config <connector-name>`  ([#2464](https://github.com/infinyon/fluvio/pull/2464))
* Add performance counters to producer ([#2424](https://github.com/infinyon/fluvio/issues/2424))

## Performance improvements
* Prefer ExternalIP to InternalIP if configured in kubernetes ([#2448](https://github.com/infinyon/fluvio/pull/2448))
* Move stream publishers to connection-level context ([#2452](https://github.com/infinyon/fluvio/pull/2452))
* Upgrade to fluvio-future 0.4.0 ([#2470](https://github.com/infinyon/fluvio/pull/2470))

## Upcoming events
* The date of our webinar has changed. It is now at Thu, Jul 28, 2022 10:00 AM - 10:30 AM PDT 
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
