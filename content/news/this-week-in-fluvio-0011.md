---
title: "This Week in Fluvio &#x23;11"
date: 2021-10-21
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

## New Release

### Fluvio v0.9.11

#### Producer Auto-reconnect
Prior to this release, if a client encountered any network hiccups that caused a disconnection during a Producer session, the Producer would experience an error.

Now the Producer will try to reconnect automatically.

#### SmartStreams w/ inputs
Using SmartStreams is a little more flexible now. Previously, a SmartStream filter only supported logic with inputs that were hardcoded and compiled before using.

However we've added the capability to pass in user inputs at the time of execution.

#### SmartStreams Array Map 
This is a new type of SmartStream API that will make it easier to chunk up large datasets into smaller pieces. For a more hands-on explanation with real data, please read our [blog post](https://infinyon.com/blog/2021/10/smartstream-array-map-reddit/) that demonstrates the capabilities of `#[smartstream(array_map)]`.


---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
[connectors]: /connectors