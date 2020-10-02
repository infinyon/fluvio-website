---
title: Installing Fluvio Locally
menu: Install Fluvio Locally
toc: true
weight: 40
---

Fluvio is built to run on [Kubernetes], an open-source system that automates the
deployment of containerized apps. If you haven't heard of Kubernetes or containers,
just think of them as tools that help run applications on many computers at once.
To get started with Fluvio locally, we'll need to install a version of Kubernetes 
called Minikube which is meant for testing out Kubernetes apps locally.

If you've worked with Kubernetes before and you already have `kubectl`, `helm`, and
`minikube` set up, feel free to scroll down to [installing fluvio on minikube].

[installing fluvio on minikube]: #installing-fluvio-on-minikube

## Installing Minikube

Head on over to the [Minikube installation page] and follow the instructions to
download and install `minikube`.

[Kubernetes]: https://kubernetes.io/
[Minikube installation page]: https://minikube.sigs.k8s.io/docs/start/

{{< caution >}}

**Note**: If you've never used a container technology like Docker before,
you might run into trouble at the `minikube start` step that looks like this

```shell
$ minikube start
😄  minikube v1.13.1 on Ubuntu 20.04
👎  Unable to pick a default driver. Here is what was considered, in preference order:
    ▪ docker: Not installed: exec: "docker": executable file not found in $PATH
    ▪ kvm2: Not installed: exec: "virsh": executable file not found in $PATH
    ▪ none: Not installed: exec: "docker": executable file not found in $PATH
    ▪ podman: Not installed: exec: "podman": executable file not found in $PATH
    ▪ virtualbox: Not installed: unable to find VBoxManage in $PATH
    ▪ vmware: Not installed: exec: "docker-machine-driver-vmware": executable file not found in $PATH

❌  Exiting due to DRV_NOT_DETECTED: No possible driver was detected. Try specifying --driver, or see https://minikube.sigs.k8s.io/docs/start/
```

If you run into this, try [installing docker] as described on the [minikube drivers]
page, then try again using `minikube start --driver=docker`

[installing docker]: https://hub.docker.com/search?q=&type=edition&offering=community&sort=updated_at&order=desc
[minikube drivers]: https://minikube.sigs.k8s.io/docs/drivers/docker/

{{< /caution >}}

Once you're done, you should be able to run the following command to
check that minikube is installed correctly.

-> **Note**: The version that you see may be different from the one shown here

```shell
$ minikube version
minikube version: v1.13.0
commit: eeb05350f8ba6ff3a12791fcce350c131cb2ff44
```

## Installing Kubectl

Minikube is nothing more than just a _mini_ Kubernetes. That means that we need to get
the Kubernetes tool `kubectl` in order to configure and interact with our mini cluster
just like you would with a regular cluster. Head on over to the [installing kubectl]
page to get that installed. Once you're done, you should be able to run the following
command to check that it's installed correctly.

[installing kubectl]: https://kubernetes.io/docs/tasks/tools/install-kubectl/

-> Remember, the versions you see may not match the versions shown here

```shell
$ kubectl version --short
Client Version: v1.19.1
Server Version: v1.19.0
```

{{<idea>}}

**Note**: Minikube can download the right version of `kubectl` for you, but you have to
access it by running `minikube kubectl -- <args>` rather than the typical `kubectl <args>`.
If you don't want to install another tool feel free to use it this way, but just remember
if we say to run something like `kubectl get pods`, you'll want to run
`minikube kubectl -- get pods`

{{</idea>}}

## Installing Helm

Helm is a package manager that makes it easy to install apps for Kubernetes. Fluvio is
packaged as a "Helm chart" (the name for a package in Helm), and the Fluvio CLI can
install that chart for you by calling the `helm` executable with all the right options.
In order to do this, you need to have `helm` installed.

You can install `helm` from the [helm releases page]. Once it's installed, make sure it
works by printing the version.

-> Once again, the version you see might not be the same!

```shell
$ helm version --short
v3.3.1+g249e521
```

[helm releases page]: https://github.com/helm/helm/releases

## Installing Fluvio on Minikube

Now that we've got all of our tools installed, let's actually get Fluvio up and running!
If you run into any problems along the way, make sure to check out our [troubleshooting]
page to find a fix.

[troubleshooting]: fluvio-local-faq.md

First, we have to open up a "minikube tunnel", which allows programs on our local machine
to connect to programs running in minikube. This is how Fluvio commands communicate with
the local Fluvio cluster in minikube. To do this, we need to run the following command:

```
$ sudo nohup minikube tunnel >/tmp/tunnel.out 2>/tmp/tunnel.out &
```

{{<caution>}}
Sometimes this doesn't work. If you're unable to make progress, or you can't see
`minikube tunnel` running after searching with `ps aux | grep tunnel`, then try opening
a new terminal window and running this:

```
sudo minikube tunnel
```

The `nohup` version of the command is preferable if it works for you, because it doesn't
keep your terminal busy. If you use the non-`nohup` version, you'll need to continue working
on a different terminal.

{{</caution>}}

{{<idea>}}

For the next section, it can be helpful to have some visibility into what Kubernetes is
doing during the installation. Try opening a new terminal window on the side and running
this command:

```
$ watch kubectl get all --all-namespaces
NAMESPACE     NAME                                   READY   STATUS    RESTARTS   AGE
kube-system   pod/coredns-f9fd979d6-rdz2p            1/1     Running   0          16h
kube-system   pod/etcd-minikube                      1/1     Running   0          16h
kube-system   pod/kube-apiserver-minikube            1/1     Running   0          16h
kube-system   pod/kube-controller-manager-minikube   1/1     Running   0          16h
kube-system   pod/kube-proxy-lp4kd                   1/1     Running   0          16h
kube-system   pod/kube-scheduler-minikube            1/1     Running   0          16h
kube-system   pod/storage-provisioner                1/1     Running   0          16h

NAMESPACE     NAME                 TYPE        CLUSTER-IP   EXTERNAL-IP   PORT(S)                  AGE
default       service/kubernetes   ClusterIP   10.96.0.1    <none>        443/TCP                  16h
kube-system   service/kube-dns     ClusterIP   10.96.0.10   <none>        53/UDP,53/TCP,9153/TCP   16h

NAMESPACE     NAME                        DESIRED   CURRENT   READY   UP-TO-DATE   AVAILABLE   NODE SELECTOR            AGE
kube-system   daemonset.apps/kube-proxy   1         1         1       1            1           kubernetes.io/os=linux   16hNAMESPACE     NAME                      READY   UP-TO-DATE   AVAILABLE   AGE
kube-system   deployment.apps/coredns   1/1     1            1           16h

NAMESPACE     NAME                                DESIRED   CURRENT   READY   AGE
kube-system   replicaset.apps/coredns-f9fd979d6   1         1         1       16h
```

This will refresh every 2 seconds, showing you the pods and services running in Kubernetes

{{</idea>}}

Kubernetes apps often come in two halves - a so-called "system" chart, and an "app" chart.
We need to install the system chart first. To do that, run the following:

```shell
$ fluvio cluster install --chart-version=0.6.0-latest --image-version=latest --sys
"fluvio" has been added to your repositories
Hang tight while we grab the latest from your chart repositories...
...Successfully got an update from the "fluvio" chart repository
Update Complete. ⎈Happy Helming!⎈
NAME: fluvio-sys
LAST DEPLOYED: Fri Oct  2 09:30:44 2020
NAMESPACE: default
STATUS: deployed
REVISION: 1
TEST SUITE: None
fluvio sys chart has been installed
```

Now, we can install the "app" chart:

```shell
$ fluvio cluster install --chart-version=0.6.0-latest --image-version=latest
"fluvio" already exists with the same configuration, skipping
Hang tight while we grab the latest from your chart repositories...
...Successfully got an update from the "fluvio" chart repository
Update Complete. ⎈Happy Helming!⎈
NAME: fluvio
LAST DEPLOYED: Fri Oct  2 10:20:52 2020
NAMESPACE: default
STATUS: deployed
REVISION: 1
TEST SUITE: None
waiting for sc service up come up: 0
waiting for sc service up come up: 1
0 of 1 spu ready
0 of 1 spu ready
0 of 1 spu ready
0 of 1 spu ready
0 of 1 spu ready
0 of 1 spu ready
waiting for spu to be provisioned
1 spus provisioned
```

{{<idea>}}

If you have the watch window open with `watch kubectl get all --all-namespaces`, you
should now be able to see Fluvio's pods and services running!

```
NAMESPACE     NAME                                   READY   STATUS    RESTARTS   AGE
default       pod/flv-sc                             1/1     Running   0          4m59s
default       pod/flv-spg-main-0                     1/1     Running   0          4m52s
...

NAMESPACE     NAME                      TYPE           CLUSTER-IP       EXTERNAL-IP     PORT(S)                  AGE
default       service/flv-sc-internal   ClusterIP      10.102.107.247   <none>          9004/TCP                 4m59s
default       service/flv-sc-public     LoadBalancer   10.110.110.66    10.110.110.66   9003:31763/TCP           4m59s
default       service/flv-spg-main      ClusterIP      None             <none>          9005/TCP,9006/TCP        4m52s
default       service/flv-spu-main-0    LoadBalancer   10.108.48.176    10.108.48.176   9005:31135/TCP           4m52s
...
```

{{</idea>}}

You can check that everything worked by listing the Topics on the cluster:

```
$ fluvio topic list
No topics found
```

## Ready to go!

Congratulations, you've successfully installed Fluvio on your local machine! 
Head on over to our [Hello World] tutorial to learn how to start producing
and consuming messages!

[Hello World]: ../../tutorials/hello-world-node.md
