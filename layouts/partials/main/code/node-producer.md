```TypeScript
const fluvio = await Fluvio.connect();
const producer = await fluvio.topicProducer('greetings');
await producer.send("Hello", "World! 🎉");
```