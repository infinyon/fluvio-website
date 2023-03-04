#!/usr/bin/env python
import os
from datetime import datetime
from fluvio import Fluvio, Offset, ConsumerCoonfig

TOPIC_NAME = "hello-python-smartmodule"
PARTITION = 0

# This is an example of a basic Fluvio workflow in Python
#
# 1. Create a topic to store data in via CLI
# 2. Establish a connection to the Fluvio cluster
# 3. Create a producer and send some bytes
# 4. Create a consumer, and stream the data back
if __name__ == "__main__":
    # Currently the Python client does not support creating topics
    # Using the fluvio CLI
    os.popen("fluvio topic create {}".format(TOPIC_NAME))

    # Connect to cluster
    fluvio = Fluvio.connect()

    # Produce to topic
    producer = fluvio.topic_producer(TOPIC_NAME)
    producer.send_string("Hello World! - Time is: {}".format(datetime.now()))

    # Consume from topic
    # We're just going to get the last record
    consumer = fluvio.partition_consumer(TOPIC_NAME, PARTITION)


    # Create a ConsumerConfig using your "uppercase-map" smartmodule
    config = ConsumerConfig()
    config.smartmodule(name="uppercase-map")

    for record in consumer.stream_with_config(Offset.from_end(0), config):
        print("{}".format(record.value_string()))
        break
