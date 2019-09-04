---
title: Install Fluvio - Streaming Controller (SC)
menu: Install Fluvio
weight: 40
---

Run following snippets to install Fluvio schema to Kubernetes:

{{< cli yaml>}}
./k8-util/install.sh
{{< /cli>}}


To deploy Fluvio services:

{{< cli yaml>}}
./k8-util/deploy.sh
{{< /cli>}}

Then you should see Streaming controller pod:
{{< cli yaml>}}
$ kubectl get pods
NAME     READY   STATUS    RESTARTS   AGE
flv-sc   1/1     Running   0          11m
{{< /cli>}}

And services like below:
{{< cli yaml>}}
$ kubectl get svc
NAME              TYPE           CLUSTER-IP       EXTERNAL-IP     PORT(S)          AGE
flv-sc-internal   ClusterIP      10.101.198.230   <none>          9004/TCP         11m
flv-sc-public     LoadBalancer   10.102.216.65    10.102.216.65   9003:31333/TCP   11m
kubernetes        ClusterIP      10.96.0.1        <none>          443/TCP          21m
{{< /cli>}}

Note that service ```flv-sc-public``` must have EXTERNAL-IP to be reachable from CLI.

{{< links "Next Steps" >}}
* [Install CLI]({{< relref "install-cli" >}})
{{< /links >}}