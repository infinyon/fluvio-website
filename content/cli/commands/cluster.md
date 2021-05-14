---
title: Fluvio Cluster
menu: Cluster
weight: 50
---

The `fluvio cluster` family of commands is used to install and manage your own
Fluvio cluster. The two primary forms of self-hosted cluster are Kubernetes
(via Minikube) and Local clusters, both of which can be automatically set up
by cluster commands. Please make sure you have followed the
[getting started guide] and installed all the necessary dependencies before
attempting to install your own cluster.

[getting started guide]: /docs/get-started

## `fluvio cluster check`

This command is used to check whether you have all the required dependencies
set up on your system and whether they are running correctly.
If this runs and returns successfully, you should be all set to start
a local or minikube cluster.

```
fluvio-cluster-check
Check that all requirements for cluster startup are met

USAGE:
    fluvio cluster check

FLAGS:
    -h, --help    Prints help information
```

Example usage:

```bash
$ fluvio cluster check
Running pre-startup checks...
✅ ok: Kubernetes config is loadable
✅ ok: Supported kubernetes version is installed
✅ ok: Supported helm version is installed
✅ ok: Fluvio system charts are installed
✅ ok: Can create service
✅ ok: Can create customresourcedefinitions
✅ ok: Can create secret
✅ ok: Load balancer is up
All checks passed!
You may proceed with cluster startup
next: run `fluvio cluster start`
```

## `fluvio cluster start`

This command is used to start your own Fluvio cluster, with all the
machinery needed to receive, process, and serve streaming messages.

There are two main variations of this command. The default variation
is invoked simply by `fluvio cluster start`. This will install Fluvio
to a Kubernetes cluster, typically Minikube. The other variation is
`fluvio cluster start --local`, which will start the cluster components
as plain processes on your local machine.

```
fluvio-cluster-start
Start a Fluvio cluster, locally or on Minikube

USAGE:
    fluvio cluster start [FLAGS] [OPTIONS]

FLAGS:
        --develop                  use local image
        --skip-profile-creation
        --sys                      installing sys
        --local                    install local spu/sc(custom)
        --tls                      Whether to use TLS
        --skip-checks
            Whether to skip pre-install checks, defaults to false

        --setup
            Tries to setup necessary environment for cluster startup

    -h, --help                     Prints help information

OPTIONS:
        --chart-version <chart-version>
            k8: use specific chart version [default: 0.6.0-beta.1]

        --image-version <image-version>
            k8: use specific image version

        --registry <registry>
            k8: use custom docker registry

        --namespace <namespace>
            k8 [default: default]

        --group-name <group-name>
            k8 [default: main]

        --install-name <install-name>
            helm chart installation name [default: fluvio]

        --chart-location <chart-location>
            Local path to a helm chart to install

        --cloud <cloud>
            k8 [default: minikube]

        --spu <spu>
            number of SPU [default: 1]

        --rust-log <rust-log>
            RUST_LOG options

        --log-dir <log-dir>
            log dir [default: /usr/local/var/log/fluvio]
            
        --domain <domain>                                        TLS: domain
        --ca-cert <ca-cert>                                      TLS: ca cert
        --client-cert <client-cert>
            TLS: client cert

        --client-key <client-key>                                TLS: client key
        --server-cert <server-cert>
            TLS: path to server certificate

        --server-key <server-key>
            TLS: path to server private key

        --authorization-config-map <authorization-config-map>
```

Example usage:

To start a cluster on Minikube:

```bash
$ fluvio cluster start
"fluvio" has been added to your repositories
Hang tight while we grab the latest from your chart repositories...
...Successfully got an update from the "jaegertracing" chart repository
...Successfully got an update from the "prometheus-community" chart repository
...Successfully got an update from the "fluvio" chart repository
Update Complete. ⎈Happy Helming!⎈
NAME: fluvio
LAST DEPLOYED: Mon Dec 28 13:12:27 2020
NAMESPACE: default
STATUS: deployed
REVISION: 1
TEST SUITE: None
✅ ok: Kubernetes config is loadable
✅ ok: Supported helm version is installed
✅ ok: Fluvio system charts are installed
✅ ok: Previous fluvio installation not found
✅ ok: Load balancer is up
```

To start a cluster locally (as processes on your machine):

```bash
$ fluvio cluster start --local
Performing pre-flight checks
✅ ok: Supported helm version is installed
✅ ok: Supported kubernetes version is installed
✅ ok: Kubernetes config is loadable
✅ ok: Fluvio system charts are installed
```

## `fluvio cluster delete`

Deletes a Fluvio cluster and all data associated with it. Be careful, this
cannot be undone.

```
fluvio-cluster-delete
Delete a Fluvio cluster from the local machine or Minikube

USAGE:
    fluvio cluster delete [FLAGS] [OPTIONS]

FLAGS:
        --no-wait    don't wait for clean up
        --local      Remove local spu/sc(custom) fluvio installation
        --sys        Remove fluvio system chart
    -h, --help       Prints help information

OPTIONS:
        --namespace <namespace>     [default: default]
        --name <name>               [default: fluvio]
```

Example usage:

To uninstall Fluvio from Kubernetes (e.g. Minikube):

```bash
$ fluvio cluster delete
```

To uninstall Fluvio from your local machine:

```bash
$ fluvio cluster delete --local
```

## `fluvio cluster releases list`

Prints all the published versions of Fluvio that are candidates to
install on Kubernetes with `fluvio cluster start`.

The Fluvio cluster components are distributed as Helm charts which allow
them to be installed easily on Kubernetes. This prints all the releases
that are available.

```
fluvio-cluster-releases-list
Show a list of Fluvio release versions

USAGE:
    fluvio cluster releases list

FLAGS:
    -h, --help    Prints help information
```

Example usage:

```bash
$ fluvio cluster releases list
VERSION
0.6.0-latest
0.6.0-beta.1-latest
0.6.0-beta.1
0.6.0-alpha.8-latest
0.6.0-alpha.7-latest
...
```

## `fluvio cluster spu list`

This command shows details about the active SPUs in your cluster.
It is mostly useful for checking on the status of individual SPUs
to see whether they are still online, and which addresses they live at.

```
fluvio-cluster-spu-list
List all SPUs known by this cluster (managed AND custom)

fluvio cluster spu list [FLAGS] [OPTIONS]

FLAGS:
        --custom    Whether to list only custom SPUs
    -h, --help      Prints help information

OPTIONS:
    -O, --output <type>    Output [default: table]  [possible values: table,
                           yaml, json]
```

Example usage:

```bash
$ fluvio cluster spu list
 ID    NAME             STATUS  TYPE      RACK  PUBLIC          PRIVATE
 5001  custom-spu-5001  Online  "custom"   -    localhost:9010  localhost:9011
```
