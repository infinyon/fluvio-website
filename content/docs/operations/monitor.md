---
title: Monitor
weight: 10
---

These objects represent the state of the Fluvio cluster.

## Pods
`kubectl get pods` should show one for the SC and one for each SPU specified when installing Fluvio.

Example:

%copy first-line%
```bash
$ kubectl get pods
NAME                         READY   STATUS    RESTARTS   AGE
fluvio-sc-6458d598d6-qq2td   1/1     Running   0          3m35s
fluvio-spg-main-0            1/1     Running   0          3m28s
```

## Services
`kubectl get svc` should show one public and one internal service for the SC and also one public and one internal service for each SPU.

Example:

%copy first-line%
```bash
$ kubectl get svc
NAME                 TYPE           CLUSTER-IP       EXTERNAL-IP      PORT(S)             AGE
fluvio-sc-internal   ClusterIP      10.96.41.31      <none>           9004/TCP            4m18s
fluvio-sc-public     LoadBalancer   10.107.219.124   10.107.219.124   9003:30947/TCP      4m18s
fluvio-spg-main      ClusterIP      None             <none>           9005/TCP,9006/TCP   4m11s
fluvio-spu-main-0    LoadBalancer   10.111.223.127   10.111.223.127   9005:30023/TCP      4m11s
```

## CRDs
Fluvio stores internal metadata in K8s custom resources. [Fluvio CRDs]({{< ref "../kubernetes/crd" >}}).

To verify system state you can compare results from 

%copy%
```bash
kubectl get spugroups
kubectl get spu
kubectl get topics
kubectl get partitions
```

should respectively, match results from 

%copy%
```bash
fluvio cluster spg list
fluvio cluster spu list
fluvio cluster topics
fluvio partitions list
```