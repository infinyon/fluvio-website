```swift
var client = FluvioClient();
var consumer = client.consumer("topic",0);
var records = consumer.fetch(Fetch.Earliest);

for record in records {
    print(record);
}
```