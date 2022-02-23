---
title: "This Week in Fluvio #23"
date: 2022-02-17
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## New Release - Fluvio v0.9.20

### Connector config key deprecation: `create_topic`

This release deprecates the `create_topic` key from the connectors config. You can remove this key from your configs. The default behavior is to create the topic given by `topic`.

```diff
# connect.yml
api_version: v1
name: cat-facts
type: http
topic: cat-facts
- create_topic: true
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10
  output_parts: body
  output_type: text
```


### Update connector config

When developing with connectors, you may need to reconfigure your settings. This is now easier to do.

Let's work through an example using the following config.

%copy%
```yaml
# connect.yml
api_version: v1
name: cat-facts
type: http
topic: cat-facts
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10
  output_parts: body
  output_type: text
```

First we create our connector

%copy first-line%
```shell
$ fluvio connector create --config connect.yml
```

As expected, we are getting cat facts of different lengths.

%copy first-line%
```shell
$ fluvio consume cat-facts
Consuming records from the end of topic 'cat-facts'. This will wait for new records
{"fact":"The average cat food meal is the equivalent to about five mice.","length":63}
{"fact":"The first commercially cloned pet was a cat named \"Little Nicky.\" He cost his owner $50,000, making him one of the most expensive cats ever.","length":140}
{"fact":"Mohammed loved cats and reportedly his favorite cat, Muezza, was a tabby. Legend says that tabby cats have an \u201cM\u201d for Mohammed on top of their heads because Mohammad would often rest his hand on the cat\u2019s head.","length":210}
⠐
```

Later on I decided that I want to change my endpoint to add a query parameter. We no longer want facts longer than 50 characters.

```diff
- endpoint: https://catfact.ninja/fact
+ endpoint: https://catfact.ninja/fact?max_length=50
```

Running `fluvio connector update` will handle updating your connector settings. This will delete your existing connector, and recreate with the config settings.

%copy first-line%
```shell
$ fluvio connector update --config connect.yml
```

After the connector restarts, we see that our output reflects the changes made in our config.

%copy first-line%
```shell
$ fluvio consume cat-facts
Consuming records from the end of topic 'cat-facts'. This will wait for new records
{"fact":"A cat's field of vision is about 200 degrees.","length":45}
{"fact":"Blue-eyed, pure white cats are frequently deaf.","length":47}
{"fact":"Cats have supersonic hearing","length":28}
⠖
```

### CLI Version pinning

Fluvio CLI Channels now fully support version pinning. This is done by preventing users who are currently using a version channel from running `fluvio update`. Users will get a short message informing them how to switch to a different version channel.

Example:
If a user switched directly to version `0.9.20` (as opposed to running the `stable` or `latest` channel) then running `fluvio update` should not update to a new release.

First we create our version channel. (This could be any of our previous versions)

%copy first-line%
```shell
$ fluvio version create 0.9.20
🎣 Fetching '0.9.20' channel binary for fluvio...
⏳ Downloading Fluvio CLI with latest version: 0.9.19...
🔑 Downloaded and verified package file
✅ Successfully updated /home/user/.fluvio/bin/fluvio-0.9.20
```

And switch over to it

%copy first-line%
```shell
$ fluvio version switch 0.9.20
Switched to release channel "0.9.20"
```

Now if we try to update, we'll get an error message and some short instructions for how to switch to a new version channel.

%copy first-line%
```shell
$ fluvio update
Unsupported Feature: The `fluvio update` command is not supported when using a pinned version channel. To use a different version run:
  fluvio version create X.Y.Z
  fluvio version switch X.Y.Z
```

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions