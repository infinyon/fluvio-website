---
title: HTTP Connector
menu: HTTP
---

Fluvio's `http` connector allows you to periodically fetch data from an HTTP endpoint,
feeding the response body into a Fluvio topic. This is useful for monitoring APIs
continuously, and building streaming applications that react to new or updated info.
Note that this connector is _not_ intended for streaming HTTP endpoints, it instead
periodically sends HTTP requests and collects the response body as an event.

**Connector Name** - <a href="https://hub.docker.com/r/infinyon/fluvio-connect-http">infinyon/fluvio-connect-http</a>

| Versions  | Definition                                                               |
|:---------:|--------------------------------------------------------------------------|
|  0.2.0    | Add formatting parameter `output_parts` and `output_type`                |
|  0.1.0    | Initial implementation                                                   |

## Configuration Options

The HTTP connector supports the following configuration options:

- `endpoint` (required): The HTTP endpoint to send requests to
- `method`: The HTTP verb to use - i.e. `GET`, `PUT`, `POST`, `DELETE` (default: `GET`)
- `body`: The body to use in the HTTP request to the endpoint
- `interval`: The period (in seconds) between sending requests to the endpoint (default: `300`)
- `output_parts`: HTTP Response output Record parts - body | full (default: `body`)
- `output_type`: HTTP Response output Record type - text | json (default: `text`)

Additionally, the HTTP connector supports the following "Smart Connector" options:

- `filter`: The name of a SmartModule to use as a filter
- `map`: The name of a SmartModule to use as a map
- `arraymap`: The name of a SmartModule to use as an arraymap

## Deploy on InfinyOn Cloud

Getting started with InfinyOn Cloud is a simple:
1. Download [Fluvio CLI]
2. Sign-up for a [free InfinyOn Cloud account]. 
3. Login to InfinyOn Cloud via CLI: `fluvio cloud login`

#### Deploy an HTTP connector

Connectors are configured through confguration files. For example an HTTP connector that reads periodically from the <a href="https://catfact.ninja/fact" target="_blank">https://catfact.ninja</a> website has the following configuration parameters:

%copy%
```yaml
# connect.yml
version: 0.2.0
name: cat-facts
type: http
topic: cat-facts
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10
```

Use the CLI command to deploy the connector on InfinyOn Cloud:

%copy first-line%
```bash
$ fluvio connector create --config=./connect.yml
```

Once provisioned, the connector will run continuously and produce data records to the `cat-facts` every 10 seconds.

## Data Events

The HTTP connector generates one data event per HTTP response. The data depends on the endpoint you are accessing and can be formatted in multiple ways, as described below. 

### HTTP Response - Body (default)

By default, the HTTP connector produces the `body` of the HTTP response in JSON format:

%copy first-line%
```bash
$ fluvio consume cat-facts -B -d
{"fact":"A cat almost never meows at another cat, mostly just humans. Cats typically will spit, purr, and hiss at other cats.","length":116}
{"fact":"In one stride, a cheetah can cover 23 to 26 feet (7 to 8 meters).","length":65}
{"fact":"Phoenician cargo ships are thought to have brought the first domesticated cats to Europe in about 900 BC.","length":105}
```

### HTTP Response - Full

Use the `output-parts` parameter in `connect.yaml` to produce full HTTP responses. The header and the body are joined in a single record, where the`header is captured as text and body as JSON. See example:

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

### HTTP Response - Format

Set the `output_type` parameter to `json` if you want to convert the full HTTP response into JSON format. Note, that this field is only relevant when used in combination with `output_parts: full`. See below:

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

-> **JSON Output** "body" is encoded in quotes (\") as JSON String within due to HTTP Response in this example containing application/json body itself.

To convert only the body of the HTTP Response and ignore the header, set `output_parts` to `body` and  the `output_type` keep as `json`:

```json
{
  "body": "{\"fact\":\"A cat\\u2019s nose pad is ridged with a unique pattern, just like the fingerprint of a human.\",\"length\":87}"
}
```

## Deploy Locally (Advanced)

In a local environment, you have the option to delop a `Managed connector` or `Local connector`.

* `Managed connector`: follow the same instructions as in [Deploy on InfinyOn Cloud](#deploy-on-infinyon-cloud), and ensure that `fluvio profile` points to a local cluster.
* `Local Connector`: follow the instructions in the [next section](#deploy-local-connector).

### Deploy Local connector

Local connectors are deployed as Docker containers. You may launch a local connector with the following command:

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

Passing config options to a Local Connector, requires command-line arguments. 
In the above `docker` command, all arguments before the `--` are used by
Docker itself, and all arguments after the `--` are passed to the connector.

Importantly, when using a Local Connector, you _must_ include the first two arguments: a fluvio profile, and a docker container. For example:

- `-v"$HOME/.fluvio/config:/home/fluvio/.fluvio/config"`
    - Puts your `~/.fluvio/config` into the container, so the connector can reach your Fluvio cluster
- `-t infinyon/fluvio-connect-http`
    - Tells docker which image to use for the connector


[Fluvio CLI]: /download
[free InfinyOn Cloud account]: https://infinyon.cloud/signup
