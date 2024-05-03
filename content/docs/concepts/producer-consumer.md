---
title: Producers and Consumers
weight: 10
---

## Producer

Producers are applications that "produce" streaming data.
These applications may be monitoring the status of a system, collecting sensor
data, watching traffic to a website, or otherwise observing events in some way.
Producers may be general-purpose tools such as [the Fluvio CLI], or they may be
special-purpose programs built to meet a specific need, in which case the
producer program would leverage [one of the programmatic APIs].

[the Fluvio CLI]: {{< ref "/cli" >}}
[one of the programmatic APIs]: {{< ref "/api" >}}


## Consumer 
Consumers are applications that "consume" records from a particular
topic and partition<sup>[1]</sup>. Typically, a consumer will perform actions based
on the events it receives, such as sending a notification or updating
a database. There may be many consumers reading data from a particular
partition at any given time: since the records are persisted, they do
not expire after being consumed.

-> [1] For an overview of Topics and Partitions, see the [Topic documentation]

[Topic documentation]: {{< ref "/docs/concepts/topics" >}}

### Consumer Offsets
**Consumer Offset** is the [offset] value assigned to the specified consumer name. Upon consuming more records from the topic a consumer updates the offset by performing periodic commit/flush operations. Later, this offset can be retrieved from the **Fluvio** cluster to continue reading from the last record.  

**Consumer Offsets** are persistent and durable, surviving Fluvio cluster restarts and upgrades. Essentially, Fluvio provides the same storage guarantees for Consumer Offsets as it does for the data stored in topics.  

The offset value is maintained separately for each topic partition. Once created, it remains stored until explicitly deleted.  
Users can delete the offset either programmatically through their application code or via the Fluvio CLI.

#### Commit Strategy
Fluvio offers different approaches to offset management, allowing users to choose the one that best fits their use case and requirements. There are **manual** and **auto** offset management strategies:  
1. **Manual Strategy**  
In this strategy, offsets are managed manually by the user. This means that the user is responsible for committing offsets explicitly when needed. No automatic commits or flushes occur, and all offset management operations must be initiated by the user.
2. **Auto Strategy**  
In contrast to the manual strategy, the auto strategy involves automatic management of offsets by the system. When using this strategy, committing offsets is triggered implicitly when reading the next record, effectively committing the offset for the previous record. Additionally, periodic flushes occur at intervals defined in the configuration.

#### Configuration
* `offset_consumer: String` - the consumer name to uniquely identify the consumer offset.  
* `offset_start: Offset` - the default offset value will be used if the consumer offset does not exist.
* `offset_strategy: OffsetManagementStrategy` - defines whether offsets are committed/flushed automatically, manually, or not. 
* `offset_flush: Duration` - the period for auto flushes(only relevant for auto offset strategy).

### Examples
#### Fluvio CLI
Imagine we have an empty topic `hello-topic`. Let's produce some records:

%copy%
```bash
$ echo "One" | fluvio produce hello-topic
$ echo "Two" | fluvio produce hello-topic
```
Ok, now let's read the topic from the beginning on behalf of consumer `c1`:

%copy first-line%
```bash
$ fluvio consume hello-topic -c c1 -Bd
Consuming records from 'hello-topic' starting from the beginning of log
One
Two
```
From now we can see our `c1` consumer in the consumers list:

%copy first-line%
```bash
$ fluvio consumer list
  CONSUMER  TOPIC        PARTITION  OFFSET  LAST SEEN
  c1        hello-topic  0          1       4m 14s
```
The offset here denotes the last seen offset for the given consumer in the given partition.

Let's add another record to the topic:

%copy first-line%
```bash
$ echo "Three" | fluvio produce hello-topic
```
and read the topic again from the beginning for the same consumer:

%copy first-line%
```bash
$ fluvio consume hello-topic -c c1 -Bd
Consuming records from 'hello-topic' starting from the beginning of log
Three
```
we get only one "unseen" record which is correct.
Now if you try another consumer `c2`, you get all the records:

%copy first-line%
```bash
$ fluvio consume hello-topic -c c2 -Bd
Consuming records from 'hello-topic' starting from the beginning of log
One
Two
Three
```

The consumer list now shows us two consumers:

%copy first-line%
```bash
$ fluvio consumer list
  CONSUMER  TOPIC        PARTITION  OFFSET  LAST SEEN
  c1        hello-topic  0          2       3m 51s
  c2        hello-topic  0          2       2m 21s
```

We can delete them now:

%copy first-line%
```bash
$ fluvio consumer delete c1
consumer "c1" on topic "hello-topic" and partition "0" deleted
$ fluvio consumer delete c2
consumer "c2" on topic "hello-topic" and partition "0" deleted
```

#### Manual offset management
This is an example of programmatic consumers with the manual offset management strategy:
```rust
use fluvio::{
   consumer::{ConsumerConfigExtBuilder, ConsumerStream, OffsetManagementStrategy},
   Fluvio, Offset,
};
use futures_util::StreamExt;
async fn do_consume_with_manual_commits(fluvio: &Fluvio) -> anyhow::Result<()> {
   let mut stream = fluvio
       .consumer_with_config(
           ConsumerConfigExtBuilder::default()
               .topic("my-topic".to_string())
               .offset_consumer("my-consumer".to_string())
               .offset_start(Offset::beginning())
               .offset_strategy(OffsetManagementStrategy::Manual)
               .build()?,
       )
       .await?;
   while let Some(Ok(record)) = stream.next().await {
       println!("{}", String::from_utf8_lossy(record.as_ref()));
       stream.offset_commit()?;
       stream.offset_flush().await?;
   }
   Ok(())
}
```
#### Auto offset management
This is an example of programmatic consumers with the auto offset management strategy. As you can see there are no commits and flushes in the code, everything happens under the hood automatically.
```rust
use fluvio::{
   consumer::{ConsumerConfigExtBuilder, ConsumerStream, OffsetManagementStrategy},
   Fluvio, Offset,
};
use futures_util::StreamExt;
async fn do_consume_with_auto_commits(fluvio: &Fluvio) -> anyhow::Result<()> {
   let mut stream = fluvio
       .consumer_with_config(
           ConsumerConfigExtBuilder::default()
               .topic("my-topic".to_string())
               .offset_consumer("my-consumer".to_string())
               .offset_start(Offset::beginning())
               .offset_strategy(OffsetManagementStrategy::Auto)
               .build()?,
       )
       .await?;
   while let Some(Ok(record)) = stream.next().await {
       println!("{}", String::from_utf8_lossy(record.as_ref()));
   }
   Ok(())
}
```
#### Using with connectors
The following example demonstrates how to enable **Consumer Offsets** on [Fluvio Http Sink connector]. The same configuration applies to all official Fluvio `sink` connectors.
```yaml
apiVersion: 0.2.0
meta:
  version: 0.2.7
  name: my-http-sink
  type: http-sink 
  topic:
    meta:
      name: http-sink-topic
  consumer:
    id: my-http-sink
    offset:
      strategy: auto
      start: beginning
      flush-period: 
        secs: 2
        nanos: 0
http:
  endpoint: "http://127.0.0.1/post"
```

In this setup, the Consumer initially attempts to retrieve the offset value for the identifier `my-http-sink` upon starting. If this value not found, it will commence from the beginning. Offset flushes occur automatically every 2 seconds.  
More information about [connectors configuration].

[offset]: {{< ref "/docs/concepts/offsets" >}}
[Fluvio Http Sink connector]: {{< ref "/connectors/outbound/http" >}}
[connectors configuration]: {{< ref "/connectors/connector-config" >}}

