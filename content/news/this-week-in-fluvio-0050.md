---
title: "This Week in Fluvio #50"
date: 2023-03-31
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

This Week in Fluvio is our weekly newsletter for development updates to [Fluvio open source].

Please subscribe to This Week in Fluvio to receive new posts in your inbox
{{< subscribe-button >}}


{{< banner >}}

---
Welcome to the 50th edition of this week in Fluvio.

It's been 18 weeks since we last published our weekly newsletter. It was our first Fluvio Newsletter winter!

We are rebooting the newsletter and there is a lot more to come...

## New release
https://github.com/infinyon/fluvio/releases/tag/v0.10.6

## New features
*Connector Development Kit Launch:*
The goal of the Connector Development Kit (CDK) is to enable developers to write connectors to read data from data sources across different services using fluvio and Rust. We have been internally building our connectors using the CDK and are now ready to share it with the open source community.

Here are the docs to get started: https://www.fluvio.io/connectors/cdk/install/

We are excited to see what you are going to build with the CDK!

We are currently working on a way to certify connectors built by the community and would love your inputs on [our Discord channel].

Besides the Connector Development Kit, we have made other improvements to the packaging of Fluvio components.

Full changelog available here: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md

## Upcoming features
We are aware of the frustrations of getting started with fluvio due to the clunky installation process. We are considering a clean binary install, as well as installing using a package manager. However, we have been busy with the CDK, SMDK and other Cloud Features.

We would love to know what do you recommend we build next to make your development flow easier. Please comment in [our Discord channel] and let us know. Tag Deb (DRC) #7918 on your feedback and feature requests.

## Bug fixes
Enabled --file and --key-separator to be used together, fix --key handling when producing lines (#3092)

## Community update
[Carson Rajcan](https://github.com/crajcan) presented to our team a really cool contribution to the Fluvio OSS. In the project Carson developed the functionality to apply Smart Module transformation for Producer. The team is evaluating the PR - https://github.com/infinyon/fluvio/pull/3014

As Winter is now over... We are planning community events to champion community contributors, as well as planning hackathons to build som cool stuff with the community using CDK, SMDK etc.

Let us know what events you are interested in [our Discord channel].

## InfinyOn Cloud updates
We are putting the finishing touches on connector secrets for you to use the InfinyOn certified connectors built using CDK to interact with InfinyOn Cloud while useing passwords, API keys.

We are also shaping work for materialized views of time based aggregates from topics.

We are also exploring a developer user experience for our cloud platform. More updates on this in a couple of months.

## New blog post
[Using DuckDB with Fluvio](https://infinyon.com/blog/2023/02/duckdb-fluvio/)

## New video
<iframe width="560" height="315" src="https://www.youtube.com/embed/r6xiXti78Ek" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

## Open positions
[Sr. Rust Cloud Software Engineer](https://infinyon.com/careers/cloud-engineer-senior-level/)
[Developer Advocate](https://infinyon.com/careers/developer-advocate-mid-senior-level/)

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
