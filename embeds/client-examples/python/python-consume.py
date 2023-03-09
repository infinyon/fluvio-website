#!/usr/bin/env python
from fluvio import Fluvio, Offset

TOPIC_NAME = "python-data"
PARTITION = 0

if __name__ == "__main__":
    # Connect to cluster
    fluvio = Fluvio.connect()

    # Consume last 10 records from topic
    consumer = fluvio.partition_consumer(TOPIC_NAME, PARTITION)
    for idx, record in enumerate( consumer.stream(Offset.from_end(10)) ):
        print("{}".format(record.value_string()))
        
        if idx >= 9:
            break