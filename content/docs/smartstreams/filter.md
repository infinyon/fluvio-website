---
title: Filter
weight: 20
---

- Filters are SmartStreams that may accept or reject records
- Accepted records are passed on to consumers
- Rejected records remain in the topic but are not sent to consumers
- A filter is a WASM function that is provided with a record and must return a bool
