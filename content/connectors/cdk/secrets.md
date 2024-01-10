---
title: Secrets
weight: 70
---

Connectors often connect to external entities such as `databases`, `message brokers`, or `APIs` that require a confidential authentication key.

Connectors offers this facility through [secrets]({{<ref "/connectors/secrets">}}).

### Use Secrets

Deploy connectors with a `--secrets` flag to pass a file with the secrets definitions:

%copy first-line%
```bash
$ cdk deploy start --config sample-config.yaml --secrets secrets.txt
```

In the secrets file, you should define a secret per line in the format `SECRET_NAME=SECRET_VALUE`:

%copy%
```bash
SECRET_NAME=SECRET_VALUE
SECRET_NAME_2=SUPER_SECRET_VALUE
```

Code to indicate that a connector config parameter can contain a secret should use the `SecretString` type. This allows the parameter to receive secrets which are not printable to logs.
```rust
use fluvio_connector_common::{connector, secret::SecretString};

#[derive(Debug)]
#[connector(config, name = "myconnector")]
pub(crate) struct MyconnectorConfig {
    /// A parameter receiving a secret string
    pub a_param: SecretString,

    ...
}
```

This allows a config file to provision secrets to the connector.
```yaml
# config-example.yaml
apiVersion: 0.1.0
meta:
  version: 0.3.0
  name: instancename
  type: my-connector
  topic: atopicname
  create-topic: true
  secrets:
    - name: SECRET_NAME
myconnector:
  a_param: "${{ secrets.SECRET_NAME }}_${{ secrets.SECRET_NAME_2 }}"

```

More extensive examples of secrets in connectors can be seen in use with the [Http Source]({{< ref "../inbound/http" >}}) connector and its repo https://github.com/infinyon/http-source-connector.

In the [next section]({{< ref "publish" >}}) we'll publish our connector to the Hub.

### Steps

1. [Generate a Connector]({{< ref "generate" >}})
2. [Build and Test]({{< ref "build-test" >}})
3. [Start and Shutdown]({{< ref "start-shutdown" >}})
4. [Troubleshooting]({{< ref "troubleshooting" >}})
5. **[Secrets]({{< ref "secrets" >}})**
6. [Publish to Connector Hub]({{< ref "publish" >}})
7. [Use Examples in Github]({{< ref "github-examples" >}})