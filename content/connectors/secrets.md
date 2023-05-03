---
title: Secrets
weight: 30
---

Connectors support secrets. Secrets are used to store sensitive information such as passwords, API keys, etc. The secrets can be referenced in the connector configuration file.

In order to use secrets, first, they need to be defined in the metadata section of the connector configuration file. The secrets are defined as a list of names. The names are used to reference the secret in the connector configuration file.

{{<code file="embeds/connectors/http-source-with-secrets.yaml" lang="yaml" copy="true">}}

In that config, it is defined a `MY_TOKEN` secret and it is used in the `headers` configuration of the http-source connector.

# Setting secrets

In order to use secrets, they have to be set. The way we set secrets depend on the platform used to run the connector.

## Setting secrets with CDK

If running connectors with `CDK`, you can use the `--secrets` flag to pass a file with the secrets definition. See the [CDK documentation]({{<ref "connectors/cdk/start-shutdown.md">}}) for more information.

## Setting secrets on Cloud

For cloud, we need to use `fluvio cloud secret` command. See the [Cloud documentation]({{<ref "cli/cloud/secret.md">}}) for more information.