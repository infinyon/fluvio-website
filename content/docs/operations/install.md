---
title: Installing Fluvio on Kubernetes
weight: 10
---

## Install

The kubernetes installation process requires two steps. Both are performed with the CLI.

First we need to install the "system" chart with sets up the [CRDs](https://kubernetes.io/docs/concepts/extend-kubernetes/api-extension/custom-resources/).

```
fluvio cluster start --sys
```

If installing in K8s environments other than minikube, you need to specify `--cloud <cloud>` option. Currently the only value supported is `aws`.

See other options by running 

```
fluvio cluster start -h
```

## Uninstall

Uninstallation must be performed using the following CLI commands

```
fluvio cluster delete --sys
fluvio cluster delete
```