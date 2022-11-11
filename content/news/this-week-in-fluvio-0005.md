---
title: "This Week in Fluvio #5"
date: 2021-09-02
weight: 20
---

Welcome to the fifth edition of This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## New Release - Fluvio v0.9.5

### Bugfix for SmartStream request handler

This week we discovered a bug in the SPU that was causing the request handler
for consumer streams to become blocked, preventing concurrent consumer stream
requests. This happened because upon receiving a stream request with a SmartStream
attached, the main SPU task would start compiling the SmartStream WASM code before
spawning a handler task for it. This meant that the SPU could not accept any
new stream requests until the WASM was finished compiling. In `0.9.5`, this has
been fixed by immediately spawning a new task before processing any of the work
for a stream request.

### Internal improvements: Refactoring around SmartStreams

When we originally implemented SmartStreams, we were new to using embedded WASM
runtimes, so we hadn't developed the cleanest patterns and had a fair amount of
code duplication. We've since gained a lot of experience in best practices for
how to structure common patterns for working with WebAssembly modules. This is
also largely thanks to the excellent improvements to the [wasmtime API since 0.28],
which has made it much easier to work with WASM in a multithreaded server
environment.

## Conclusion

This was a relatively light week for _releasing_ new features, which is partly
due to having started some brand-new features that are still in-flight, stay tuned
to hear more about those in upcoming weeks.

For the full list of changes this week, be sure to check out [our CHANGELOG]. If you have any
questions or are interested in contributing, be sure to [join our Discord channel] and
come say hello!

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[wasmtime API since 0.28]: https://github.com/alexcrichton/rfcs-2/blob/new-api/accepted/new-api.md
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[join our Discord channel]: https://discordapp.com/invite/bBG2dTz
