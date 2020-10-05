```TypeScript
const fluvio = await Fluvio.connect();
const producer = await fluvio.topicProducer('my-topic');
await producer.sendRecord("Hello, World! 🎉");
```