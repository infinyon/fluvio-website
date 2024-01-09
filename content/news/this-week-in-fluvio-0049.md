---
title: "This Week in Fluvio #49"
date: 2022-11-29
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

This Week in Fluvio is our weekly newsletter for development updates to [Fluvio open source].

Please subscribe to This Week in Fluvio to receive new posts in your inbox
{{< subscribe-button >}}


{{< banner >}}

---

## New release

* [Fluvio v0.10.1](https://github.com/infinyon/fluvio/releases/tag/v0.10.1)
* [Kafka outbound connector v0.3.1]({{< ref "/connectors/outbound/kafka" >}})

## New features

* [Chaining support]({{< ref "/docs/concepts/transformations-chain" >}})
  * Producer: ([#2753](https://github.com/infinyon/fluvio/pull/2753))
  * Consumer: ([#2759](https://github.com/infinyon/fluvio/pull/2759))
  * SMDK: ([#2784](https://github.com/infinyon/fluvio/pull/2784))
  * Fluvio CLI:  ([#2812](https://github.com/infinyon/fluvio/pull/2812))

SmartModule transformation chaining was [introduced in the last release]({{<ref "/news/this-week-in-fluvio-0048#smartmodule-chaining-preview" >}}) as a preview with our [SQL outbound connector]({{<ref "/connectors/outbound/sql">}}). 

In this release, support is now available to the Rust client, `fluvio` and `smdk` CLI, and connectors wit the keyword `transforms`.

To get familiar, check out the example configs from our tutorials.

* [HTTP to SQL tutorial]({{< ref "/docs/tutorials/data-pipeline" >}})
* [MQTT to SQL tutorial]({{< ref "/docs/tutorials/mqtt-to-sql" >}})


* Kafka outbound connector now supports SSL - For more info check out the [Kafka outbound connector docs]({{< ref "/connectors/outbound/kafka" >}})

## Bug fixes

* [Validate WASM payload before packaging](https://github.com/infinyon/fluvio/pull/2760)

## Developer experience improvements
* Make behavior with producing records from file consistent between `fluvio` and `smdk` - ([#2756](https://github.com/infinyon/fluvio/pull/2756))
* [New docs for our Certified SmartModule: Jolt]({{<ref "/smartmodules/certified/jolt" >}})

## Open positions
* [Head of Product Management](https://www.infinyon.com/careers/head-of-product-management) [Remote - Global]
* [SW Engineer (Cloud services)](https://www.infinyon.com/careers/cloud-engineer-mid-level) [Remote - Global]
* [Sr. Rust engineer (Infrastructure)](https://www.infinyon.com/careers/infrastructure-engineer-senior-level) [Remote - US]
* [Sr. SW engineer (Connectors)](https://www.infinyon.com/careers/connectors-old-engineer-senior-level) [Remote - Global]
* [Developer Advocate](https://www.infinyon.com/careers/developer-advocate-mid-senior-level) [Remote - US timezones +/- 5hr]
* [Solutions Architect](https://www.infinyon.com/careers/solutions-architect) [Remote - US timezones +/- 5hr]

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
