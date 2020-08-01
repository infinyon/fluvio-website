```js
const flvConnection = await FluvioClient.connect();
let replica = await flvConnection.replica("my-topic", 0);
await replica.produce("test");
```