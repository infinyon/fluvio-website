---
title: Network Endpoints
weight: 20
---

# Exposed Services

By default Fluvio services are exposed via NodePort configuration.

SCs are exposed via port 9003 and SPUs are exposed via port 9005. Port numbers do not change based on whether TLS is enabled, the same ports are used regardlesss.

## Configuration

Service behavior can be configured at installation by overriding values in the helm charts.

The CLI takes a `--chart-values` option which accepts a file path to a YAML file with values that are applied to the Fluvio Helm chart.

## View Service Enpoint Details

Run `kubectl describe svc fluvio-sc-public` to view address and port information for accessing the SC public endpoint.

SPU service names follow the pattern `fluvio-spu-<group name>-<id>`, for example the first SPU of the "main" group is named `fluvio-spu-main-0`. It can be view with ``kubectl describe svc fluvio-spu-main-0`