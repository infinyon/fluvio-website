---
title: ANALYTICS
weight: 100
_build:
  render: never
---

## Data Transformations

The HTTP connector supports the following [SmartModules]({{<ref "/docs/smartmodules/overview/" >}}) for data transforamtions:

- `filter`: to eliminate invalid records
- `map`: to correct or transform data formats
- `filtermap`: to apply both, filter and map.
- `arraymap`: to divide a composite object into individual records.

Once a SmartModule is uploaded on the cluster, it can be referenced in the `parameters` section. In this example the http connector applies `catfact-map`.

{{< highlight yaml "hl_lines=10" >}}
# connect.yml
version: 0.3.0
name: cat-facts
type: http-source
topic: cat-facts
direction: source
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 10s
  map: "catfact-map"
{{< /highlight >}}

For additional information checkout [Connector SmartModules]({{<ref "/connectors/#smartmodules" >}}).