---
title: "This Week in Fluvio #12"
date: 2021-10-27
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## New Release - Fluvio v0.9.12

### ArrayMap examples
This is an addition to [last week]( {{< ref "/news/this-week-in-fluvio-0011.md" >}})'s release of ArrayMap. We have some new examples.

- [ArrayMap w/ JSON array inputs](https://github.com/infinyon/fluvio/blob/d63e3e2569e4d64a098e5c2189ac68e6e9cd2670/crates/fluvio-smartmodule/examples/array_map_json_array/src/lib.rs)
- [ArrayMap w/ JSON object inputs](https://github.com/infinyon/fluvio/blob/d63e3e2569e4d64a098e5c2189ac68e6e9cd2670/crates/fluvio-smartmodule/examples/array_map_json_object/src/lib.rs)
- [ArrayMap example code](https://github.com/infinyon/fluvio/blob/d63e3e2569e4d64a098e5c2189ac68e6e9cd2670/crates/fluvio-smartmodule/examples/array_map_json_reddit/src/lib.rs) from [our previous blog post](https://www.infinyon.com/blog/2021/10/smartstream-array-map-reddit/)

### Improved user experience when loading invalid SmartStream code
Prior to this release, trying to use invalid SmartStream code (but valid Rust code) with a consumer would result in the stream closing without a specific reason being reported to the user.

A user can compile their own SmartStream code, but if they forget to decorate their function with the `#[smartstream]` attribute, it will still compile because it is valid Rust. However, not all valid Rust code is valid SmartStream code.

Example invalid SmartStream code:

%copy%
```rust
use fluvio_smartstream::{Record, Result};

// Note the missing #[smartstream] attribute that should be here!
pub fn filter(record: &Record) -> Result<bool> {
    let string = std::str::from_utf8(record.value.as_ref())?;
    Ok(string.contains('a'))
}
```

Previous experience using the invalid SmartStream filter:

%copy first-line%
```bash
$ fluvio consume fruits --tail --filter=crates/fluvio-smartstream/examples/target/wasm32-unknown-unknown/debug/fluvio_wasm_filter.wasm
Consuming records starting 10 from the end of topic 'fruits'
Consumer stream has closed
```

Now, if a user tries to use incorrectly compiled SmartStream code they will receive an error message.

%copy first-line%
```bash
$ fluvio consume fruits --tail --filter=crates/fluvio-smartstream/examples/target/wasm32-unknown-unknown/debug/fluvio_wasm_filter.wasm
Consuming records starting 10 from the end of topic 'fruits'
Error:
   0: Fluvio client error
   1: SmartStream error
   2: WASM module is not a valid 'filter' SmartStream. Are you missing a #[smartstream(filter)] attribute?
```

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
[connectors]: /connectors