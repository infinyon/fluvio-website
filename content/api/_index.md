---
title: API Overview
menu: Overview
section: APIs
---

This page describes how to generally use any of the Fluvio clients.  Each
client has some differences but these are the general rules about them.

For generated API Docs visit:

<div style="padding-left: 20px; display: inline">
{{< icon-rust link="https://docs.rs/fluvio/" external="true">}}
</div>

<div style="padding-left: 10px; display: inline">
{{< icon-python link="https://infinyon.github.io/fluvio-client-python/fluvio.html" external="true" >}}
</div>

<div style="padding-left: 10px; display: inline">
{{< icon-node  link="https://infinyon.github.io/fluvio-client-node/" external="true">}}
</div>

<div style="padding-left: 10px; display: inline">
{{< icon-java link="https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/package-summary.html" external="true">}}
</div>

<div style="padding-left: 12px; display: inline">
{{< icon-gopher link="https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/package-summary.html" external="true">}}
</div>

## Connect to Fluvio

The first thing you want to do to use a Fluvio client is connect to the Fluvio
cluster.

## Producer

Once you've got a connection handler, you will want to create a producer for a
given topic.

The producer could be created with the following configurations: `batch_size`, `compression`, `linger` and `partitioner`.

These configurations control the behavior of the producer in the following way:

* `batch_size`: Maximum amount of bytes accumulated by the records before sending the batch. Defaults to 16384 bytes.
* `compression`: Compression algorithm used by the producer to compress each batch before sending to the SPU. Supported compression algorithms are `none`, `gzip`, `snappy` and `lz4`.
* `linger`: Time to wait before sending messages to the server. Defaults to 100 ms.
* `partitioner`: custom class/struct that assigns the partition to each record that needs to be send. Defaults to Siphash Round Robin partitioner.

### Sending

When sending into a stream, the general `send` will take a `key` and a `value`.
The `key` is optional. For clients which don't have `Option` as a feature, this
is simply an empty array.

Depending on the client, these can be `string` or an array of `bytes`.

Depending on the producer configuration, a `send` call will not send immediately the record to the SPU. `flush` is used to immediately send all the queued records in the producer batches. Producers should `flush` before terminating to ensure that all records are sent properly.

## Consumer

Similar to a [producing]({{< ref "#producer" >}}), once you've got a connection, you'll need
to create a consumer for a given topic.

### Streams

Once you've got a consumer, you can create a stream given an [offset]({{< ref "#offsets" >}})
and listen for new items.

Most of our clients support idiomatic ways of iterating over the items in the stream:
* The rust client stream uses a [`Stream`]
* The node client stream implements the [`asyncIterator`]
* The python client stream implements `__next__` making it a [Python Iterator]

[`Stream`]: https://docs.rs/futures/0.3.15/futures/stream/trait.Stream.html
[`asyncIterator`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for-await...of
[Python Iterator]: https://wiki.python.org/moin/Iterator 

This functionality has not been implemented for the java client yet.

### Offsets

An offset is used when a stream is created to request the stream to start at
`N` items from the beginning/end of the stream.

### Records

Each item in a [stream]({{< ref "#streams" >}}) is a `Record`.

Our clients differ a little bit on this but a `Record` is a wrapper around
array of bytes with accessor methods of `key` or `value`.

In the python, node and java clients, we have to-string convenience methods.

### Timestamps

Fluvio `Records` contain timestamp information. As of Fluvio `0.9.25`, the timestamp of each record is set by Fluvio Producer on creation, previously the timestamp fields were uninitialized. This information is available to the consumer using the respective API call `timestamp()`.
