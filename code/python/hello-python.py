#!/usr/bin/env python
import os
from datetime import datetime
from fluvio import Fluvio, Offset

TOPIC_NAME = "hello-python"
PARTITION = 0

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
    for record in consumer.stream(Offset.from_end(0)):
        print("{}".format(record.value_string()))
        break
