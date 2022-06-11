import Fluvio, { Offset, Record } from "@fluvio/client";
import { createInterface } from "readline";

const TOPIC_NAME = "hello-node";
const PARTITION = 0;

// Create Fluvio Client Instance
const fluvio = new Fluvio();

async function createTopic() {
  try {
    // Connect to the Fluvio cluster
    console.log("Connecting client to fluvio");
    await fluvio.connect();

    // Create admin client;
    const admin = await fluvio.admin();

    // Create topic
    console.log("Creating topic");
    await admin.createTopic(TOPIC_NAME);
  } catch (ex) {
    console.log("Topic already exists", ex);
  }
}

const produce = async () => {
  // Connect to the Fluvio cluster
  console.log("Connecting client to fluvio");
  await fluvio.connect();

  // Create a topic producer;
  const producer = await fluvio.topicProducer(TOPIC_NAME);
  await producer.send("example-key", "Hello World!  - Time is " + Date.now());
};

const consume = async () => {
  try {
    // Connect to the fluvio cluster referenced in the cli profile.
    await fluvio.connect();

    // Create partition consumer
    const consumer = await fluvio.partitionConsumer(TOPIC_NAME, PARTITION);

    console.log("read from the end");
    await consumer.stream(Offset.FromEnd(), async (record: Record) => {
      // handle record;
      console.log(`Key=${record.keyString()}, Value=${record.valueString()}`);
      process.exit(0);
    });
  } catch (ex) {
    console.log("error", ex);
  }
};

createTopic();
produce();
consume();

