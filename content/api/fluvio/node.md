---
title: Node.js
weight: 30
---

This client uses [`node-bindgen`] to wrap the rust client. It supports most
administrator features. The blocking calls to fluvio return promises allowing
for async on blocking fluvio calls.

The [`PartitionConsumer.createStream`] call returns an [`asyncIterator`] to
allow iterating over the stream in a for-loop.

To see the full docs, visit [our typedoc page].

[`node-bindgen`]: https://github.com/infinyon/node-bindgen
[our typedoc page]: https://infinyon.github.io/fluvio-client-node/
[`PartitionConsumer.createStream`]: https://infinyon.github.io/fluvio-client-node/classes/partitionconsumer.html#createstream
[`asyncIterator`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for-await...of

## Connect

To [connect to fluvio] do:

```javascript
import Fluvio from '@fluvio/client';
const fluvio = await Fluvio.connect();
```

[connect to fluvio]: https://infinyon.github.io/fluvio-client-node/interfaces/fluvioclient.html#connect

## Produce

To [create a producer] do:
```javascript
const producer = await fluvio.topicProducer("my-topic");
```

[create a producer]: https://infinyon.github.io/fluvio-client-node/interfaces/fluvioclient.html#topicproducer

### Send

To [send a record] for a producer do:
```javascript
producer.send("my-key", "my-value");
```

[send a record]: https://infinyon.github.io/fluvio-client-node/classes/topicproducer.html#send

Note: The `send` call can take either an `ArrayBuffer` or a `string` for the
key/value fields in `send`.

## Consume

To [get a consumer] do:
```javascript
const partition = 0;
const consumer = await fluvio.partitionConsumer("my-topic", partition)
```

[get a consumer]: https://infinyon.github.io/fluvio-client-node/interfaces/fluvioclient.html#partitionconsumer

### Stream

To iterate over the items in a consumer do [create a stream]
and iterate over it like this:

[create a stream]: https://infinyon.github.io/fluvio-client-node/classes/partitionconsumer.html#createstream

```javascript
const stream = await consumer.createStream(Offset.FromBeginning());
for await (const record of stream) {
    let value = record.valueString();
    let key = record.keyString();
    console.log(`${key} - ${value});
}
```
