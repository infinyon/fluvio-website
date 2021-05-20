---
title: Java
weight: 40
---

Similar to the Python client, this client also wraps the rust but does not yet
support any admin features. The calls to using fluvio objects across the
network are all blocking.

To see the full docs, visit [our javadoc page].

[our javadoc page]: https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/package-summary.html

## Connect
To get a connection to fluvio do:
```java
Fluvio fluvio = Fluvio.connect();
```

## Producer
To [create a producer](https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/Fluvio.html#topic_producer(java.lang.String)) do:
```java
TopicProducer producer = fluvio.topic_producer("hello-java");
```
### Send

To [send to the topic](https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/TopicProducer.html#send(byte%5B%5D,byte%5B%5D)) do:
```java
for (int i = 0; i < 10; i++) {
    producer.send(String.valueOf(i).getBytes(), ("Hello " + i).getBytes());
}
```
## Consumer

To [create a consumer](https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/Fluvio.html#partition_consumer(java.lang.String,int)) do:
```java
PartitionConsumer consumer = fluvio.partition_consumer("hello-java", 0);
```

### Stream
For now useg of the [consumer
stream](https://infinyon.github.io/fluvio-client-java/com/infinyon/fluvio/PartitionConsumer.html#stream(com.infinyon.fluvio.Offset))
requires calling `next` on the `stream` as seen here:

```java
PartitionConsumerStream stream = consumer.stream(Offset.beginning());
for (int i = 0; i < 10; i++) {
    Record record = stream.next();
    System.out.printf("Consumed record, key=%s, value=%s\n", record.key_string(), record.value_string());
}
```
