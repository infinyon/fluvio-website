```TypeScript
const fluvio = await Fluvio.connect();
const producer = await fluvio.topicProducer('greetings');
await producer.sendRecord("Hello, World! 🎉");
```