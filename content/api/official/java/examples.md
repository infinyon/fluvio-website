---
title: Java Examples
menu: Examples
weight: 40
---

{{< caution >}}

Only Producers and consumers support. No cluster administration support.

{{</ caution >}}

* This client [wraps the Rust code](https://www.infinyon.com/blog/2021/05/java-client/).
* The calls to using Fluvio objects across the network are all blocking.

To see the full docs, visit [our javadoc page](https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/package-summary.html).

## Example Workflow

Follow the [installation instructions]({{< ref "installation.md" >}}) to run this example.

{{<code file="code-blocks/java/fluvio-java/app/build.gradle" lang="gradle" copy=true >}}

{{<code file="code-blocks/java/fluvio-java/app/src/main/java/fluvio/App.java" lang="java" copy=true >}}

### Run

%copy first-line%
```shell
$ gradle run
```

## Links to Docs:
- [Connect to Fluvio](https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/Fluvio.html#connect())
- [Create a Producer](https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/Fluvio.html#producer(java.lang.String))
- [Send to Topic](https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/TopicProducer.html#send(byte%5B%5D,byte%5B%5D))
- [Create a Consumer](https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/Fluvio.html#consumer(java.lang.String,int))
- [Create a Stream](https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/PartitionConsumerStream.html)