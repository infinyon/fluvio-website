---
title: "This Week in Fluvio #57"
date: 2023-08-08
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

---
Last week we released Fluvio 0.10.14 with exciting updates!

We had 4 conversations with lead data engineers and architects to discuss their data pipelines. We have talked to over 14 data engineers and architects in the last couple of weeks and these have been awesome. If you'd like to a have a conversation about your data pipelines and discuss problems, validate ideas - email me at drc@infinyon.com

## Latest release
* We released topic level deduplication which enables us to implement exactly once semantics on topics and deduplicate based on keys.
* We have also released timestamp injection in smartmodule context.
* Full [changelog is available here](https://github.com/infinyon/fluvio/blob/v0.10.14/CHANGELOG.md)

## Upcoming features
* We are produtizing a lean binary that runs on top of edge devices with minimum memory and storage with caching and mirroring to ensure data delivery from edge to cloud without losing data.
* We are also building out the foundations for multi region deployment to support users in the EU
* This is a way for us to get hybrid deployments and a single binary install of our stream processing engine.
* Upcoming blog on deduplication is in progress

## Open positions
We are hiring!
* [Sr. Rust Cloud Software Engineer](https://infinyon.com/careers/cloud-engineer-senior-level/)

## New blog post
* [Deduplicate data streaming events with SQL Upsert]('https://infinyon.com/blog/2023/07/sql-upsert/')

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