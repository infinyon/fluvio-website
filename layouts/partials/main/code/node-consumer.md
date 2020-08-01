```js
const flvConnection = await FluvioClient.connect();
let replica = await flvConnection.replica("my-topic", 0);
replica.consume(
    {offset: "earliest"},
    emitter.emit.bind(emitter)
);

emitter.on('data', (msg) => {
    console.log(msg);
})
```