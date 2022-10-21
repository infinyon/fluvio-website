---
title: HTTP Connector
menu: HTTP
connector:
  name: "infinyon/fluvio-connect-http"
  link: "https://github.com/infinyon/fluvio-connectors/tree/main/rust-connectors/sources/http"
---

Fluvio's `http` connector allows you to periodically fetch data from an HTTP endpoint,
feeding the HTTP response into a Fluvio topic.

This is useful for monitoring APIs
continuously, and building streaming applications that react to new or updated info.


Note that this connector is _not_ intended for streaming HTTP endpoints, it instead
periodically sends HTTP requests and collects the response as an event.


## Configuration Options

The inbound HTTP connector supports the following configuration options:

### `endpoint`
*required*

The HTTP endpoint to send requests to

### `method`
default: `GET`

The HTTP verb to use - i.e. `GET`, `PUT`, `POST`, `DELETE`

### `body`
*optional*

The body to use in the HTTP request to the endpoint

### `interval`
default: `300`

The period (in seconds) between sending requests to the endpoint

### `output_parts`
default: `body`

HTTP Response output Record parts

Choices:
* `body`
* `full`

### `output_type`
default: `text`

HTTP Response output Record type

Choices:
* `text`
* `json`

#### Example connector config 

%copy%

{{<code file="code-blocks/yaml/connectors/inbound-examples/inbound-http.yaml" lang="yaml" copy=true >}}

This example config for inbound HTTP connector config reads from the {{<link "https://catfact.ninja/fact" "https://catfact.ninja" >}} website, and stores the output in the `cat-facts` topic every `300` seconds

## Data Events

The HTTP connector generates one data event per HTTP response. The data depends on the endpoint you are accessing and can be formatted in multiple ways, as described below.

### HTTP - Body

By default, the HTTP connector produces the `body` of the HTTP response in JSON format:

Example output: `output_parts: body`

%copy first-line%
```bash
$ fluvio consume cat-facts -T -d
{"fact":"A cat almost never meows at another cat, mostly just humans. Cats typically will spit, purr, and hiss at other cats.","length":116}
{"fact":"In one stride, a cheetah can cover 23 to 26 feet (7 to 8 meters).","length":65}
{"fact":"Phoenician cargo ships are thought to have brought the first domesticated cats to Europe in about 900 BC.","length":105}
```


### HTTP - Header/Body

Use the `output_parts` parameter to produce full HTTP responses:

Example output: `output_parts: full`

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

{{<idea>}}
The `header` is text and `body` is JSON.
{{</idea>}}

### HTTP - JSON

Set the `output_type` parameter to `json` if you want to convert the full HTTP response into JSON format.

 The following is the example output from {{<link "https://catfact.ninja/fact" "https://catfact.ninja" >}} with the following inbound HTTP connector config:
* `output_parts: full`
* `output_type: json`

%copy first-line%
```bash
$ fluvio consume cat-facts -T=1 -d | jq
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


## Changelog

| Version | Date       | PR                                                               | Subject                                           |
|:-------:|:----------:|:----------------------------------------------------------------:| ------------------------------------------------- |
| 0.3.0   | 2022-02-01 | [278](https://github.com/infinyon/fluvio-connectors/pull/278)    | <ul><li>Add interval times in human readable times</li><li>Support for producer and consumer top level config options</li><ul> |
| 0.2.0   | 2022-02-01 | [141](https://github.com/infinyon/fluvio-connectors/pull/141)    | <ul><li>Feature json Response Type Record</li><li>Deprecate metadata output_format in favor of `output_parts` [ body (default)</li><li>Add Metadata `output_type` [ text (default) | json ] | full ]</li><ul> |
| 0.1.1   | 2022-01-31 | [127](https://github.com/infinyon/fluvio-connectors/pull/127)    | <ul><li>Feature full Response Parts Record</li><li>Add Metadata `output_format` [ body (default) | full ]</li><ul> |
| 0.1.0   | 2021-11-09 | -   | <ul><li>Initial version with text/body Response (default) Record</li></ul> |
