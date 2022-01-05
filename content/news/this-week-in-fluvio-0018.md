---
title: "This Week in Fluvio #18"
date: 2022-01-05
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

## New Release - Fluvio v0.9.16

### Time-based Data Retention

We have added time-based retention policy for data stored in topics. When records are created, we keep track of its age. When a record's age reaches the same duration as the retention policy, it is purged.

You can configure the retention duration time when you create a topic. 

%copy%
```bash
# Some example durations: '1h', '2d 10s', '7 days'
$ fluvio topic create <topic-name> --retention-time <time>
```

Along with the introduction of retention policy, new topics will be created with a default `7 day` retention.

Docs about retention policy coming soon.

### Auto-batching producer

For processing live data, using a batching workflow for sending records improves the efficiency of your data transfers by increasing throughput and reducing latency for each producer send. (As opposed to sending records individually)

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

### Support for 3rd-Party Connectors

We've improved the experience for running connectors. Previously, the only connectors to run were our officially built connectors, or you could [build and run your own local connectors]( {{<ref "/connectors/developer-guide/overview">}}).

In this new release, users who have built their own connectors and published the image(s) to Docker Hub can share those **pre-built** connector images with others.

We'll walk through an example scenario [from our tests](https://github.com/infinyon/fluvio/blob/master/tests/cli/smoke_tests/connector-3rd-party.bats).

First You need to start your cluster with the **HTTP URL prefix** where the YAML config for your the 3rd party image source is located.

We'll be using the URL "[https://raw.githubusercontent.com/infinyon/fluvio-connectors](https://raw.githubusercontent.com/infinyon/fluvio-connectors)" (Github's HTTP source to raw content of our repo, [fluvio-connectors](https://github.com/infinyon/fluvio-connectors)).

%copy%
```bash
$ fluvio cluster start --connector-prefix "https://raw.githubusercontent.com/infinyon/fluvio-connectors"
```

For our connectors to use an arbitrary image location, we specify a URL to a YAML config in the `type` field of in our connector config `3rd-party-test-connector-config.yaml`. Notice that the start of the URL matches the `--connector-prefix` we defined when we started the cluster.

%copy%
```yaml
# 3rd-party-test-connector-config.yaml
version: v1
name: my-third-party-connector
type: "https://raw.githubusercontent.com/infinyon/fluvio-connectors/main/rust-connectors/utils/test-connector/connector.yaml"
topic: my-test-connector-topic
create_topic: true
direction: source
```

The following is the contents of the `connector.yaml` config which we reference at the URL in `3rd-party-test-connector-config.yaml`. It only has one field `image`. The value is the name of the [Docker Hub image](https://hub.docker.com/r/infinyon/fluvio-connect-test-connector). 

```yaml
# connector.yaml
image: infinyon/fluvio-connect-test-connector
```

So when we create a connector with the `3rd-party-test-connector-config.yaml` config, it will use the image `infinyon/fluvio-connect-test-connector`

%copy%
```bash
$ fluvio connector create --config 3rd-party-test-connector-config.yaml
```

This feature experimental, but ready for feedback. More information about this feature will be documented soon! Until then, please reach out on our Discord for assistance getting started.

### CLI Release Channel

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

### Consume to end offset (CLI)

In the CLI, to start consuming records for a specific starting offset, you would use the `--offset` flag. Now you can also provide a final offset to close the Consumer stream when reached with the `--end-offset` flag.

Example 1:

* In Terminal 1, we open a consumer stream from the beginning of topic `twif` with an ending offset of `5`.
* In Terminal 2, we use `fluvio produce` to send over `10` records, which we will show first.

Terminal 1 - Producer:
```bash
$ fluvio produce twif
> 0   
Ok!
> 1
Ok!
> 2
Ok!
> 3
Ok!
> 4
Ok!
> 5
Ok!
> 6
Ok!
> 7
Ok!
> 8
Ok!
> 9
Ok!
> 10
Ok!
```

Terminal 2 - Record indexing is 0-based, so we expect the stream to close when we receive the 6th record.

```bash
$ fluvio consume -B --end-offset 5 twif
Consuming records from the beginning of topic 'twif'
0
1
2
3
4
5
⠋
Consumer stream has closed
```

Example 2:

We can also use a starting offset and ending offset together. As a result you can capture chunks of continuous blocks of records.

Here we use the existing `twif` topic, and consume a small subset of the records we produced earlier between offset 3-7 (inclusive).

```bash
$ fluvio consume --offset 3 --end-offset 7 twif
Consuming records from offset 3 in topic 'twif'
3
4
5
6
7
⠁
Consumer stream has closed
```

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions