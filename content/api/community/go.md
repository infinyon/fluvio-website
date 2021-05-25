---
title: Go
weight: 19
---

The [fluvio-go] client is a community
project put together by [@avinassh].

It's still under development but still very exciting!

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

### Sending

To send into a topic do:
```go
val := fmt.Sprintf("(from Go) %d (%s)", i, time.Now().String())
err = producer.SendString(fmt.Sprintf("%d", i), val)
```
