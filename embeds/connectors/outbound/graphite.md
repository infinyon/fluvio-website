# graphite-sink
Graphite Metrics Server Fluvio Sink Connector

## Usage

This connectors establishes a TCP Stream against the specified host on Graphite,
records are sent as UTF-8 encoded strings following Graphite's PlainText format.

```yaml
# sample-config.yaml
apiVersion: 0.1.0
meta:
  version: 0.1.0
  name: my-graphite-connector-test-connector
  type: graphite-sink
  topic: test-graphite-connector-topic
graphite:
  # https://graphite.readthedocs.io/en/latest/feeding-carbon.html#step-1-plan-a-naming-hierarchy
  metric-path: "weather.temperature.ca.sandiego"
  addr: "localhost:2003"
```
