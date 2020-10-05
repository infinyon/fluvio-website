```TypeScript
// Connect to a Fluvio cluster;
const fluvio = await Fluvio.connect();

// Consume a topic and partition;
const consumer = await fluvio.partitionConsumer('my-topic', 0);

// Listen for events;
await consumer.stream({
    // Index offset to begin stream;
    index: 0,
    // Stream all messages from beginning of topic;
    from: OffsetFrom.Beginning
}, async (data) => {
    // handle streaming data events;
    console.log(data) // Hello, World! 🎉
})
```