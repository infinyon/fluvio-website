from fluvio import (Fluvio, Offset)
fluvio = Fluvio.connect()
partition = 0
consumer = fluvio.partition_consumer("hello-python", partition)
for i in consumer.stream(Offset.beginning()):
    print("Received message: %s" % i.value_string())