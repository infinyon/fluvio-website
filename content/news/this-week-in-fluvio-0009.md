---
title: "This Week in Fluvio #9"
date: 2021-10-08
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## New Release - Fluvio v0.9.10

### SmartStreams error handling improvements

SmartStreams users will now see error messages if there any problems loading their WASM modules.

Previously, these errors would occur within the Fluvio cluster while trying to consume from a topic, and the connection would abruptly close without an explanation.

Before

```bash
$ fluvio consume my-topic -B --filter=invalid-smartstream.wasm
Consuming records from the beginning of topic 'echo'
Consumer stream has closed
```

from: https://github.com/infinyon/fluvio/issues/1143#issuecomment-873432660

Now the client will report a more helpful error message

```bash
$ fluvio consume my-topic -B --filter=invalid-smartstream.wasm
Consuming records from the beginning of topic 'echo'
Error:
   0: Fluvio client error
   1: SmartStream error
   2: WASM Module error: failed to parse WebAssembly module

```

### Producer error handling improvements

Producer connection errors that occur within the Fluvio cluster will also report a more user friendly error message.

Before

```bash
$ fluvio produce my-topic
[...]
Error:
   0: Consumer Error
   1: Fluvio client error
   2: Fluvio socket error
   3: time out in serial: 0 request: 1

```

After

```bash
$ fluvio produce topic
[...]
Timed out while waiting on socket response
```

## New feature

### Managed Connectors

We have always wanted to provide our users with the capability to work with their data (wherever it is) in a meaningful way without the need to write and maintain custom code. Our new feature (which we've been calling `Managed Connectors` or `connectors` for short) provides the structure to accomplish this feat.

The idea is that a developer can create a custom connector that will do the following:
* Read in from your data source and store in a Fluvio topic
* Send out from a Fluvio topic to your data store

These connectors can then be re-used by anyone without the need to maintain custom code.

### First look

The rest of this post is first look at a connector in action.

We create a new connector, with configurable details stored in a this example config file `config.yaml`.

%copy%

```yaml
version: 0.1.0
name: my-test-connector
type: test-connector
topic: my-test-connector
direction: source
parameters:
  topic: my-test-connector
```


%copy first-line%

```bash
$ fluvio cluster connector create --config config.yaml
```

%copy first-line%

```bash
$ fluvio cluster connector list

-------------
 NAME               STATUS
 my-test-connector  Running
```

The test connector produces to a topic `my-test-connector`, where each record says `Hello, Fluvio! - #` where `#` is a number that counts up.

%copy first-line%

```bash
$ fluvio consume my-test-connector --tail -d
Consuming records starting 10 from the end of topic 'my-test-connector'
Hello, Fluvio! - 166
Hello, Fluvio! - 167
Hello, Fluvio! - 168
Hello, Fluvio! - 169
Hello, Fluvio! - 170
Hello, Fluvio! - 171
Hello, Fluvio! - 172
Hello, Fluvio! - 173
Hello, Fluvio! - 174
Hello, Fluvio! - 175
```

To stop the connector, you need to delete it:

%copy first-line%

```bash
$ fluvio cluster connector delete my-test-connector
```

The details about connectors and documentation for creating your own connectors are coming soon! We've created the section dedicated to [connectors].

If you would like to help us prioritize what connectors to create, or if this sounds interesting and you'd like to talk to us about connectors then please get in touch with us!

Communicate with us on [Github Discussions] or join [our Discord channel] and come say hello!

---

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
[connectors]: {{<ref "/connectors" >}}