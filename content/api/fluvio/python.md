---
title: Python
weight: 20
---

The python client [wraps the rust client] and uses nearly the exact same
function definitions. It currently does not support the administrator features
that the rust client does. Handling async runners across the rust FFI is a bit
tricky so for now, this client does not support [async python].

The [`PartitionConsumer.stream`] returns an object which implements [python
iterator convention] to allow for iterating over the stream in a for-loop.

To see the full docs, visit [our pdoc page].

## Connect

To [connect to fluvio](https://infinyon.github.io/fluvio-client-python/fluvio.html#Fluvio.connect) do:

```python
from fluvio import Fluvio
fluvio = Fluvio.connect()
```

## Produce

To [get a producer](https://infinyon.github.io/fluvio-client-python/fluvio.html#Fluvio.topic_producer) do:
```python
producer = fluvio.topic_producer("my-topic")
```

### Send
To [send a record](https://infinyon.github.io/fluvio-client-python/fluvio.html#TopicProducer.send) for the producer do:
```python
partition = 0
producer.send_record_string("my-record", partition)
```

[`send`](https://infinyon.github.io/fluvio-client-python/fluvio.html#TopicProducer.send)
is also available. You will need to convert your key/value into a byte array
first to use it.

## Consumer

To [get a consumer](https://infinyon.github.io/fluvio-client-python/fluvio.html#Fluvio.partition_consumer), do:
```python
partition = 0
consumer = fluvio.partition_consumer("my-topic", partition)
```

### Stream

To [get a stream](https://infinyon.github.io/fluvio-client-python/fluvio.html#PartitionConsumer.stream) from the consumer do:

```python
stream = consumer.stream(Offset.beginning())
for i in stream:
    key = i.key_string()
    value = i.value_string()
    print("%s - %s" % (key, value))
```

[wraps the rust client]: https://www.infinyon.com/blog/2021/03/python-client/
[our pdoc page]: https://infinyon.github.io/fluvio-client-python/fluvio.html
[async python]: https://docs.python.org/3/library/asyncio.html
[`PartitionConsumer.stream`]: https://infinyon.github.io/fluvio-client-python/fluvio.html#PartitionConsumer.stream
[python iterator convention]: https://www.programiz.com/python-programming/iterator
