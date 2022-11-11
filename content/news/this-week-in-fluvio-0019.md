---
title: "This Week in Fluvio #19"
date: 2022-01-12
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## New Release - Fluvio v0.9.17

### Tune configuration defaults to Producer auto-batching

This is a small change to the default auto-batching producer configuration linger and batch size.

* Linger time was set to `250ms` but now it is `100ms`.
* Batch size was `16000` but now is `16384` (That's 2<sup>14</sup>)

If you want to configure your producer to something other than the defaults, you can set a different value for your producer when you create the config.

Example of auto-batching producer w/ linger of `1 second` and a batch size of `100` bytes.

%copy%
```rust
let fluvio_client = Fluvio::connect().await?;
let config = TopicProducerConfigBuilder::default()
    .linger(Duration::from_secs(1))
    .batch_size(100)
    .build()
    .expect("failed to build config");

let producer: TopicProducer = fluvio_client 
    .topic_producer_with_config(topic, config)
    .await;
```
### CLI consumer output change

This is a fix to the CLI consumer using the `--format` output feature. Due to the default behavior of [handlebars](https://crates.io/crates/handlebars), the templating engine we use for [custom formatting on the CLI]({{<ref "/cli/commands/consume#example-6-print-consumed-records-with-custom-formatting">}}), we were unintentionally HTML escaping record data. But that is not longer the behavior.

Before:
```bash
# Fluvio v0.9.16 and older
$ fluvio consume twif19 --format "{{offset}} {{value}}" -B -d
Consuming records from the beginning of topic 'twif19'
0 {&quot;examplekey&quot;:&quot;examplevalue&quot;}
```

After:
```bash
# Fluvio v0.9.17+
$ fluvio consume twif19 --format "{{offset}} {{value}}" -B -d
Consuming records from the beginning of topic 'twif19'
0 {"examplekey":"examplevalue"}
```

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions