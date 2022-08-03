---
title: "This Week in Fluvio #41"
date: 2022-08-03
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

This Week in Fluvio is our weekly newsletter for development updates to [Fluvio open source].

Please subscribe to This Week in Fluvio to receive new posts in your inbox
{{< subscribe-button >}}


{{< banner >}}

---

## New release
* SmartModule development environment
  * SmartModule developers may use our [container-based development environment](https://github.com/infinyon/fluvio/blob/master/dev-tools/smartmodule.Dockerfile) which contains all the tools required to build their Rust code into WASM
  * Check out our [Dev tools](https://github.com/infinyon/fluvio/blob/master/dev-tools) for more information about how to get started
* Rust crate [fluvio-jolt](https://crates.io/crates/fluvio-jolt) 
  * This is a native Rust port of the [Java library of the same name](https://github.com/bazaarvoice/jolt)
  * JSON to JSON transformation where the "specification" for the transform is itself a JSON document
  * Compatible for use in SmartModules

## Feature Highlight
This feature was added included in the previous release but was not mentioned in [last week's issue]({{<ref "news/this-week-in-fluvio-0040">}}).

* Support for `at-least-once` and `at-most-once` in the Producer Client. ([#2481](https://github.com/infinyon/fluvio/issues/2481))
  * This feature introduces the notion of **Delivery Semantic** to Fluvio Producer. From now, you can choose in which manner you want
  your records to be transported from the producer to the SPU unit. It's either `at-most-once` guarantee or `at-least-once`.
  The first one is sending without waiting for the response, hence no reaction for errors. The latter one is sending and 
  retrying until succeeded (with certain assumptions). Check out more details in
  [Delivery Semantics]({{< ref "/docs/clients/producer#delivery-semantics" >}}) section.

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
