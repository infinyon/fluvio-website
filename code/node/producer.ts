import Fluvio from "@fluvio/client";
import { createInterface } from "readline";

// Create Fluvio Client Instance
const fluvio = new Fluvio();

// Create Readline Instance
const rl = createInterface({
  input: process.stdin,
  output: process.stdout,
});

const produce = async () => {
    // Connect the fluvio cluster;
    await fluvio.connect();

    // Create a topic producer;
    const producer = await fluvio.topicProducer("hello-node");
    console.log("Fluvio Producer created, waiting for input:\n\n>")
    // Relay terminal input to fluvio topic producer;
    rl.on("line", async (input) => {
        await producer.send("line", input);
    });
};

produce();