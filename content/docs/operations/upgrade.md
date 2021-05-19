---
title: Upgrade
weight: 50
---

You can upgrade your Fluvio cluster using the CLI. Underneath this uses the <a href="https://helm.sh/docs/helm/helm_upgrade/" target="_blank">upgrade function in Helm</a>, therefore similar behavior can be expected.

Note that upgrading is not reversable as downgrading is not supported.

## Upgrade Command

Run `fluvio cluster upgrade` to upgrade fluvio the latest version supported by the CLI. The upgrade command supports options similar to the `start` command.

Optionally you can specify the target version with `--chart-version <chart-version>`. You can view the list of releases supported by the CLI by running `fluvio cluster releases list`

See `fluvio cluster upgrade -h` to see all options.
