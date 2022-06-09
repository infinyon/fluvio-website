from fluvio import Fluvio
fluvio = Fluvio.connect()
producer = fluvio.topic_producer("hello-python")
while True:
    line = input('> ')
    producer.send_string(line)