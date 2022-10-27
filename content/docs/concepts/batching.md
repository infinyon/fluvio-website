---
title: Batching
weight: 50
---
Fluvio producer tries to send records in batches to reduce the number of messages sent and improve throughput. A producer has some configurations that can be set to improve performance for a specific use case. For instance, they can be used to reduce disk usage, reduce latency, improve throughput, among other reasons.
As of today, batching behavior in Fluvio Producer can be modified with the following configurations:

- `batch_size`: Indicates the maximum amount of bytes that can be accumulated in a batch.
- `linger`: Time to wait before sending messages to the server. Defaults to 100 ms.
- `compression`: Compression algorithm used by the producer to compress each batch before sending it to the SPU. Supported compression algorithms are none, gzip, snappy and lz4.

In general, each one of these configurations has a benefit and a potential drawback. For instance, with the compression algorithm, it is a trade-off between disk usage in the server and CPU usage in the producer and the consumer for compression and decompression. Typically, the compression ratio is improved when the payload is large, therefore a larger `batch_size` could be used to improve the compression ratio. A `linger` equals `0` means that each record is sent as soon as possible. A `linger` time larger than zero introduces latency but improves throughput.

The ideal parameters for the `batch_size`, `linger` and `compression` depends on your application needs.

