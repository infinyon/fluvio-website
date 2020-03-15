---
title: Install Fluvio - Streaming Controller (SC)
menu: Install Fluvio
weight: 40
---

Run the following shell script to install Fluvio schema on your Kubernetes installation:

{{< fluvio >}}
$ ./k8-util/install.sh
{{< /fluvio >}}

Next, deploy Fluvio services:

{{< fluvio >}}
$ ./k8-util/deploy.sh
{{< /fluvio >}}

Ensure the Streaming Controller pod is running:

{{< fluvio >}}
$ kubectl get pods
NAME     READY   STATUS    RESTARTS   AGE
flv-sc   1/1     Running   0          11m
{{< /fluvio >}}

Ensure the following services have been successfully provisioned:

{{< fluvio >}}
$ kubectl get svc
NAME              TYPE           CLUSTER-IP       EXTERNAL-IP     PORT(S)          AGE
flv-sc-internal   ClusterIP      10.101.198.230   <none>          9004/TCP         11m
flv-sc-public     LoadBalancer   10.102.216.65    10.102.216.65   9003:31333/TCP   11m
kubernetes        ClusterIP      10.96.0.1        <none>          443/TCP          21m
{{< /fluvio >}}


The services are operational, let's install the CLI.

{{< caution >}}
Service ```flv-sc-public``` must have EXTERNAL-IP to be reachable from CLI.
{{< /caution >}}

{{< links "Next Steps" >}}
* [Install CLI]({{< relref "install-cli" >}})
{{< /links >}}