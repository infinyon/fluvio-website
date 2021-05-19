---
title: Helm Chart Values
weight: 20
---

## Configuration

The CLI takes a `--chart-values` option, during install and upgrade, which accepts a file path to a YAML file with values that are applied to the Fluvio Helm chart.

## Chart Values

Following are the default values:

```
service:
  type: NodePort # Passed to K8s services
loadBalancer:
  serviceAnnotations: {} # Annotations to add to service objects
scLog: info # Log level, one of trace, debug, info, warn or error
tls: false # Enable TLS on SC and SPU endpoints
image: # Fluvio image to deploy
  registry: infinyon
  tag: latest
  pullPolicy: IfNotPresent
cert: 
  caCert: fluvio-ca # K8s TLS Secret with CA certificate
  tls: fluvio-tls # K8s TLS Secret with server certificate and key
scPod: # Passed to SC pod spec
  resources:
    requests:
      memory: 512Mi
    limits:
      memory: 512Mi
  nodeSelector: {}
spuPod: # Passed to SPU pod spec
  resources:
    requests:
      memory: 256Mi
    limits:
      memory: 1Gi
  nodeSelector: {}
serviceAccount: # Service account used by SC for storing metadata and configuring managed SPU groups
  name: fluvio
spuStorageClass: fluvio-spu # Storage class to use for SPU PVs
```
