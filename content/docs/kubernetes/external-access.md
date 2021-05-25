---
title: Access Fluvio Externally
menu: External Access
weight: 40
---

By default Fluvio services are exposed via NodePorts.

* SCs are exposed via port 9003
* SPUs are exposed via port 9005.

Port numbers do not change based on whether TLS is enabled, the same ports are used regardless.

Service behavior can be configured at installation by overriding values in the helm charts.

### Service Enpoint Details

Run `kubectl describe svc fluvio-sc-public` to view address and port information for accessing the SC public endpoint.

SPU service names follow the pattern `fluvio-spu-<group name>-<id>`, for example the first SPU of the "main" group is named `fluvio-spu-main-0`. It can be view with `kubectl describe svc fluvio-spu-main-0`
