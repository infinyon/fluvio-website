---
title: Python Examples
menu: Examples
weight: 20
---

* The Python client [wraps the rust client](https://www.infinyon.com/blog/2021/03/python-client/).
* It currently does not support the administrator features that the rust client does.
* The [PartitionConsumer.stream](https://infinyon.github.io/fluvio-client-python/fluvio.html#PartitionConsumer.stream) returns an object which implements the [python iterator convention](https://wiki.python.org/moin/Iterator) to allow for iterating over the stream in a for-loop.

To see the full docs, visit our [pdoc page](https://infinyon.github.io/fluvio-client-python/fluvio.html).
## Example Workflow

Follow the [installation instructions]({{< ref "installation.md" >}}) to run this example.

### Prerequisites

Create the topic used to produce and consume records:

%copy%
```bash
fluvio topic create python-data
```

### Login

Login to Infinyon Cloud using username/password

%copy%
```shell
from fluvio import cloud
cloud.login(email="my@email.com", password="mypassword")
```

Login to Infinyon Cloud using `--ouath2`

%copy%
```shell
from fluvio import cloud
cloud.login()
```

### Producer

Create a file called `python-produce.py`:

{{<code file="embeds/client-examples/python/python-produce.py" lang="python" copy=true >}}

Let's run the file: 

%copy first-line%
```shell
$ python python-produce.py
```

### Consumer

Create a file called `python-consume.py`:

{{<code file="embeds/client-examples/python/python-consume.py" lang="python" copy=true >}}

Let's run the file: 

%copy first-line%
```shell
$ python python-consume.py
```

## Limitations
* Fluvio cluster administration is not supported.
* Python [async](https://docs.python.org/3/library/asyncio.html) is not supported.

## Example with a SmartModule

{{<code file="embeds/client-examples/python/hello-python-smartmodule.py" lang="python" copy=true >}}


## Links to Docs:
* [Connect to Fluvio](https://infinyon.github.io/fluvio-client-python/fluvio.html#Fluvio.connect)
* [Get a Producer](https://infinyon.github.io/fluvio-client-python/fluvio.html#Fluvio.topic_producer)
* [Send to Topic](https://infinyon.github.io/fluvio-client-python/fluvio.html#TopicProducer.send)
* [Get a Consumer](https://infinyon.github.io/fluvio-client-python/fluvio.html#Fluvio.partition_consumer)
* [Get a Stream](https://infinyon.github.io/fluvio-client-python/fluvio.html#PartitionConsumer.stream)
