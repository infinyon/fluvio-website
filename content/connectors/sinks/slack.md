---
title: Slack
section: Sink
---

## Overview

The Slack Connector is quite simple. It will stringify any record coming from a
fluvio stream and `POST` it to the slack via a [slack webhook url]

[slack webhook url]: https://api.slack.com/messaging/webhooks

## Configuration Options

%copy%
```yaml
# slack-connector.yml
name: my-slack-sink
version: 0.2.0
type: slack-sink
topic: slack-topic
parameters:
  webhook-url: ...
secrets:
  WEBHOOK_URL: ...
```

You need either `webhook-url` in the `params` section or `WEBHOOK_URL` in the
secrets.

This configuration file is used together with the `fluvio connector create` command, like so:

%copy first-line%
```bash
$ fluvio connector create --config=./slack-connector.yml
```

## Data Events

Data is transformed to a string in a [UTF-8 lossy conversion] as is. If you'd like data in a specific format, you should use a smartmodule with your connector.


[UTF-8 lossy conversion]: https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy
