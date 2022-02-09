---
title: "This Week in Fluvio #22"
date: 2022-02-09
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}


## New Release - Fluvio v0.9.19

### Connector versions
Now when you run `fluvio connector list`, the version of the connector is returned

%copy first-line%
```bash
$ fluvio connector list

```

### WASI support

SmartModule devs can now compile Fluvio with WASI support. This provides SmartModules access to `stdout` and `stderr` for debugging purposes.

Compilation commands
```
```

Example Smart Module
```
```

Build smart module
```
```

Load smart module
```
```

Example producer input
```
```

Use smart module
```
```

## Connectors

### HTTP

Our http connector has new options available to format its output.

We provide HTTP response information in a JSON format.

config
```yaml
parameters:
  output_type: json
  output_parts: body 
```

This output returns the body content as a single JSON string.

```json
{"body":"{\"fact\":\"The largest breed of cat is the Ragdoll with males weighing in at 1 5 to 20 lbs. The heaviest domestic cat on record was a neutered male tabby named Himmy from Queensland, Australia who weighed 46 lbs. 1 5 oz.\",\"length\":209}"}
```

If you need all of the header information, you can request the full response 

config
```yaml
parameters:
  output_type: json
  output_parts: full
```

```json
{"version":"HTTP/1.1","status_code":200,"status_string":"OK","headers":{"server":"nginx","transfer-encoding":"chunked","date":"Sat, 05 Feb 2022 12:15:01 GMT","x-content-type-options":"nosniff","access-control-allow-origin":"*","set-cookie":["XSRF-TOKEN=eyJpdiI6IlB5T1FVNXNDR3NvMlgrQzQ3TEJ4dGc9PSIsInZhbHVlIjoiKzRuckJrTU16SG9ycFk4L0w1QjYvdWQ1MDdJZGZZNURZSW9jQkN4RnlmMEcyMUo0TWVCSjE2SFJJblVrVWtTM05QOTE1VEdpOWlaTFlWMlFISEhSS0FRWThJMGNOaWpLOGFWTXVMRklWTzZ3dDJvSUxQTW5qeDdVNTYvV1M4ek8iLCJtYWMiOiJkOWE4NDQwZTIyOTdlZDAyMmU4OGQ5YTBkOWMzNjY1YmY0NDU5Y2RhOTZlNDg0NGI1YTdjMTE0ZTRkM2U2ZGJkIiwidGFnIjoiIn0%3D; expires=Sat, 05-Feb-2022 14:15:01 GMT; path=/; samesite=lax","cat_facts_session=eyJpdiI6IkNMWWxZLzFFL21wODdmanAyWjk0cFE9PSIsInZhbHVlIjoiNG1OTEZlKzhxVW9YaXBsbzl4TCtrVjJvQjBjWmdUYkg0VWtoVEgycVhGMWduUThNekNBQ0hxVFpCaG5PdHc3NnZsbWVhUHI2NU5Yem9mU1pxbk1CcDFLYUNIbkw1M2EzN1IrNTFlbDFkbG9YMjUyRUkrQnJ3OGR6NEdscVZpRnAiLCJtYWMiOiJlMWJlNTZmYzU1MmY2M2U1YjliMjEyMjFmZTcwM2FiOWI0MzEwM2M5YmM4ZDFiZjhkZTg5NDNmNGMwMmUzOTg5IiwidGFnIjoiIn0%3D; expires=Sat, 05-Feb-2022 14:15:01 GMT; path=/; httponly; samesite=lax"],"connection":"keep-alive","vary":"Accept-Encoding","cache-control":"no-cache, private","content-type":"application/json","x-ratelimit-limit":"100","x-xss-protection":"1; mode=block","x-frame-options":"SAMEORIGIN","x-ratelimit-remaining":"98"},"body":"{\"fact\":\"The first official cat show in the UK was organised at Crystal Palace in 1871.\",\"length\":78}"}
```

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions