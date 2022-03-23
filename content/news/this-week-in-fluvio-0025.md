---
title: "This Week in Fluvio #25"
date: 2022-03-17
weight: 20
---
This Week in Fluvio is our weekly newsletter for development updates to [Fluvio open source].

Fluvio is a distributed, programmable streaming platform written in Rust.

{{< banner >}}

## New release
* [Fluvio v0.9.21](https://github.com/infinyon/fluvio/releases/tag/v0.9.21)

## New features
* Compression support ([#2082](https://github.com/infinyon/fluvio/issues/2082))
* Disk usage visibility in CLI via `Size` field added to the output of `fluvio partition list` ([#2148](https://github.com/infinyon/fluvio/issues/2148))
* Add support for partial CA Intermediate Trust Anchors ([#2232](https://github.com/infinyon/fluvio/pull/2232))

## Performance improvements
* Make store time out configurable for cluster startup ([#2212](https://github.com/infinyon/fluvio/issues/2212))
* Optimize partition size computation ([#2230](https://github.com/infinyon/fluvio/issues/2230))

## Bug fixes
* Fix Installer problem with self-signed certs ([#2216](https://github.com/infinyon/fluvio/issues/2216))
* Report SPU error codes to FutureRecordMetadata ([#2228](https://github.com/infinyon/fluvio/issues/2228))

## Miscellaneous
* Data generator support for `fluvio-test` ([#2237](https://github.com/infinyon/fluvio/pull/2237))

## Recent events
* Our CTO Sehyo Chang was on [Data Engineer's lunch episode #58](https://www.youtube.com/watch?v=H1tkiGsNz-Q) where he introduced Fluvio, gave a short demo of streaming data transformation using the CLI and SmartModules, and concluded with a Q&A.

## Upcoming events
* Register for Webinar on Apr 12, 2022: [Real-time Event Streaming and Data Transformation for Financial Services](https://register.gotowebinar.com/register/4870730280061351695)


---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
