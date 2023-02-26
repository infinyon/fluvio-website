---
title: Inbound HTTP Connector
menu: HTTP
connector:
  name: "Source"
  link: "https://github.com/infinyon/http-connector"
---

Fluvio's `http` connector allows you to periodically fetch data from an HTTP endpoint
and feed the HTTP response into a Fluvio topic.

This is useful for monitoring APIs
continuously and building streaming applications that react to new or updated info.


Note that this connector is _not_ intended for streaming HTTP endpoints, it instead
periodically sends HTTP requests and collects the response as an event.

## Example connector config

{{<code file="embeds/connectors/inbound-examples/inbound-http.yaml" lang="yaml" copy=true >}}

## Example connector config with transformation 

{{<code file="embeds/connectors/inbound-examples/inbound-http-transformation.yaml" lang="yaml" copy=true >}}


## Configuration
| Option       | default                    | type   | description                                                                                |
| :------------| :--------------------------| :----- | :----------------------------------------------------------------------------------------- |
| interval     | 10s                        | String | Interval between each HTTP Request. This is in the form of "1s", "10ms", "1m", "1ns", etc. |
| method       | GET                        | String | GET, POST, PUT, HEAD                                                                       |
| endpoint     | -                          | String | HTTP URL endpoint                                                                          |
| headers      | -                          | String | Request header(s) Key=Value pairs                                                          |
| body         | -                          | String | Request body e.g. in POST                                                                  |
| user-agent   | "fluvio/http-source 0.1.0" | String | Request user-agent                                                                         |
| output_type  | text                       | String | `text` = UTF-8 String Output, `json` = UTF-8 JSON Serialized String                        |
| output_parts | body                       | String | `body` = body only, `full` = all status, header and body parts                             |

## Record Type Output
| Matrix                                                      | Output                                  |
| :---------------------------------------------------------- | :-------------------------------------- |
| output_type = text (default), output_parts = body (default) | Only the body of the HTTP Response      |
| output_type = text (default), output_parts = full           | The full HTTP Response                  |
| output_type = json, output_parts = body (default)           | Only the "body" in JSON struct          |
| output_type = json, output_parts = full                     | HTTP "status", "body" and "header" JSON |


