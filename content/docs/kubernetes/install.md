---
title: Install Fluvio on Kubernetes
menu: Install
weight: 10
---

Fluvio is a Kubernetes-native containerized application.  It uses Helm internally to manage its installation.  
Fluvio CLI is a tool to manage Fluvio's installation.

You can install multiple Fluvio instances on the same Kubernetes cluster by using different namespaces.  In order to do so, you need to specify the namespace when installing Fluvio otherwise Fluvio will install in the default namespace.

If you only want to install a single Instance of Fluvio, Fluvio will automatically install all necessary dependencies and run the Fluvio service.

However, if you want to install multiple instances of Fluvio, you need to install the helm chart manually.  There are two charts. First is a cluster side chart (`sys` chart) which is common to all Fluvio instances. Second is a `app` chart which can be configured for each instance.

## Managing a single instance of Fluvio

This command will install Fluvio and it's dependencies in the default namespace.
```
$ fluvio cluster start 
```

This command will de-install Fluvio and it's dependencies (including all data) in the default namespace.

```
$ fluvio cluster delete
```

This however, will not uninstall sys chart.  You can use the following command to uninstall sys chart.
```
$ fluvio cluster delete --sys
```


## Install Multiple Instances of Fluvio

For this scenario, you need to install the charts manually. 

First, install the `sys` chart.  This only has to be done once.

%copy first-line%
```bash
fluvio cluster start --sys
```

Then install each instance of Fluvio one by one on a different namespace.  

First instance:

```
$ kubectl create namespace first
$ fluvio cluster start --namespace first
```

Second instance:
```
$ kubectl create namespace second
$ fluvio cluster start --namespace second
```

and so forth.


To delete a Fluvio instances, supply namespace as an argument.

```
$ fluvio cluster delete --namespace first
```

You can only a delete `sys` chart when you have deleted all the Fluvio instances.

### Options

The CLI takes a `--chart-values` option which accepts a file path to a YAML file with values that are applied to the Fluvio [Helm chart].

[Helm chart]: {{< ref "./helm" >}}

For installing on a remote Kubernetes cluster where the machine running the CLI does not have network access to the cluster pods/services via NodePort, use the `--use-k8-port-forwarding` option. This will tunnel traffic to Fluvio cluster components via the Kubernetes API server. After installation you will need to manually configure a load balancer to expose Fluvio services externally.

See other options by running 

%copy first-line%
```bash
$ fluvio cluster start -h
```

