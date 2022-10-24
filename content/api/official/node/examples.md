---
title: Node.js Examples
menu: Examples
weight: 30
---

* This client uses [`node-bindgen`] to wrap the Rust client.
* It supports most administrator features.
* The blocking calls to Fluvio return promises allowing for async on blocking Fluvio calls.
* The [`PartitionConsumer.createStream`] call returns an [`asyncIterator`] to allow iterating over the stream in a for-loop.

To see the full docs, visit [our typedoc page].

[`node-bindgen`]: https://github.com/infinyon/node-bindgen
[our typedoc page]: https://infinyon.github.io/fluvio-client-node/
[`PartitionConsumer.createStream`]: https://infinyon.github.io/fluvio-client-node/classes/PartitionConsumer.html#createStream
[`asyncIterator`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for-await...of


## Example Workflow

Follow the [installation instructions]({{< ref "installation.md" >}}) to run this example.

{{<code file="code-blocks/node/example.ts" lang="typescript" copy=true >}}

### Run

%copy first-line%
```shell
$ npx ts-node example.ts
```

## Links to Docs:
- [Connect to Fluvio](https://infinyon.github.io/fluvio-client-node/interfaces/FluvioClient.html#connect)
- [Create a Producer](https://infinyon.github.io/fluvio-client-node/interfaces/FluvioClient.html#topicProducer)
- [Send to Topic](https://infinyon.github.io/fluvio-client-node/classes/TopicProducer.html#send)
- [Get a Consumer](https://infinyon.github.io/fluvio-client-node/interfaces/FluvioClient.html#partitionConsumer)
- [Create a Stream](https://infinyon.github.io/fluvio-client-node/classes/PartitionConsumer.html#createStream)