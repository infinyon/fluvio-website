---
title: "This Week in Fluvio #53"
date: 2023-07-05
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

---
After yet another hiatus this week in Fluvio is back! We have released a number of improvements in the last little while.

Our [public roadmap](https://github.com/orgs/infinyon/projects/12/views/1) is live!
The high level functionality that we are building on Fluvio and InfinyOn Cloud is availble on the roadmap.

We want to hear from you about the features and functionality you are looking for and encourage you to submit issues to the [Fluvio Repo] (https://github.com/infinyon/fluvio).

## New release
[Fluvio 0.10.12](https://github.com/infinyon/fluvio/releases/tag/v0.10.12):
Our latest release inludes:
* Updates to the CLI to pass topic-configuration to create topics.
* Updated SQL Sink Connectore to support upsert.
* We launched a certified DuckDB connector based on [Fluvio DUck](https://github.com/infinyon/fluvio-duck)!


## New features
Fluvio stream processing core now enables the functionality to look-back which is exactly what you think! It's a way to time travel into the past retained records. We need this feature to dedupe records on read.

## Upcoming features
If you did not guess it from the last section, we are building deduplication on read to deliver records exactly once.

We have also started working on stateful stream processing, specifically grouping and time window based aggregation.

## Developer experience improvements
There are updates to logging of connectors to improve the traceability of when the legacy systems struggle to keep up with the streams and the connectors require a refresh. [Fluvio Docs](https://fluvio.io/connectors/cdk/list-log/) are updated with the log levels.

## Community Engagement

We are offering free architecture reviews for data practitioners who are looking to improve their data flows and optimize for:

- Simplicity
- Efficiency
- Cost

Send a note to our head of product and part time community advocate if you want a free architecture review session.

Email - drc@infinyon.com

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
