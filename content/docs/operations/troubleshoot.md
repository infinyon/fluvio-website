---
title: Troubleshoot
weight: 50
---

# Troubleshooting

## Logs

When debuging errors, the first step should be to examle the logs from the following components:

### SC
```
kubectl logs -l app=fluvio-sc
```
### SPU
```
kubectl logs -l app=spu
```

## Handling Bugs

### Records logs and create and GitHub Issue

In the event of a bug in Fluvio, we appreciate if you could save the log output to file and create a [GitHub Issue](https://github.com/infinyon/fluvio/issues/new?assignees=&labels=bug&template=bug_report.md&title=%5BBug%5D%3A).

### Restart Pods

To attempt to recover from the bug, you can try restarting the K8s pods. 

```
kubectl delete pod -l app=fluvio-sc
kubectl delete pod -l app=spu
```

Fluvio pods are created by either Deployments or StatefulSets, therefore deleting them will automatically cause new pods to be started.