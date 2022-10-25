---
title: "This Week in Fluvio #30"
date: 2022-04-27
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

This Week in Fluvio is our weekly newsletter for development updates to [Fluvio open source].

Please subscribe to This Week in Fluvio to receive new posts in your inbox
{{< subscribe-button >}}

{{< banner >}}


## New release
* [Fluvio v0.9.24](https://github.com/infinyon/fluvio/releases/tag/v0.9.24)
* [DynamoDB Sink connector]({{<ref "/connectors/outbound/dynamodb.md">}})
  * The Dynamodb Sink Connector is a sink connector which reads events from a Fluvio topic, deserializes them as json and inserts those key value pairs based on the columns in the config.
* [Slack Sink connector]({{<ref "/connectors/outbound/slack.md">}})
  * The Slack Connector is quite simple. It will stringify any record coming from a Fluvio stream and POST it to the slack via a slack webhook url

## New features
* Storage: Enforce size based retention for topic ([#2179](https://github.com/infinyon/fluvio/issues/2179))
  * Previously, Fluvio supported only a time-based retention policy for data in a topic. For some workloads, it was inconvenient as it was needed to consider the incoming data pace to properly calculate retention time to fit the
  data into the available storage size. With this new feature, you can tell Fluvio what is the maximum size of the
  partition you want, and it will control it for you. Check out the details in
  [Data Retention]({{< ref "/docs/operations/retention#max-partition-size" >}}).
* Export cluster profile to a file ([#2327](https://github.com/infinyon/fluvio/issues/2327))
  * Can be used to initialize the connection to a Fluvio cluster via client APIs.

## Bug fixes
* Don't try to use directories as smartmodule if passed as argument ([#2292](https://github.com/infinyon/fluvio/issues/2292))
* CLI: Migrate all fluvio crates to `comfy-table` from `prettytable-rs` ([#2285](https://github.com/infinyon/fluvio/issues/2263))

## New blog post
* [Real-time Gaining Momentum in the Enterprise](https://www.infinyon.com/blog/2022/02/real-time-gaining-momentum/)
  * Grant shares how legacy data infrastructures operate and how Infinyon with Fluvio's community are positioned to leverage modern technology for faster and higher quality data infrastructures. 

## Recent events
* Infinyon was featured as a one of <a href="https://reneeshah.medium.com/how-webassembly-gets-used-the-18-most-exciting-startups-building-with-wasm-939474e951db" rel="nofollow">The 18 Most Exciting Startups Building with Wasm</a>
  * Our CEO, A.J. Hunyady, answered questions about our decision to use WebAssembly, and how we use it in Fluvio and Infinyon Cloud.

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
