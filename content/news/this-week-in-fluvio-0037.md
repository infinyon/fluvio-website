---
title: "This Week in Fluvio #37"
date: 2022-06-29
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

This Week in Fluvio is our weekly newsletter for development updates to [Fluvio open source].

Please subscribe to This Week in Fluvio to receive new posts in your inbox
{{< subscribe-button >}}


{{< banner >}}

---

## New release
* [Fluvio v0.9.28](https://github.com/infinyon/fluvio/releases/tag/v0.9.28)
* [Fluvio v0.9.29](https://github.com/infinyon/fluvio/releases/tag/v0.9.29)

## New features

* Display multi-word subcommand aliases in CLI help info ([#2033](https://github.com/infinyon/fluvio/issues/2033))
* Add filter-map support to SmartProducer ([#2418](https://github.com/infinyon/fluvio/issues/2418))


## Performance improvements
* Upgrade to Wasmtime 0.37 ([#2400](https://github.com/infinyon/fluvio/pull/2400))

## Bug fixes
* Allow Cluster diagnostics to continue even if profile doesn't exist  ([#2400](https://github.com/infinyon/fluvio/pull/2402))
* Add timeout when creating SPG ([#2364](https://github.com/infinyon/fluvio/issues/2411))

* Revert 0.9.28 updates to Connector yaml config ([#2436](https://github.com/infinyon/fluvio/pull/2436))
    * Soon after the `0.9.28` release, we discovered an issue that slipped past our CI. For those interested, the following are a preview of what changes are coming soon to connectors.
        * Add top level `producer` and `consumer` entries to connector yaml configurations. ([#2426](https://github.com/infinyon/fluvio/issues/2426))
        * Allow string, dictionaries and lists as options to `parameters` section in connector yaml. ([#2426](https://github.com/infinyon/fluvio/issues/2426))

## Developer experience improvements
* Log fluvio version and git rev on client creation ([#2403](https://github.com/infinyon/fluvio/issues/2403))
* Fix `wasi` functions binding relying on order ([#2428](https://github.com/infinyon/fluvio/pull/2428))

## Upcoming events
* We are hosting a webinar on Tue, Jul 12, 2022 10:00AM - 10:30AM (PDT)
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
