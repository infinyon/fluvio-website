---
title: Monitor
weight: 30
---

# Items to check in Kubernetes

## Pods
`kubectl get pods` should show one for the SC and one for each SPU specified when installing Fluvio.

Example:
```
❯ kubectl get pods
NAME                         READY   STATUS    RESTARTS   AGE
fluvio-sc-6458d598d6-qq2td   1/1     Running   0          3m35s
fluvio-spg-main-0            1/1     Running   0          3m28s
```

## Services
`kubectl get svc` should show one public and one internal service for the SC and also one public and one internal service for each SPU.

Example:
```
❯ kubectl get svc
NAME                 TYPE           CLUSTER-IP       EXTERNAL-IP      PORT(S)             AGE
fluvio-sc-internal   ClusterIP      10.96.41.31      <none>           9004/TCP            4m18s
fluvio-sc-public     LoadBalancer   10.107.219.124   10.107.219.124   9003:30947/TCP      4m18s
fluvio-spg-main      ClusterIP      None             <none>           9005/TCP,9006/TCP   4m11s
fluvio-spu-main-0    LoadBalancer   10.111.223.127   10.111.223.127   9005:30023/TCP      4m11s
```

## CRDs
Results from 
```
kubectl get spu
kubectl get topics
kubectl get partitions
```
should match results from 
```
fluvio cluster spu list
fluvio cluster topics
fluvio partitions list
```
respectively.
