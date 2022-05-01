---
title: ANALYTICS
weight: 20
_build:
  render: never
---

## Data Transformations

The HTTP connector supports the following [SmartModules](/docs/smartmodules/overview/) for data transforamtions:

- `filter`: to eliminate invalid records
- `map`: to correct or transform data formats
- `filtermap`: to apply both, filter and map.
- `arraymap`: to divide a composite object into individual records.

Once a SmartModule is uploaded on the cluster, it can be referenced in the `parameters` section. In this example the http connector applies `catfact-map`.

{{< highlight yaml "hl_lines=10" >}}
# connect.yml
version: 0.2.0
name: cat-facts
type: http
topic: cat-facts
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10
  map: "catfact-map"
{{< /highlight >}}

For additional information checkout [Connector SmartModules](/connectors/#smartmodules).