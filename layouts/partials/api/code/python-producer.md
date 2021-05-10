```python
fluvio = Fluvio.connect()
producer = fluvio.topic_producer("greetings")
producer.send("Hello", "World! 🎉"))
```
