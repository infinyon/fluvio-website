---
title: "This Week in Fluvio #16"
date: 2021-12-01
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

## New Release

### Fluvio v0.9.14

#### Connector logs
<<<<<<< HEAD
Logs from connectors are now accessible from the CLI
=======
Logs from [connectors](/connectors) are now accessible from the CLI
>>>>>>> master

%copy first-line%
```shell
fluvio connector logs <connector name>
```

In this example output, we can see some error output from the connector that would have previously been inaccessible.

Example:

%copy first-line%
```shell
$ fluvio connector logs my-test-mqtt -f

2021-12-15T05:20:21.770042Z ERROR mqtt: Mqtt error MqttState(Deserialization(PayloadSizeLimitExceeded(16635)))
2021-12-15T05:20:27.684853Z ERROR mqtt: Mqtt error MqttState(Deserialization(PayloadSizeLimitExceeded(16635)))
```

#### CLI Consumer wait spinner

Using the CLI consumer now has a new spinner to give feedback about the stream waiting for data.

Here's a short clip of the spinner in action with streaming data.

<script id="asciicast-pxtVvacxOTE3XWCQFzTCW3DLy" src="https://asciinema.org/a/pxtVvacxOTE3XWCQFzTCW3DLy.js" async></script>

##### Change to multi-word subcommands

Given that these are still long commands, we've added aliases too


| Previous subcommand | New subcommand | New alias |
|---------------------|----------------|-----------|
| smartmodule         | smart-module   | sm        |
| tableformat         | table-format   | tf        |
<<<<<<< HEAD
| derivedstream       | derived-stream | df        |
=======
| derivedstream       | derived-stream | ds        |
>>>>>>> master

##### CLI shell autocompletions
This actually isn't new, but it was hidden. Thanks to [bohlmannc](https://github.com/bohlmannc) for helping us resolve this!

```
$ fluvio completions -h
fluvio-completions 0.0.0
Generate command-line completions for Fluvio

Run the following two commands to enable fluvio command completions.

Open a new terminal for the changes to take effect.

$ fluvio completions bash > ~/fluvio_completions.sh 
$ echo "source ~/fluvio_completions.sh" >> ~/.bashrc

USAGE:
    fluvio completions <SUBCOMMAND>

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    bash    Generate CLI completions for bash
    fish    Generate CLI completions for fish
    help    Prints this message or the help of the given subcommand(s)
```

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions