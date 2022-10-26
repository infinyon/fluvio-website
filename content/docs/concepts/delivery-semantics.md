---
title: Delivery Semantics
---
The Internet, as well as other networks, is considered an unreliable communication channel. There can be delays or lost messages.
The connection can be gone for some period of time. This aspect affects records delivery reliability between Fluvio Producer and the SPU.
To control that, Fluvio Producer has a `delivery_semantic` configuration option, which allows choosing a delivery mechanism. Each mechanism has
a different trade-off between reliability and performance. There are two delivery semantics currently supported by Fluvio Producer:
`at-most-once` and `at-least-once`.

### At Most Once
`at-most-once` delivery means that for each record handed to Fluvio Producer, that record is delivered zero or one times;
in more casual terms it means that messages may be lost. Fluvio Producer sends the message with records to the SPU and **does not
wait** for the response. Consider it as **fire and forget** approach. This delivery method has higher throughput but no
any guarantees if the message is delivered.


[Producer Isolation]({{< ref "/docs/concepts/data-consistency#producer-isolation" >}}) has no effect if this delivery
semantic is used unless the user explicitly waits for the response, as shown in the following snippet:

%copy%
```rust
let fluvio = Fluvio::connect().await?;
let config = TopicProducerConfigBuilder::default()
    .delivery_semantic(DeliverySemantic::AtMostOnce)
    .build()?;
let producer = fluvio.topic_producer_with_config("greetings", config).await?;
let output = producer.send("Hello", "Fluvio!").await?;
output.wait().await?; // wait for the response, considering `Isolation` as well
```


### At Least Once
`at-least-once` delivery means that for each record handed to the Fluvio Producer potentially **multiple attempts** are made
at delivering it, such that at least one succeeds; again, in more casual terms this means that messages may be duplicated
but not lost. Fluvio Producer sends the message with records to the SPU, **waits** for the response and **re-send** in case of
transport errors occur. This delivery method has lower throughput comparing to `at-most-once` but better total reliability.


There are three main parameters that one should consider using `at-least-one` semantic: maximum amount of retries, the time
distribution (fixed, Fibonacci or exponential) of delays between them, and maximum timeout for all attempts.

Example:

%copy%
```rust
let policy = RetryPolicy {
    max_retries: 5,
    initial_delay: Duration::from_millis(10),
    max_delay: Duration::from_sec(2),
    timeout: Duration::from_sec(10),
    strategy: RetryStrategy::ExponentialBackoff
};
let config = TopicProducerConfigBuilder::default()
    .delivery_semantic(DeliverySemantic::AtLeastOnce(policy))
    .build()?;
let producer = fluvio.topic_producer_with_config("greetings", config).await?;
```
In the above example, Fluvio Producer retries at most five times; all retries take a maximum of 10 seconds. The delay time distribution
is exponential. The first delay is 10ms, the second is 100ms, then 1000ms, and all others are 2000ms as it's defined as a maximum allowed delay.
