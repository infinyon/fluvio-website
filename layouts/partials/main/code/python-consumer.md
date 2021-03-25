```python
from fluvio import (Fluvio, Offset)
consumer = fluvio.partition_consumer("greetings", 0)
for i in consumer.stream(Offset.beginning()):
    key = i.key_string()
    value = i.value_string()
    print("Consumed record: Key=%s, value=%s" % (key, value))
```
