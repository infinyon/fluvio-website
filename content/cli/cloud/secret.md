---
title: Cloud Secrets
menu: Secret
weight: 40
---

Fluvio cloud secrets are set via the cli.  Each secret is a named value with all secrets sharing a namespace per account. Connector configuration files can refer to secrets by name, and the cloud connector infrastructure will provision the connector with the named secrets.

Due to security concerns, listing actual secret values or downloading them after they have been set is not allowed. However, a listing of secret names as well as what date they were last set is accessible.

## `fluvio cloud secret` subcommands

The secrets cli is an added subcommand fluvio cloud as 'fluvio cloud secret'.

Actions possible with a fluvio cloud secret are:
- set
- delete
- list

```sh
fluvio cloud secret set <NAME> <VALUE>
fluvio cloud secret list
fluvio cloud secret delete <NAME>
```

## `fluvio cloud secret set`

Setting a scret of `<NAME>` will allow it to be refrenced by that name in connector configuration parameters that can use secret references.

```
fluvio cloud secret set <NAME> <VALUE>
```
All secrets are in a shared connector namespace, but a specific connector is only given access to secrects named in the configuration file of the connector.

## `fluvio cloud secret list`

`fluvio cloud secret list` will list only the secret names and their last update time. Once a secret has been set into fluvio cloud, it is stored so only referencing connectors may access the secret. There is no way to retreive the secret value from fluvio cloud.

```sh
$ fluvio cloud secret list
SecretNames          LastUpdate
CAT_FACTS_CLIENT_ID  12-10-2022 1:07pm
CAT_FACTS_SECRET     01-02-2023 12:01am
```


## `fluvio cloud secret delete`

This will delete the named secret.

```
fluvio cloud secret delete <NAME>
```


## Connector config file references

The connector config files can reference cloud secrets by NAME generically as follows:

```yaml
meta:
  version: 0.1.0
  name: my-connector
  type: package-name
  topic: a-topic
  create-topic: true
<CUSTOM>:  # named section for custom config parameters, usually a short name like "http", or "mqtt"
  param_client_id: 
    secret:
      name: CAT_FACTS_CLIENT_ID
  param_client_secret:
    secret: 
      name: CAT_FACTS_SECRET
```

## Example

An example of a connector that can use secret parameters, the http connector might be setup and configured as follows.

```
# setup a secret
$ fluvio cloud secret set AUTH_HEADER "Authorization: bearer 1234abcd"

# write a connector config http-config-with-secret.yaml
$ cat << END_CONFIG > http-config-with-secret.yaml
meta:
  version: 0.1.1
  name: cat-facts
  type: http-source
  topic: cat-facts-secret
  create-topic: true
http:
  endpoint: "https://catfact.ninja/fact"
  interval: 10s  
  headers:
    - secret:
        name: AUTH_HEADER
END_CONFIG

# run the connector
$ fluvio cloud connector --config http-config-with-secret.yaml
```

This same configuration file is compatible with both fluvio cloud connectors and cdk locally run connectors. The cloud connectors are provisioned via the `fluvio cloud secret ...` set of commands, while the cdk secrets are provided locally.


