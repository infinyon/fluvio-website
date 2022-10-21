---
title: Outbound Slack Connector
menu: Slack
---

## Overview

The Slack Connector is quite simple. It will stringify any record coming from a
fluvio stream and `POST` it to the slack via a [slack webhook url]

[slack webhook url]: https://api.slack.com/messaging/webhooks

## Connector config `parameters`

### `webhook-url`

## Connector config `secrets`

### `WEBHOOK_URL`

You need either `webhook-url` in the `params` section or `WEBHOOK_URL` in the
secrets.

#### Example connector config 
%copy%

{{<code file="code-blocks/yaml/connectors/outbound-examples/outbound-slack.yaml" lang="yaml" copy=true >}}

## Data Events

Data is transformed to a string in a [UTF-8 lossy conversion] as is. If you'd like data in a specific format, you should use a smartmodule with your connector.


[UTF-8 lossy conversion]: https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy
