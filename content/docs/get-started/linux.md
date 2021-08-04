---
title: Linux Installation
menu: Linux
weight: 20
---

## Install Fluvio CLI

The Fluvio CLI (_command-line interface_) is an all-in-one tool for setting up, interacting, and managing with Fluvio clusters.

Install the Fluvio CLI by running the following command:

%copy first-line%
```bash
curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```



## Required Packages for Local Fluvio cluster

1) [Docker]({{< ref "#install-docker" >}})
2) [Minikube]({{< ref "#install-minikube" >}})
3) [Kubectl]({{< ref "#install-kubectl" >}})
4) [Helm]({{< ref "#install-helm" >}})

If you have `docker`, `kubectl`, `helm`, and `minikube` already set up, then continue to steps for [running a local Fluvio cluster].

[running a local Fluvio cluster]: {{< ref "/docs/get-started/linux.md#start-fluvio-cluster" >}}

### Install Docker

Docker is a container engine which is used by Minikube to run a local Kubernetes cluster.

Follow the instructions for your Linux distro to install [Docker engine for Linux].

[Docker engine for Linux]: https://docs.docker.com/engine/install/#server 

### Install Minikube

Minikube is a tool for running a local Kubernetes cluster

Follow the instructions at the [Minikube installation page] to download and install `minikube` for your Linux distro.

[Minikube installation page]: https://minikube.sigs.k8s.io/docs/start/

#### Start a Kubernetes cluster
Start a Kubernetes cluster locally with minikube by running the following in a terminal window:

%copy first-line%
```bash
$ minikube start
```

### Install Kubectl

`kubectl` is the Kubernetes command-line tool. It is used to run commands against Kubernetes clusters.

Follow the instructions at the [kubectl installation page] and follow the instructions to download and install `kubectl` on Linux.

[kubectl installation page]: https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/ 

### Install Helm

Helm is the package manager for Kubernetes. 

Follow the instructions at the [helm installation page] and follow the instructions to download and install `helm` on Linux.

[helm installation page]: https://v3.helm.sh/docs/intro/install/ 

## Start Fluvio cluster 

You can start a Fluvio cluster by running `fluvio cluster start`.

If this is your first time starting a cluster in your session, be prepared to enter your password.

%copy first-line%
```bash
$ fluvio cluster start
✅ ok: Kubernetes config is loadable
✅ ok: Supported helm version is installed
✅ ok: Fluvio system charts are installed
✅ ok: Previous fluvio installation not found
Waiting up to 120 seconds for Fluvio cluster version check...
Successfully installed Fluvio!
```

### Verify cluster is running

You can start a Fluvio cluster by running `fluvio cluster start`.

If this is your first time starting a cluster in your session, be prepared to enter your password.

%copy first-line%
```bash
$ fluvio cluster start
✅ ok: Kubernetes config is loadable
✅ ok: Supported helm version is installed
✅ ok: Fluvio system charts are installed
✅ ok: Previous fluvio installation not found
Waiting up to 120 seconds for Fluvio cluster version check...
Successfully installed Fluvio!
```
### Verify cluster is running

We can verify that our Fluvio components are running with `kubectl`

All Fluvio pods in the `default` namespace should be running.

%copy first-line%

```bash
$ kubectl get po
NAME                         READY   STATUS    RESTARTS   AGE
fluvio-sc-695cfb4cf5-89lss   1/1     Running   0          15s
fluvio-spg-main-0            1/1     Running   0          9s
```


You can check that everything worked by listing out the cluster's [SPUs]({{< ref "/docs/architecture/spu">}}):

%copy first-line%
```bash
$ fluvio cluster spu list
 ID  NAME    STATUS  TYPE       RACK  PUBLIC                PRIVATE
  0  main-0  Online  "managed"   -    192.168.99.103:31314  fluvio-spg-main-0.fluvio-spg-main:9006
```

The public address of the cluster's [SPU]({{< ref "/docs/architecture/spu">}}) should match the IP from `minikube ip` and the NodePort for the `fluvio-spu-main-0` kubernetes service.

%copy first-line%
```bash
$ minikube ip
192.168.99.103
```

%copy first-line%
```bash
$ kubectl get svc
NAME                 TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)             AGE
fluvio-sc-internal   ClusterIP   10.99.183.216   <none>        9004/TCP            58s
fluvio-sc-public     NodePort    10.109.6.50     <none>        9003:30763/TCP      58s
fluvio-spg-main      ClusterIP   None            <none>        9005/TCP,9006/TCP   52s
fluvio-spu-main-0    NodePort    10.100.254.17   <none>        9005:31314/TCP      52s
kubernetes           ClusterIP   10.96.0.1       <none>        443/TCP             15h
```



## Hello, Fluvio!

Congratulations, you've successfully installed Fluvio on your local machine! 

Let's use the Fluvio CLI to play with some basic functionality.

The first thing we need to do is create a [topic].

%copy first-line%
```bash
$ fluvio topic create greetings
topic "greetings" created
```

Now that we have a topic, we can [produce] some messages!

Use the following command to send a message to the `greetings` topic:

%copy first-line%
```bash
$ echo "Hello, Fluvio" | fluvio produce greetings
Ok!
```

Finally, we can [consume] messages back from the topic

%copy first-line%
```bash
$ fluvio consume greetings -B -d
Hello, Fluvio
```

Way to go! You're well on your way to writing real-time distributed apps with Fluvio!

Next, check out our [Tutorials page] to see real-world examples of Fluvio in action.

[topic]: {{< ref "/cli/commands/topic.md" >}}
[produce]: {{< ref "/cli/commands/produce.md" >}}
[consume]: {{< ref "/cli/commands/consume.md" >}}
[Tutorials page]: https://www.infinyon.com/tutorials 

#### Related Topics
----------------

- ["Hello World" in Java](https://www.infinyon.com/tutorials/java/hello-world/)
- ["Hello World" in Node.js](https://www.infinyon.com/tutorials/node/hello-world/)
- ["Hello World" in Python](https://www.infinyon.com/tutorials/python/hello-world/)
- ["Hello World" in Rust](https://www.infinyon.com/tutorials/rust/hello-world/)

---

If you run into any problems along the way, make sure to check out our [troubleshooting]
page to find a fix.

[troubleshooting]: {{< ref "/docs/operations/troubleshoot.md" >}}
