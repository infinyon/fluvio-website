# InfinyOn Graphite Sink Connector
The [Graphite][4] Sink connector reads records from Fluvio topic and sends them to
the configured Graphite's Metric using the PlainText approach.

# Configuration

This connectors establishes a TCP Stream against the specified host on Graphite,
records are sent as UTF-8 encoded strings following [Graphite's PlainText][5] format.

The following example connector configuration can be used to send records to
the Graphite's Metric `weather.temperature.ca.sandiego`, the Graphite's TCP
server address is specified on the `addr` field.

```yaml
# sample-config.yaml
apiVersion: 0.1.0
meta:
  version: 0.1.1
  name: my-graphite-connector-test-connector
  type: graphite-sink
  topic: test-graphite-connector-topic
graphite:
  # https://graphite.readthedocs.io/en/latest/feeding-carbon.html#step-1-plan-a-naming-hierarchy
  metric-path: "weather.temperature.ca.sandiego"
  addr: "localhost:2003"
```

## Configuration Fields

| Model           | Data Type | Description                         |
|:----------------|:----------|:------------------------------------|
| `metric-path`   | `String`  | Graphite Metric to send records to  |
| `addr`          | `String`  | Graphite TCP Adddress to stream out |

## Usage

In this section we are going to setup a Graphite instance on Docker and then
we will use the CDK to spin up the Graphite Sink Connector to send metrics from
Fluvio Records to the Graphite instance.

Setup a Graphite instance for local development, you can follow the recipe used
to contribute to the official InfinyOn connector [here][3].

Reduce the Graphite's data retention interval by updating `storage-schemas.conf`
file for [Carbon][6]:

```conf
[all]
pattern = .*
retentions = 10s:12h
```

Make sure the Connector Development Key is setup in your system by issuing
the following command in your terminal.

```bash
fluvio install cdk
```

> If you dont have the Fluvio CLI installed already visit the [CLI][2] section

Create a YAML file with the name `weather-monitor-config.yaml` and specify connector settings:

```yaml
apiVersion: 0.1.0
meta:
  version: 0.1.1
  name: weather-monitor-sandiego
  type: graphite-sink
  topic: weather-ca-sandiego
graphite:
  # https://graphite.readthedocs.io/en/latest/feeding-carbon.html#step-1-plan-a-naming-hierarchy
  metric-path: "weather.temperature.ca.sandiego"
  addr: "localhost:2003"
```

> Make sure your Graphite instance is running on `localhost:2003`, use the
> `cdk log` subcommand to read logs from the connector instance.

Then produce records as usual:

```bash
echo 120 | fluvio produce weather-ca-sandiego
```

You can check on Graphite's API for your data points:

```bash
curl -o ./data.json http://localhost:12345/render\?target\=weather.temperature.ca.sandiego\&format\=json\&noNullPoints
```

[1]: https://infinyon.cloud/login
[2]: https://www.fluvio.io/cli/
[3]: https://github.com/infinyon/graphite-sink-connector/blob/main/CONTRIBUTING.md
[4]: https://graphiteapp.org/
[5]: https://graphite.readthedocs.io/en/latest/feeding-carbon.html#the-plaintext-protocol
[6]: https://graphite.readthedocs.io/en/latest/config-carbon.html#storage-schemas-conf
