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

**Todo** Provide sample code on how to implement secrets in a custom connector.

In the [next section]({{< ref "publish" >}}) we'll publish our connector to the Hub.

## Steps

1. [Generate a Connector]({{< ref "generate" >}})
2. [Build and Test]({{< ref "build-test" >}})
3. [Start and Shutdown]({{< ref "start-shutdown" >}})
4. [Troubleshooting]({{< ref "troubleshooting" >}})
5. **[Secrets]({{< ref "secrets" >}})**
6. [Publish to Connector Hub]({{< ref "publish" >}})
7. [Start from Connector Hello]({{< ref "github-examples" >}})