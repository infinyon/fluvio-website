---
title: "This Week in Fluvio #36"
date: 2022-06-15
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

This Week in Fluvio is our weekly newsletter for development updates to [Fluvio open source].

Please subscribe to This Week in Fluvio to receive new posts in your inbox
{{< subscribe-button >}}


{{< banner >}}

## New release
* New Kafka Source and Sink connectors
  * ~~Kafka Source docs~~ (Support discontinued since Fluvio v0.10.0)
  * [Kafka Sink docs]({{<ref "/connectors/outbound/kafka.md">}})

## Developer experience improvements
The Fluvio Java client is now [hosted in Maven Central](https://search.maven.org/artifact/com.infinyon/fluvio), which should reduce the friction for Java developers to install.

## InfinyOn Cloud updates
We've added the capability to query your account's [CPU and memory usage via Cloud CLI]({{< ref "cli/cloud/usage" >}})

## New blog post
* [Handling JSON data in Fluvio SmartModules](https://www.infinyon.com/blog/2022/06/smartmodule-json/)
  * Luis ([@morenol](https://github.com/morenol)) walks us through writing a SmartModule to transform JSON to into another form of JSON.
  * This post is Rust beginner friendly!

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
