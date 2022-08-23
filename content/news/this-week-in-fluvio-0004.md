---
title: "This Week in Fluvio #4"
date: 2021-08-26
weight: 20
---

Welcome to the fourth edition of This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

[Fluvio open source]: https://github.com/infinyon/fluvio

{{< banner >}}

## New Release - Fluvio v0.9.4

### Compression for WASM binaries

This week we received an excellent contribution from [@tomindisguise] for compressing
WebAssembly binaries before uploading them to Fluvio's Streaming Processing Units (SPUs).
As some background, one of Fluvio's premiere features is the ability to upload
user-defined code to perform inline processing on streaming data,
[a feature we call SmartStreams]. User code must be compiled to WebAssembly and
uploaded to SPUs upon opening a stream, and this change helps by significantly reducing
the size of the upload request. Thank you to [@tomindisguise] for the contribution!

[@tomindisguise]: https://github.com/tomindisguise
[a feature we call SmartStreams]: {{<ref "/smartmodules" >}}

### Bugfix for applying SmartStreams while using `--disable-continuous`

This is a fix to a CLI bug that cropped up when we introduced SmartStreams. For some
context, Fluvio used to have two types of requests for consuming data: a "fetch" request
and a "stream" request. The fetch request would retrieve only data that was known in
advance to be available, whereas a stream request keeps the connection open and continues
to deliver new data to the consumer as it arrives to the topic.

When using the Fluvio CLI Consumer, the default behavior is to open a stream and continue
listening for data, but one may optionally use the `-d`/`--disable-continuous` flag in order
to make a one-time request and close the connection. These two behaviors internally mapped
to the "stream request" and the "fetch request", respectively. The bug that cropped up is
that we realized that SmartStream logic was only being applied to stream requests, not to
fetch requests.

This led us to simply hurry up on deprecating the fetch request, which is something that
we have been meaning to do for a while now. As of this fix, `--disable-continuous` is now
implemented by using a stream request that closes upon reaching the known end of the stream.
This should not cause any breakages to CLI users, but now using `-d`/`--disable-continuous`
with a SmartStream will properly apply the logic to the stream as expected.

### Now publishing `aarch64` docker images

That's right, not only do we now offer support for `aarch64` binaries themselves, we are
also publishing Fluvio docker images for `aarch64` targets. This means that Fluvio may
now be deployed on AWS Graviton! We're very excited about this advancement, and we're
looking forward to seeing it deployed on ARM in the near future!

## Conclusion

For the full list of changes, be sure to check out [our CHANGELOG]. If you have any
questions or are interested in contributing, be sure to [join our Discord channel] and
come say hello!

Until next week!

[SmartStreams Aggregate]: /docs/smartstreams/aggregate
[sums]: https://github.com/infinyon/fluvio/blob/master/src/smartstream/examples/aggregate-sum/src/lib.rs
[averages]: https://github.com/infinyon/fluvio/blob/master/src/smartstream/examples/aggregate-average/src/lib.rs
[combining structural key-value data]: https://github.com/infinyon/fluvio/blob/master/src/smartstream/examples/aggregate-json/src/lib.rs
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[join our Discord channel]: https://discordapp.com/invite/bBG2dTz
