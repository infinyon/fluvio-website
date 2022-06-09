import Fluvio, { Offset } from "@fluvio/client";

// Create Fluvio Client Instance
const fluvio = new Fluvio();

const consume = async () => {
    // Connect the fluvio cluster;
    await fluvio.connect();

    const partition = 0;

    // Create Topic Consumer
    const consumer = await fluvio.partitionConsumer("hello-node", partition);
    console.log("Fluvio Consumer created, listening for events:\n\n")
    const stream = await consumer.createStream(Offset.FromBeginning());

    for await (const record of stream) {
        const key = record.keyString();
        const value = record.valueString();
        console.log(`Received record: Key=${key}, value=${value}`);
    }
};

consume();