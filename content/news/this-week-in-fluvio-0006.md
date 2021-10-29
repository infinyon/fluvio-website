---
title: "This Week in Fluvio #6"
date: 2021-09-16
weight: 20
---

Welcome to the sixth edition of This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

## Release of Fluvio `0.9.6` and `0.9.7`

### Custom record formatting for Consumer

The Fluvio CLI's consumer now allows you to provide your own formatting string,
which it will use as a template for printing records to the screen. The formatting
string may contain the variables `{{offset}}`, `{{key}}`, and `{{value}}`, and
will substitute each record's contents into the format string.

This functionality is provided by the `--format` option on the consumer. Here is
an example where we can output each record in a CSV format:

```bash
$ fluvio consume my-topic --format="{{offset}},{{key}},{{value}}"
Consuming records from the beginning of topic 'my-topic'
0,null,Apple
1,null,Banana
2,null,Cranberry
3,null,Date
```

You can get pretty creative with this. For example, you could use a format string
to capture your record's contents and metadata in a JSON object by doing this:

```bash
$ fluvio consume my-topic -B --format='{"offset":{{offset}},"key":"{{key}}","value":"{{value}}"}'
Consuming records from the beginning of topic 'my-topic'
{"offset":0,"key":"null","value":"Apple"}
{"offset":1,"key":"null","value":"Banana"}
{"offset":2,"key":"null","value":"Cranberry"}
```

### Pretty status printout for cluster installation

When using the Fluvio CLI to provision your own local Fluvio cluster, you'll now
see the installation progress indicated by neat and uniform progress messages.

Before:

```bash
$ fluvio cluster start --local
checking fluvio crd attempt: 0
fluvio crd installed
Starting sc server
Trying to connect to sc at: localhost:9003, attempt: 0
Got updated SC Version0.9.6
Connection to sc suceed!
Launching spu group with: 1
Starting SPU (1 of 1)
All SPUs(1) are ready
Setting local profile
Successfully installed Fluvio!
```

After:

```bash
$ fluvio cluster start --local
📝 Running pre-flight checks
     ✅ Supported helm version is installed
     ✅ Supported kubernetes version is installed
     ✅ Kubernetes config is loadable
     ✅ Fluvio system charts are installed
🖥️ Starting SC server
     ✅ SC Launched
🤖 Launching SPU Group with: 1
     🤖 Starting SPU: (1/1)
     ✅ SPU group launched (1)
💙 Confirming SPUs
     ✅ All SPUs confirmed
👤 Profile set
🎯 Successfully installed Fluvio!
```

The emojis let you know that we mean business!

### Proper error message for invalid Topic name

There are some limitations on what a Topic may be named, and now when you
try to use an invalid Topic name, you'll see a descriptive error message
that tells you what characters are and are not allowed.

For example, you cannot use spaces in a topic name, so if you try to create
a topic called "hello world" you'll now get the following error:

```bash
$ fluvio topic create "hello world"
Error:
   0: Invalid argument: Topic name must only contain lowercase alphanumeric characters or '-'.
```

## Conclusion

For the full list of changes this week, be sure to check out [our CHANGELOG]. If you have any
questions or are interested in contributing, be sure to [join our Discord channel] and
come say hello!

### Shoutout to the Rustconf shoutout!

Thanks [`@nellshamrell`] for the project updates shoutout for This Week in Fluvio!
This-Week-in-Rust was a huge inspiration for this newsletter, and we thank you for
your tireless work assembling great Rust content and for helping us spread the word
about Fluvio!

<a href="https://youtu.be/OZPXhmy-wVw?t=761">
<img src="/news/images/0006/rustconf.png" style="width:600px" />
</a>

[`@nellshamrell`]: https://github.com/nellshamrell

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[wasmtime API since 0.28]: https://github.com/alexcrichton/rfcs-2/blob/new-api/accepted/new-api.md
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[join our Discord channel]: https://discordapp.com/invite/bBG2dTz
