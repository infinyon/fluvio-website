---
title: Outbound Slack Connector
menu: Slack
connector:
  name: "infinyon/fluvio-connect-slack-sink"
  link: "https://github.com/infinyon/fluvio-connectors/tree/main/rust-connectors/sinks/slack"
---

The Slack Connector will stringify any record coming from a
fluvio stream and `POST` it to the Slack via Webhook.

Follow the Slack docs for creating a webhook: https://api.slack.com/messaging/webhooks

## Common config values

%copy%
```yaml
type: slack-sink
```

%copy%
```yaml
version: 0.2.0
```

## Secrets

### `WEBHOOK_URL`
*required*

The Slack webhook URL should look like this:

```
https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX
```

#### Example connector config

{{<code file="code-blocks/yaml/connectors/outbound-examples/outbound-slack.yaml" lang="yaml" copy=true >}}

## Data Events

Data is transformed to a string in a [UTF-8 lossy conversion] as is. If you'd like data in a specific format, you should use a smartmodule with your connector.


[UTF-8 lossy conversion]: https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy
