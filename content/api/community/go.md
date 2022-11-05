---
title: Go
weight: 19
---

The [fluvio-go] client is a community
project put together by [@avinassh].

It's still under development but very exciting nonetheless!

[fluvio-go]: https://github.com/avinassh/fluvio-go 
[@avinassh]: https://github.com/avinassh

## Connecting

To connect to the fluvio cluster do:
```go
f, err := fluvio.Connect()
```

## Producer

To create a `TopicProducer` do:
```go
producer, err := f.TopicProducer("hello-go")
```

### Send

To send into a topic do:
```go
val := fmt.Sprintf("(from Go) %d (%s)", i, time.Now().String())
err = producer.SendString(fmt.Sprintf("%d", i), val)
```

## Consumer 

To get a consumer, do:
```go
partitionConsumer, err := f.PartitionConsumer("hello-go", 0)
```

### Stream

To get a stream from the consumer do:
```go
stream, err := partitionConsumer.Stream(fluvio.NewOffsetFromBeginning(0))
for {
     r, err := stream.Next()
     fmt.Printf("Got record: key=%s, value=%s\n", string(r.Key), string(r.Value))
}
```

## Smart Streams

### Filter

Create a consumer config with the wasm file and get the filtered stream:
```go
wasmFile := "example/filter.wasm"
config, err := fluvioClient.ConsumerConfigWithWasmFilter(wasmFile)
stream, err := partitionConsumer.StreamWithConfig(fluvio.NewOffsetFromBeginning(0), config)
for {
     r, err := stream.Next()
     fmt.Printf("Got record: key=%s, value=%s\n", string(r.Key), string(r.Value))
}
```

