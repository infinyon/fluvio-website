```js
const flvConnection = await FluvioClient.connect();
let replica = await flvConnection.replica("my-topic", 0);
let len = await replica.produce("test");
console.log("OK: %d bytes sent", len);
```