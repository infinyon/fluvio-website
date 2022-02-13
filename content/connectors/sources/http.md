---
title: HTTP Connector
menu: HTTP
---

## Overview

Fluvio's `http` connector allows you to periodically fetch data from an HTTP endpoint,
feeding the response body into a Fluvio topic. This is useful for monitoring APIs
continuously, and building streaming applications that react to new or updated info.
Note that this connector is _not_ intended for streaming HTTP endpoints, it instead
periodically sends HTTP requests and collects the response body as an event.

## Configuration Options

The HTTP connector supports the following configuration options:

- `endpoint` (required): The HTTP endpoint to send requests to
- `method`: The HTTP verb to use - i.e. `GET`, `PUT`, `POST`, `DELETE` (default: `GET`)
- `body`: The body to use in the HTTP request to the endpoint
- `interval`: The period (in seconds) between sending requests to the endpoint (default: `300`)
- `output_parts`: HTTP Response output parts - body | full (default: `body`)
- `output_type`: HTTP Response output Record type - text | json (default: `text`)

Additionally, the HTTP connector supports the following "Smart Connector" options:

- `filter`: The name of a SmartModule to use as a filter
- `map`: The name of a SmartModule to use as a map
- `arraymap`: The name of a SmartModule to use as an arraymap

### As a Managed Connector

When deploying the HTTP Connector in Kubernetes alongside the Fluvio cluster, we pass
configuration options via a `connect.yml` file. For the HTTP connector, this would look
something like this:

%copy%
```yaml
# connect.yml
api_version: v1
name: cat-facts
type: http
topic: cat-facts
create_topic: true
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10
```

To run as a Managed Connector, use the `fluvio connector` command:

%copy first-line%
```bash
$ fluvio connector create --config=./connect.yml
```

### As a Local Connector

When using the HTTP Connector locally, it is deployed as a Docker container.

You may launch it with the following command:

%copy%
```bash
docker run -d --name="my-http" \
    -v"$HOME/.fluvio/config:/home/fluvio/.fluvio/config" \
    -t infinyon/fluvio-connect-http \
    -- \
    --endpoint="https://catfact.ninja/fact" \
    --fluvio-topic="cat-facts" \
    --interval=10
```

As you can see, when passing config options to a Local Connector, we use command-line
arguments. In the above `docker` command, all arguments before the `--` are used by
Docker itself, and all arguments after the `--` are passed to the connector.

Importantly, when using a Local Connector, you _must_ include the first two arguments,
otherwise it will not work. They are used for the following:

- `-v"$HOME/.fluvio/config:/home/fluvio/.fluvio/config"`
    - Puts your `~/.fluvio/config` into the container, so the connector can reach your Fluvio cluster
- `-t infinyon/fluvio-connect-http`
    - Tells docker which image to use for the connector

## Data Events

The data events by default from the HTTP connector are the contents of the HTTP body
of each response. Therefore, the format will be different depending on what
endpoint you specify and the type of content that endpoint returns.

In our running example with CatFacts, the data events that are sent to the
topic will be the JSON response received by the endpoint, such as:

%copy first-line%
```bash
$ fluvio consume cat-facts -B -d
{"fact":"A cat almost never meows at another cat, mostly just humans. Cats typically will spit, purr, and hiss at other cats.","length":116}
{"fact":"In one stride, a cheetah can cover 23 to 26 feet (7 to 8 meters).","length":65}
{"fact":"Phoenician cargo ships are thought to have brought the first domesticated cats to Europe in about 900 BC.","length":105}
```

Alternatively the events can be set to carry the full HTTP response

When `output_parts` set to `full` while the `output_type` is kept as the default `text`:

%copy first-line%
```bash
$ fluvio consume cat-facts -B -d
HTTP/1.1 200 OK
server: nginx
date: Fri, 28 Jan 2022 19:29:38 GMT
content-type: application/json
transfer-encoding: chunked
connection: keep-alive
vary: Accept-Encoding
cache-control: no-cache, private
x-ratelimit-limit: 100
x-ratelimit-remaining: 78
access-control-allow-origin: *
set-cookie: XSRF-TOKEN=zz; expires=Fri, 28-Jan-2022 21:29:38 GMT; path=/; samesite=lax
set-cookie: cat_facts_session=zz; expires=Fri, 28-Jan-2022 21:29:38 GMT; path=/; httponly; samesite=lax
x-frame-options: SAMEORIGIN
x-xss-protection: 1; mode=block
x-content-type-options: nosniff

{"fact":"In relation to their body size, cats have the largest eyes of any mammal.","length":73}
```

## JSON Record Output

To get a `full` JSON `output_parts` we set `output_type` as `json`:

```json
{
  "status": {
    "version": "HTTP/1.1",
    "code": 200,
    "string": "OK"
  },
  "header": {
    "date": "Sun, 13 Feb 2022 08:12:18 GMT",
    "transfer-encoding": "chunked",
    "vary": "Accept-Encoding",
    "x-ratelimit-limit": "100",
    "access-control-allow-origin": "*",
    "set-cookie": [
      "XSRF-TOKEN=xx; expires=Sun, 13-Feb-2022 10:12:18 GMT; path=/; samesite=lax",
      "cat_facts_session=yy; expires=Sun, 13-Feb-2022 10:12:18 GMT; path=/; httponly; samesite=lax"
    ],
    "content-type": "application/json",
    "x-ratelimit-remaining": "97",
    "x-xss-protection": "1; mode=block",
    "server": "nginx",
    "x-frame-options": "SAMEORIGIN",
    "x-content-type-options": "nosniff",
    "cache-control": "no-cache, private",
    "connection": "keep-alive"
  },
  "body": "{\"fact\":\"The chlorine in fresh tap water irritates sensitive parts of the cat's nose. Let tap water sit for 24 hours before giving it to a cat.\",\"length\":134}"
}
```

_Note: JSON Output "body" is encoded (\" quotes) as JSON String within due to HTTP Response in this example containing application/json body itself_

And to only get `body` of the HTTP Response part as `output_parts` in JSON,

We set `output_parts` to `body` and `output_type` to `json`:

```json
{
  "body": "{\"fact\":\"A cat\\u2019s nose pad is ridged with a unique pattern, just like the fingerprint of a human.\",\"length\":87}"
}
````