---
title: "This Week in Fluvio #56"
date: 2023-07-25
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

---
Big updates are on the verge of being released this week. Fluvio 0.10.14 is going to have some awesome updates!

This week hase been great with our efforts aligning towards our next release.

BTW, we have been offering free data architecture reviews to data practitioners to help with planning data pipelines. We have talked to over 10 data engineers and architects in the last couple of weeks and these have been awesome. If you'd like to a have a conversation about your data pipelines and discuss problems, validate ideas - email me at drc@infinyon.com

## Upcoming features
We are testing timestamp access and manipulation in the transformation smart modules. This would complement our lookback functionality to look at the past data, and deduplication on read. We are so close to exactly once delivery guarantees with on stream deduplication based on user defined keys.

Our biggest update that is in the works is an engine that powers unbounded stateful processing in our platform.

This is a big step towards our vision for a composable unified stateful stream processing platform. 

## Developer experience improvements
We have been working on running our coure stream processor at the edge to support ARM based sensors with limited memory and we brought back the raspberry pi out of our shelves and working on a demo.

We are also working on lean implementations of our runtime and control plane to build the foundations to support issues of data privacy and data sovereignty with hybrid deployment patterns.

## Open positions
We are hiring!
* [Sr. Rust Cloud Software Engineer](https://infinyon.com/careers/cloud-engineer-senior-level/)

## New blog post
* [OLAP for Event Streaming with MotherDuck Connector]('https://infinyon.com/blog/2023/07/infinyon-motherduck/')

## New video
* [Collecting Hackernews RSS Feeds using InfinyOn Cloud]('https://youtu.be/fVzLoaIHQfM')

That's all folks. until next week.

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions