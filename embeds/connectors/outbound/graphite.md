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
  version: 0.1.2
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

This section will walk you through the process of setting up a Graphite
instance and using Fluvio to send metrics to this Graphite instance.

> This section assumes you have Docker and Fluvio installed in your system.

### Setting Up Graphite

We will run our Graphite instance on Docker using the `docker compose` command
for simplicity.

The Graphite container will setup [Carbon Configuration][6] files in your
working directory, we need to update these files to reduce Carbon's persisntance
intervals, making it more frecuent.

Create a copy of our [`docker-compose.yml`][7] file and execute the container:

```bash
docker compose up --build -d
```

This will generate a directory with the name `.graphite`, which contains
configuration files.

Replace the contents of `.graphite/conf/storage-schemas.conf` to record on an
interval of 10 seconds and persist the last 12 hours of data.

```conf
[all]
pattern = .*
retentions = 10s:12h
```

Now we need to re run the Graphite container so Carbon uses the new
configuration.

```bash
docker compose down
docker compose up --build -d
```

You can visit `http://localhost:12345` in your browser to access the Dashboard.

> Credentials for the Dashbord are User: `root` and Password: `root`

With the Graphite instance set, we can move into [Setting Up Fluvio with Graphite Sink Connector][8].

### Setting Up Fluvio with Graphite Sink Connector

In this section we are going use the CDK to spin up the Graphite Sink Connector
to send metrics from Fluvio Records to the Graphite instance.

Make sure the Connector Development Kit is setup in your system by issuing
the following command in your terminal.

```bash
fluvio install cdk
```

> If you dont have the Fluvio CLI installed already visit the [CLI][2] section

Create a YAML file with the name `weather-monitor-config.yaml` and specify connector settings:

```yaml
apiVersion: 0.1.0
meta:
  version: 0.1.2
  name: weather-monitor-sandiego
  type: graphite-sink
  topic: weather-ca-sandiego
graphite:
  # https://graphite.readthedocs.io/en/latest/feeding-carbon.html#step-1-plan-a-naming-hierarchy
  metric-path: "weather.temperature.ca.sandiego"
  addr: "localhost:2003"
```

Deploy the Connector using the CDK

```bash
cdk deploy start --config weather-monitor-config.yaml
```

> Make sure your Graphite instance is running on `localhost:2003`, use the
> `cdk log` subcommand to read logs from the connector instance.

Then produce records as usual:

```bash
echo 120 | fluvio produce weather-ca-sandiego
```

> Remember that Carbon's retention is set to `10s:12h`, this means that if will
> write metrics every 10s.

Use Graphite's REST API to check on the stored data.

```bash
curl -o ./data.json http://localhost:12345/render\?target\=weather.temperature.ca.sandiego\&format\=json\&noNullPoints
```

[1]: https://infinyon.cloud/login
[2]: https://www.fluvio.io/cli/
[3]: https://github.com/infinyon/graphite-sink-connector/blob/main/CONTRIBUTING.md
[4]: https://graphiteapp.org/
[5]: https://graphite.readthedocs.io/en/latest/feeding-carbon.html#the-plaintext-protocol
[6]: https://graphite.readthedocs.io/en/latest/config-carbon.html#storage-schemas-conf
[7]: https://github.com/infinyon/graphite-sink-connector/blob/main/docker-compose.yml
[8]: #setting-up-fluvio-with-graphite-sink-connector
