---
title: "This Week in Fluvio #17"
date: 2021-12-08
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## New Release - Fluvio v0.9.15

### TableFormat improvements

Prior to this release, rendering events with `fluvio consume --output=full-table` required events to arrive as individual JSON objects. But now we can additionally accept a list of events as an array of JSON objects.

Example topic data. Mix of JSON objects, and an array of objects.

%copy%
```json
{"key1":"a","key2":"1","key3":"Alice","id":123}
{"key1":"b","key2":"2","key3":"Bob","id":456}
{"key1":"c","key2":"3","key3":"Carol","id":789}
[{"key1":"x","key2":"10","key3":"Alice","id":123},{"key1":"y","key2":"20","key3":"Bob","id":456},{"key1":"c","key2":"30","key3":"Carol","id":789}]
```

Check out the [TableFormat docs]({{<ref "/cli/commands/table-format#examples">}}) for more information about using `TableFormat`.

### Migration to Rust edition 2021

This update affects those using our [Rust API]({{<ref "/api/official/rust/installation">}}). Our crates have transitioned to the new [Rust 2021 edition](https://doc.rust-lang.org/edition-guide/rust-2021/index.html).

If you want to migrate your existing projects with our crates, you can follow the official [Rust edition guide](https://doc.rust-lang.org/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html)

And finally edit your `Cargo.toml` to use the new edition.

```toml
edition = "2021"
```

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions