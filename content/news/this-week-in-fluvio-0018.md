---
title: "This Week in Fluvio #18"
date: 2022-01-05
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

## New Release

### Fluvio v0.9.16

#### Time-based Data Retention

We have added time-based retention policy for data stored in topics. When records are created, we keep track of its age. When a record's age reaches the same duration as the retention policy, it is purged.

You can configure the retention duration time when you create a topic. 

%copy%
```bash
# Some example durations: '1h', '2d 10s', '7 days'
$ fluvio topic create <topic-name> --retention-time <time>
```

Along with the introduction of retention policy, new topics will be created with a default `7 day` retention.

Docs about retention policy coming soon.

#### Auto-batching producer

For processing live data, using a batching workflow for sending records improves the efficiency of your data transfers by increasing throughput and reducing latency for each producer send.

Producer batch support already exists in the CLI using `fluvio produce`, but you can realistically only use this CLI feature if you produce with the `--file` option. 

Using the Rust API, you could have used [`send_all`], but this primarily enables sending multiple records whenever called. Using [`send_all`] by itself didn't ensure a consistent behavior.

[`send_all`]: https://docs.rs/fluvio/0.12.0/fluvio/struct.TopicProducer.html#method.send_all 

At the end of the day, it meant that if you want time-based or size-based batching, this was extra effort for the developer to implement themselves.

In this release, we make it easier to use batching in the Rust API. To use create an auto-batching Producer, you need to create your `TopicProducer` configured with batch and/or linger.

Example:

%copy%
```rust
let fluvio_client = Fluvio::connect().await?;
let config = TopicProducerConfigBuilder::default()
    .linger(Duration::from_millis(600000))
    .batch_size(17)
    .build()
    .expect("failed to build config");

let producer: TopicProducer = fluvio_client 
    .topic_producer_with_config(topic, config)
    .await;
```

For more detail on the available config options, see the [Rust docs](https://docs.rs/fluvio/0.12.0/fluvio/struct.TopicProducerConfigBuilder.html)

#### Support for 3rd-Party Connectors

We've improved the experience for running connectors. Previously, the only options for connectors were to run our officially built connectors, or to locally build your own. In this new release, users can set up their connector pipelines and allow for others to use unofficial Fluvio connectors in their own Fluvio clusters.

More information about this feature will be documented soon! Until then, please reach out on our Discord for assistance getting started.

#### CLI Release Channel

The ability to test pre-release changes in CLI is now easier to do with CLI channels.

More documentation is coming soon, but if you're familiar with [Rust's release channels](https://rust-lang.github.io/rustup/concepts/channels.html), you'll be familiar with Fluvio's CLI channels.

New Fluvio installations support the ability to switch back and forth between the most recent `stable` release or the `latest` development builds of the Fluvio CLI.

CLI channels will be especially useful for the current users who have reached out to us on Discord. Now we can more easily work together to quickly validate fixes to issues without the need to build the Fluvio code locally.

To try out channels now, you will need to re-install Fluvio with the [instructions on the download page]({{<ref "/download">}}). This will download the channel-enabled frontend and the most recent `stable` release.

%copy%
```bash
# Switch to the `latest` channel
$ fluvio version switch latest
# Switch to the `stable` channel
$ fluvio version switch stable 
```

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions