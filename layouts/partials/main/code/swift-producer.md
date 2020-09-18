```swift
var client = FluvioClient();
var producer = client.producer("topic",0);
producer.produce("hello world");
```