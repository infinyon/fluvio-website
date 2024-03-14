---
title: Fluvio Network Access
menu: Network Access
weight: 40
---

By default Fluvio helm charts setup services exposed via NodePorts.

Port numbers do not change based on whether TLS is enabled, the same ports are used regardless. Additional tls ports may be used.

Service behavior can be configured at installation by overriding values in the helm charts.

### Service Endpoint Details

Run `kubectl describe svc fluvio-sc-public` to view address and port information for accessing the SC public endpoint.
The configuration may vary by networking configuration as well as the type of kubernetes cluster installed.

In this case the accss point is the ingress ip address on pord desribed by the NodePort: `127.0.0.1:30003`

```shell
$ kubectl describe svc  fluvio-sc-public
Name:                     fluvio-sc-public
Namespace:                default
Labels:                   app.kubernetes.io/managed-by=Helm
Annotations:              fluvio.io/ingress-address: 127.0.0.1
                          meta.helm.sh/release-name: fluvio-app
                          meta.helm.sh/release-namespace: first
Selector:                 app=fluvio-sc
Type:                     NodePort
IP Family Policy:         SingleStack
IP Families:              IPv4
IP:                       10.43.5.147
IPs:                      10.43.5.147
Port:                     <unset>  9003/TCP
TargetPort:               9003/TCP
NodePort:                 <unset>  30003/TCP
Endpoints:                10.42.0.9:9003
Session Affinity:         None
External Traffic Policy:  Cluster
Events:                   <none>
```


SPU service names follow the pattern `fluvio-spu-<group name>-<id>`, for example the first SPU of the "main" group is named `fluvio-spu-main-0`. It can be viewed with `kubectl describe svc fluvio-spu-main-0`
