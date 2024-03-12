---
title: Install Fluvio on Kubernetes
menu: Install
weight: 10
---

Fluvio is a Kubernetes-native containerized application.  It uses Helm internally to manage its installation.
Fluvio CLI is a tool to manage Fluvio's installation.

If you only want to install a single Instance of Fluvio, Fluvio will automatically install all necessary dependencies and run the Fluvio service.

You can install multiple Fluvio instances on the same Kubernetes cluster by using different namespaces.  In order to do so, you need to specify the namespace when installing Fluvio otherwise Fluvio will install in the default namespace.

However, if you want to install multiple instances of Fluvio, you need to install the helm chart manually.  There are two charts. First is a cluster side chart (`sys` chart) which is common to all Fluvio instances. Second is a `app` chart which can be configured for each instance.

## Managing a single instance of Fluvio

This command will install Fluvio and it's dependencies in the default namespace.

%copy first-line%
```shell
$ fluvio cluster start --k8
```

This command will de-install Fluvio and it's dependencies (including all data) in the default namespace.

%copy first-line%
```shell
$ fluvio cluster delete --k8
```

This however, will not uninstall sys chart.  You can use the following command to uninstall sys chart.
```shell
$ fluvio cluster delete --k8 --sys
```
For installing on a remote Kubernetes cluster where the machine running the CLI is not the local host, you may want to use the use the `--proxy-addr <DNS or IP>` option. To specify how to connect to the kubernetes cluster.

## Install Multiple Instances of Fluvio

For this scenario, you need to install the charts manually.

First, install the `sys` chart.  This only has to be done once. Helm needs access
to a working `kubectl` context.

%copy first-line%
```bash
helm upgrade --install fluvio-sys ./k8-util/helm/fluvio-sys
```

Then install each instance of Fluvio one by one on a different namespace,
setting the image tag to the fluvio release version, and spacing
the scPod ports apart.

Depending on the implementation of the kubernetes cluster being used,
the `fluvio profile add NAME  HOST:PORT`, the host might be a dns name,
local host, or an IP.  The example below assumes a local host access to
the nodeport, and a copy of the [fluvio repository](https://github.com/infinyon/fluvio).

Other networking configurations besides nodePort configuration are beyond
the scope of this guide and require modification of the helm chart values.

First instance:

%copy%
```shell
kubectl create namespace first
helm install fluvio-app k8-util/helm/fluvio-app  --values ./k8-util/helm/fluvio-app/values.yaml \
  --namespace third \
  --set "image.tag=0.11.5" \
  --set "scPod.nodePort=30003"

fluvio profile add k1 127.0.0.1:30003
fluvio cluster create spg default
```

Second instance:
%copy%
```shell
kubectl create namespace second
helm install fluvio-app k8-util/helm/fluvio-app  --values ./k8-util/helm/fluvio-app/values.yaml \
  --namespace third \
  --set "image.tag=0.11.5" \
  --set "scPod.nodePort=30103"

fluvio profile add k2 127.0.0.1:30103
fluvio cluster create spg default
```

and so forth.


To delete a Fluvio instances, supply namespace as an argument.

%copy first-line%
```shell
$ fluvio cluster delete --k8 --namespace first
```

You can only a delete `sys` chart when you have deleted all the Fluvio instances.

### Options

The CLI takes a `--chart-values` option which accepts a file path to a YAML file with values that are applied to the Fluvio [Helm chart].

[Helm chart]: {{< ref "./helm" >}}

See other options by running

%copy first-line%
```bash
$ fluvio cluster start -h
```

