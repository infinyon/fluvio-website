```TypeScript
// Connect to a Fluvio cluster;
const fluvio = await Fluvio.connect();

// Create a topic producer;
const producer = await fluvio.topicProducer('my-topic');

// Send a new topic record;
await producer.sendRecord("Hello, World! 🎉");
```