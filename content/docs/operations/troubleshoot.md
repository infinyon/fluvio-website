---
title: Troubleshooting
weight: 20
---

## Run cluster check

To diagnose abnormal behavior, a good first step is to run `fluvio cluster check`, which checks against common problems and misconfigurations. 

If everything is configured properly, you should see a result like this:

```bash
$ fluvio cluster check
Running pre-startup checks...
✅ ok: Kubernetes config is loadable
✅ ok: Supported kubernetes version is installed
✅ ok: Supported helm version is installed
✅ ok: Can create service
✅ ok: Can create customresourcedefinitions
✅ ok: Can create secret
✅ ok: Load balancer is up
✅ ok: Fluvio system charts are installed
All checks passed!
You may proceed with cluster startup
```

## Logs

To discover errors, you should examine logs from the following components:

### SC
```bash
kubectl logs -l app=fluvio-sc
```
### SPU
```bash
kubectl logs -l app=spu
```

## Handling Bugs

### Records logs and create and GitHub Issue

In the event of a bug in Fluvio, we appreciate if you could save the log output to file and create a [GitHub Issue](https://github.com/infinyon/fluvio/issues/new?assignees=&labels=bug&template=bug_report.md&title=%5BBug%5D%3A).

### Reach out to community

[Discord](https://discord.gg/zHsWBt5Z2n)

### Restart Pods

To attempt to recover from the bug, you can try restarting the K8s pods. 

```bash
kubectl delete pod -l app=fluvio-sc
kubectl delete pod -l app=spu
```

Fluvio pods are created by either Deployments or StatefulSets, therefore deleting them will automatically cause new pods to be started.
