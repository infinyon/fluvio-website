---
title: "This Week in Fluvio #52"
date: 2023-04-19
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

Welcome to the 52nd edition of this week in Fluvio.This one is going to be short and sweet.

## New release
https://github.com/infinyon/fluvio/releases/tag/v0.10.7

## New features
We have released connector secrets and here are the details:

# Summary
Specify user interfaces to cloud connector secrets. This includes a cli for populating the secrets, as well as how
a connector configuration can refer to the secrets.

# Motivation
Customers need to be able to provide secrets to connectors
in a protected way in order to access their own or third-party
services with the confidence that Infynyon infrastructure 
provides protection from loss of the secrets.

# Approach
Cloud secrets are set via cli.  Each secret is a named value with all secrets sharing a secrets namespace per account. Connector configuration files can refer to secrets by name, and the cloud connector infrastructure will provision the connector with the named secrets.

Due to security concerns, listing actual secret values or downloading them after they have been set is not allowed. However, a listing of secret names as well as what date they were last set is described in the interface.

## New `fluvio cloud secret` subcommands

The secrets cli is an added subcommand to 'fluvio cloud' with the following cli ui interface:

```sh
fluvio cloud secret set --connector <NAME> <VALUE>
fluvio cloud secret set --connector <NAME> --file <FILENAME> # e.g. a tls cert file
fluvio cloud secret delete <NAME>
fluvio cloud secret list
```

Note: The current implementation limits the scope of the secrets to connectors only. Also see open questions below regarding set, update, create.


## Displaying secrets: `fluvio cloud secret list`

One security principle in effect with secret list is that we never want to send custoemr secrets out of the cloud. They are only decrypted at the point of use inside the connector. But users still need to see what named secrets have been set, and potentially when they were last updated.

```sh
$ fluvio cloud secret list
SecretNames          LastUpdate
CAT_FACTS_CLIENT_ID  12-10-2022 1:07pm
CAT_FACTS_SECRET     01-02-2023 12:01am
```

## Connector config file references

The connector config files can reference cloud secrets by NAME as follows:

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


## Upcoming features
We are working on a few interesting problems around deduplication and stream processing. We have also made solid progress on a public roadmap to share with the community.

We would love to know what do you recommend we build next to make your development flow easier. Please comment in [our Discord channel] and let us know. Tag [Deb (DRC)](https://discordapp.com/users/887863207232954418) on your feedback and feature requests.


## New blog post
[Why 87% of all data projects are doomed to fail, and how you can improve the odds of success](https://www.infinyon.com/blog/2023/03/failing-data-projects/)


## Open positions
* [Sr. Rust Cloud Software Engineer](https://infinyon.com/careers/cloud-engineer-senior-level/)
* [Developer Advocate](https://infinyon.com/careers/developer-advocate-mid-senior-level/)

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions