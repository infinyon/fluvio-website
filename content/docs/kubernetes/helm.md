---
title: Helm Chart
weight: 20
---

The CLI takes a `--chart-values` option, during install and upgrade, which accepts a file path to a YAML file with values that are applied to the Fluvio Helm chart.

The image.tag value should be replaced with a fluvio release version e.g. ("0.11.5").

## Chart Values

Following are the default values:

```yaml
service:
  type: NodePort # Passed to K8s services
loadBalancer:
  serviceAnnotations: {} # Annotations to add to service objects
scLog: info # Log level, one of trace, debug, info, warn or error
tls: false # Enable TLS on SC and SPU endpoints
image: # Fluvio image to deploy
  registry: infinyon
  tag: ""
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
  storageClass: null   # Storage class to use for SPU volume
rbac:
  create: true
serviceAccount: # Service account used by SC for storing metadata and configuring managed SPU groups
  name: fluvio
podSecurityContext: {}   # Override
```
