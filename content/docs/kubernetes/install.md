---
title: Install Fluvio on Kubernetes
menu: "Kubernetes Install"
weight: 10
---

Fluvio is a Kubernetes-native containerized application.  There are multiple ways to install Fluvio in a kubernetes cluster.
Fluvio CLI is a tool to manage Fluvio's installation, or the helm charts can be used directly.

Installing with the cli will install a simple default configuration for many types of kubernetes clusters.
If you only want to install a single instance of Fluvio, Fluvio will automatically install and run the Fluvio service.

However, if you want a particular networking configuration, or wish to install multiple instances of Fluvio, you should install the helm chart manually (modifying them for your needs).

If you run into any problems along the way, make sure to check out our [troubleshooting] page to find a fix.

[troubleshooting]: {{< ref "/docs/operations/troubleshoot.md" >}}

## Install with the CLI

This command will install Fluvio and it's dependencies in the default namespace. This method works with many but not all kubernetes cluster types and is an opinionated set of configurations for a simple, working fluvio cluster.

%copy first-line%
```shell
$ fluvio cluster start --k8
```

For installing on a remote Kubernetes cluster where the machine running the CLI is not the local host, consider using the `--proxy-addr <DNS or IP>` option which will access the fluvio app endpoints through the specified proxy address.

%copy first-line%
```shell
$ fluvio cluster start --k8 --proxy-addr <DNS or IP>
```

## Install a fluvio instance with Helm

If you want more precise control of the kubernetes installation, you need to install the charts manually.  Helm needs access
to a working `kubectl` context. The charts are available in the [fluvio repository](https://github.com/infinyon/fluvio) under [`k8-util/helm/`](https://github.com/infinyon/fluvio/tree/master/k8-util/helm).

There are two charts. First is the `fluvio-sys` chart which is common to all Fluvio instances. Second is a `fluvio-app` chart which can be configured for one or more instances of clusters

%copy first-line%
```bash
helm upgrade --install fluvio-sys ./k8-util/helm/fluvio-sys
```

%copy%
```shell
helm install fluvio-app k8-util/helm/fluvio-app  --values ./k8-util/helm/fluvio-app/values.yaml \
  --set "image.tag=0.11.5" \
  --set "scPod.nodePort=30003"

fluvio profile add k1 127.0.0.1:30003
fluvio cluster create spg default
```

## Install a Multi Cluster Instance with Helm

You can install multiple Fluvio instances on the same Kubernetes cluster by using different namespaces.  In order to do so, you need to specify the namespace when installing Fluvio otherwise Fluvio will install in the default namespace.  The charts are available in the [fluvio repository](https://github.com/infinyon/fluvio) under [`k8-util/helm/`](https://github.com/infinyon/fluvio/tree/master/k8-util/helm).

Other networking configurations besides nodePort configuration are beyond
the scope of this guide and require modification of the helm chart values. Feel free to ask about other configurations on our [Discord](https://discordapp.com/invite/bBG2dTz) or on the Fluvio repository [discussions(https://github.com/infinyon/fluvio/discussions).]

First, install the `fluvio-sys` chart.  This only has to be done once.

%copy first-line%
```bash
helm upgrade --install fluvio-sys ./k8-util/helm/fluvio-sys
```

Then install each instance of Fluvio one by one on a different namespace and spacing
the scPod ports apart.

Depending on the implementation of the kubernetes cluster being used,
the `fluvio profile add NAME  HOST:PORT`, the host might be a dns name,
local host, or an IP.  The example below assumes a local host access to
the nodeport, and a copy of the [fluvio repository](https://github.com/infinyon/fluvio).


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

You can only a delete `fluvio-sys` chart when you have deleted all the Fluvio instances.
