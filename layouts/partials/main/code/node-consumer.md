```TypeScript
const fluvio = await Fluvio.connect();
const consumer = await fluvio.partitionConsumer('greetings', 0);
await consumer.stream({ 
    index: 0, 
    from: OffsetFrom.Beginning 
}, async (record) => {
    console.log(record) // "Hello, World! 🎉"
})
```