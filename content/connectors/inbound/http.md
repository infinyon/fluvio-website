---
title: HTTP Connector
menu: HTTP
connector:
  name: "infinyon/fluvio-connect-http"
  link: "https://github.com/infinyon/fluvio-connectors/tree/main/rust-connectors/sources/http"
---

Fluvio's `http` connector allows you to periodically fetch data from an HTTP endpoint,
feeding the HTTP response into a Fluvio topic. This is useful for monitoring APIs
continuously, and building streaming applications that react to new or updated info.
Note that this connector is _not_ intended for streaming HTTP endpoints, it instead
periodically sends HTTP requests and collects the response as an event.


## Configuration Options

The HTTP connector supports the following configuration options:

- `endpoint` (required): The HTTP endpoint to send requests to
- `method`: The HTTP verb to use - i.e. `GET`, `PUT`, `POST`, `DELETE` (default: `GET`)
- `body`: The body to use in the HTTP request to the endpoint
- `interval`: The period (in seconds) between sending requests to the endpoint (default: `300`)
- `output_parts`: HTTP Response output Record parts - body | full (default: `body`)
- `output_type`: HTTP Response output Record type - text | json (default: `text`)

Additionally, the HTTP connector supports the following [SmartModules]({{<ref "/smartmodules/" >}}) options:

- `filter`: The name of the _filter_ SmartModule
- `map`: The name of the _map_ SmartModule
- `filter-map`: The name of the _filter-map_ SmartModule
- `arraymap`: The name of the _arraymap_ SmartModule

## Deploy on InfinyOn Cloud

Getting started with InfinyOn Cloud is a simple:
1. Download [Fluvio CLI]
2. Sign-up for a [free InfinyOn Cloud account].
3. Login to InfinyOn Cloud via CLI: `fluvio cloud login`

#### Deploy an HTTP connector

Connectors are defiend through configuration files. Create a `connect.yml` HTTP configuration file that instructs the **inbound connector** to read periodically from the {{<link "https://catfact.ninja/fact" "https://catfact.ninja" >}} website:

%copy%
```yaml
version: 0.3.0
name: cat-facts
type: http-source
topic: cat-facts
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10s
```

Use the CLI command to deploy the connector on InfinyOn Cloud:

%copy first-line%
```bash
$ fluvio connector create --config=./connect.yml
```

Once provisioned, the connector will run continuously and produce data records to the `cat-facts` every 10 seconds.

## Data Events

The HTTP connector generates one data event per HTTP response. The data depends on the endpoint you are accessing and can be formatted in multiple ways, as described below.

### HTTP - Body

By default, the HTTP connector produces the `body` of the HTTP response in JSON format:

%copy first-line%
```bash
$ fluvio consume cat-facts -T -d
{"fact":"A cat almost never meows at another cat, mostly just humans. Cats typically will spit, purr, and hiss at other cats.","length":116}
{"fact":"In one stride, a cheetah can cover 23 to 26 feet (7 to 8 meters).","length":65}
{"fact":"Phoenician cargo ships are thought to have brought the first domesticated cats to Europe in about 900 BC.","length":105}
```


### HTTP - Header/Body

Use the `output_parts` parameter to produce full HTTP responses:

%copy%

{{< highlight yaml "hl_lines=9" >}}
version: 0.3.0
name: cat-facts
type: http-source
topic: cat-facts
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10s
  output_parts: full
{{</ highlight >}}

This configruation, instructs the connector to product `header` and `body` to `cat-facts` topic.

%copy first-line%
```bash
$ fluvio consume cat-facts -T=1 -d
Consuming records starting 1 from the end of topic 'cat-facts'
HTTP/1.1 200 OK
server: nginx
date: Thu, 28 Apr 2022 14:05:43 GMT
content-type: application/json
transfer-encoding: chunked
connection: keep-alive
vary: Accept-Encoding
cache-control: no-cache, private
x-ratelimit-limit: 100
x-ratelimit-remaining: 98
access-control-allow-origin: *
set-cookie: XSRF-TOKEN=eyJpdiI6IkgvdngxWVlsRUpiaXN2WFJnQTZyU1E9PSIsInZhbHVlIjoiMlVhQThFZmxCLzFnbVBrYWVubjBqS29TUENydkNibFp1bS9CajJtQkZvdk8vWHoxNFV2YmlZbVZIeU5JRDltWHU1ekRFamtrc2Mya0JGdGtuZlFkTFlHcmIwUTFCWURBOTlJZ1dHcld0VjlJcVp3cW1LZ2Z4RU8vTVRLS0FRVWMiLCJtYWMiOiI1ZTAwZTEwODkzMjRiNzFmNWU5YzJmZDVlNTgzZmI5Y2VkZDY4ZDMyOGU2MjNmZjEwZmJkZjMzZmI0NTI1YjgwIiwidGFnIjoiIn0%3D; expires=Thu, 28-Apr-2022 16:05:43 GMT; path=/; samesite=lax
set-cookie: cat_facts_session=eyJpdiI6ImF4c0p2aWM4TkJnUXdnK2psRUlyVWc9PSIsInZhbHVlIjoiR0Rkc3NjTU45cnpUc09pdHNEWk5kOW9BRFNTWVVUWDI2bE9yWWM2TDh2OW5LS0Uwbks2cXluR2R0QnEyYmMreWJYMlorb09WNjVEMkZSRGF5NGhYUDF6WjBac2lwYTJ3dVhlV3o1bXpsMGNoZ1hvcXo0YTNRczNqM1pheVR6ZHAiLCJtYWMiOiJiYTc1ZjA3NWEwY2IzYzkyZmIxNzdiOGY1YjQxZGNlMzU2ZjdiMjYwYzhkZjI0ZWJhNjkxZTY4NWExMDg3NzgxIiwidGFnIjoiIn0%3D; expires=Thu, 28-Apr-2022 16:05:43 GMT; path=/; httponly; samesite=lax
x-frame-options: SAMEORIGIN
x-xss-protection: 1; mode=block
x-content-type-options: nosniff

{"fact":"There are more than 500 million domestic cats in the world, with approximately 40 recognized breeds.","length":100}
```

Note, the `header` is text and `body` is JSON.

### HTTP - JSON

Set the `output_type` parameter to `json` if you want to convert the full HTTP response into JSON format.

%copy%

{{< highlight yaml "hl_lines=9-10" >}}
version: 0.3.0
name: cat-facts
type: http-source
topic: cat-facts
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10s
  output_parts: full
  output_type: json
{{</ highlight >}}

Create a new connector that writes to `cat-facts`:

%copy first-line%
```bash
$ http % fluvio consume cat-facts -T=1 -d | jq
{
  "status": {
    "version": "HTTP/1.1",
    "code": 200,
    "string": "OK"
  },
  "header": {
    "server": "nginx",
    "date": "Thu, 28 Apr 2022 15:53:48 GMT",
    "vary": "Accept-Encoding",
    "cache-control": "no-cache, private",
    "x-ratelimit-limit": "100",
    "content-type": "application/json",
    "x-ratelimit-remaining": "97",
    "connection": "keep-alive",
    "set-cookie": [
      "XSRF-TOKEN=eyJpdiI6IlFnMHN0UzFpMlBEaDhjdjBaRzNoN1E9PSIsInZhbHVlIjoibElGczZPaUFBdDc4eDhiT3N5bkJxZkRVOFpVTjdJaXI0VFFGS1dkU1ZodDV5SGVHOEF6bkhVWjFjTEduMWpuRU50WkV2WURML0JyWWdqWnBtWFpyUXJRU1BBU2Q1cTA1YmdMaWp4TGViRjJnVzNaMFFFdDBaVDhPOHVlRTQxTXUiLCJtYWMiOiJmMGJmY2Q1YmJmYjAwNTQ4NzM0ZmU0MzUxMGEzYzNiMDQ4NGIyOTE1ZjJiY2NkZjFlOWQzZGZmMDBiMWMwMmUwIiwidGFnIjoiIn0%3D; expires=Thu, 28-Apr-2022 17:53:48 GMT; path=/; samesite=lax",
      "cat_facts_session=eyJpdiI6IklxdW1obmdqSFFSZ2ZRTmZGRnFNbXc9PSIsInZhbHVlIjoiSHJEQUJvZVVQbmJDNm9WaG5rbzNtYzBKZVN6Q0UwSGpXQTBRRHNxWGtVeVQ4bGxjU0lSZlhxa3g4SW0yS0k3bXpqNTIrK3R5K1Z3Vms5ck80dkdQWnlJWHhqWGpIY1R3Z25MWTBqTHAwZFlpQURGZUNFb2xRdmMwajVMaXV1TWkiLCJtYWMiOiJiN2Q3Yjk5MWE3Y2Q4ZWRkODk4MjM3YmM2NWJhZDI1MmY3ZmU0MDlkOWIyYzM0Zjg3YmE1NjdhNGMwZjliNjcyIiwidGFnIjoiIn0%3D; expires=Thu, 28-Apr-2022 17:53:48 GMT; path=/; httponly; samesite=lax"
    ],
    "x-xss-protection": "1; mode=block",
    "x-content-type-options": "nosniff",
    "transfer-encoding": "chunked",
    "access-control-allow-origin": "*",
    "x-frame-options": "SAMEORIGIN"
  },
  "body": "{\"fact\":\"Most cats had short hair until about 100 years ago, when it became fashionable to own cats and experiment with breeding.\",\"length\":120}"
}
```

-> **JSON Output** "body" is encoded in quotes (\") as JSON String within due to HTTP Response in this example containing application/json body itself.

To convert only the body of the HTTP Response and ignore the header, set `output_parts` to `body` and  the `output_type` keep as `json`:

```json
{
  "body": "{\"fact\":\"Most cats had short hair until about 100 years ago, when it became fashionable to own cats and experiment with breeding.\",\"length\":120}"
}
```


## Deploy Locally (Advanced)

In a local environment, you have the option to delop a **Managed Connector** or **Local Connector**.

* **Managed Connector**
  * ensure your [fluvio profile]({{<ref "/cli/client/profile" >}}) points to your local cluster.
  * follow the same instructions as in [Deploy on InfinyOn Cloud]({{<ref "#deploy-on-infinyon-cloud" >}}).
* **Local Connector** (for connector developers)
  * follow the instructions in the [next section]({{<ref "#deploy-local-connector" >}}).

#### Deploy Local connector

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

[Fluvio CLI]: {{<ref "/download" >}}
[free InfinyOn Cloud account]: https://infinyon.cloud/signup

## Changelog

| Version | Date       | PR                                                               | Subject                                           |
|:-------:|:----------:|:----------------------------------------------------------------:| ------------------------------------------------- |
| 0.3.0   | 2022-02-01 | [278](https://github.com/infinyon/fluvio-connectors/pull/278)    | <ul><li>Add interval times in human readable times</li><li>Support for producer and consumer top level config options</li><ul> |
| 0.2.0   | 2022-02-01 | [141](https://github.com/infinyon/fluvio-connectors/pull/141)    | <ul><li>Feature json Response Type Record</li><li>Deprecate metadata output_format in favor of `output_parts` [ body (default)</li><li>Add Metadata `output_type` [ text (default) | json ] | full ]</li><ul> |
| 0.1.1   | 2022-01-31 | [127](https://github.com/infinyon/fluvio-connectors/pull/127)    | <ul><li>Feature full Response Parts Record</li><li>Add Metadata `output_format` [ body (default) | full ]</li><ul> |
| 0.1.0   | 2021-11-09 | -   | <ul><li>Initial version with text/body Response (default) Record</li></ul> |
