---
title: "This Week in Fluvio #55"
date: 2023-07-18
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

---
Fluvio 0.10.13 was released on 14th July 2023! we are back on time for this week in Fluvio - 55.

## Latest release
* We are almost there with the deduplication functionality on stream. We just completed implementing time bound look back and topic based deduplication interface.
* We have fixed the smart module development kit and the connector development kit publish workflows, and chrono dependency.
* Full [changelog is available here](https://github.com/infinyon/fluvio/blob/v0.10.13/CHANGELOG.md)

## Upcoming features
We are wrapping up the docs to publish the graphite connector in the [InfinyOn Labs repo](https://github.com/infinyon/labs-projects).

## Developer experience improvements
* We are experimenting with running the fluvio binary on the edge devices and pushing it to the limits to enable more efficient data capture in memory and internet constraints without data loss as opposed to the existing message brokers.
* We are also testing out inbound webhook implementation to capture data into topics and conceptualizing what an oputbound webhook gateway might look liketo read from streaming topics.
* Finally, we have made progress on shaping and prototyping stateful materialized views on stream! This is one of the most asked functionality and we are working with select partners to test out the functionality in their context.


## Open positions
We are hiring!
* [Sr. Rust Cloud Software Engineer](https://infinyon.com/careers/cloud-engineer-senior-level/)

## New blog post
* [Upsert on SQL Sink Connector]('https://infinyon.com/blog/2023/07/sql-upsert/')

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
