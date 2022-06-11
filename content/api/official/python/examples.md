---
title: Python Examples
menu: Examples 
weight: 20
---

{{< caution >}}

Only Producers and consumers support. No cluster administration support.

Also client does not support [async python].

{{</ caution >}}

The [`PartitionConsumer.stream`] returns an object which implements [python
iterator convention] to allow for iterating over the stream in a for-loop.

To see the full docs, visit [our pdoc page].

## Connect

To [connect to fluvio] do:

%copy%
```python
from fluvio import Fluvio
fluvio = Fluvio.connect()
```

[connect to fluvio]: https://infinyon.github.io/fluvio-client-python/fluvio.html#Fluvio.connect
## Produce

To [get a producer] do:

%copy%
```python
producer = fluvio.topic_producer("my-topic")
```

[get a producer]: https://infinyon.github.io/fluvio-client-python/fluvio.html#Fluvio.topic_producer

### Send
To [send a record] for the producer do:

%copy%
```python
producer.send_string("my-record")
```

[`send`] is also available. You will need to convert your key/value into a byte array
first to use it.

[send a record]: https://infinyon.github.io/fluvio-client-python/fluvio.html#TopicProducer.send
[`send`]: https://infinyon.github.io/fluvio-client-python/fluvio.html#TopicProducer.send

## Consumer

To [get a consumer], do:

%copy%
```python
partition = 0
consumer = fluvio.partition_consumer("my-topic", partition)
```

[get a consumer]: https://infinyon.github.io/fluvio-client-python/fluvio.html#Fluvio.partition_consumer

### Stream

To [get a stream] from the consumer do:

[get a stream]: https://infinyon.github.io/fluvio-client-python/fluvio.html#PartitionConsumer.stream

%copy%
```python
from fluvio import Offset
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
