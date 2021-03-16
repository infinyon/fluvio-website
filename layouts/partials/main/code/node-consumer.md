```TypeScript
const fluvio = await Fluvio.connect();
const consumer = await fluvio.partitionConsumer('greetings', 0);
const stream = await consumer.createStream(Offset.FromBeginning());

for await (const record of stream) {
    const key = record.keyString();
    const value = record.valueString();
    console.log(`Consumed record: Key=${key}, value=${value}`);
}
```