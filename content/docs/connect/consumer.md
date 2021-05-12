---
title: Consumers
weight: 20
---

- Consumers are applications that take action in response to new records
- Consumers may specify an offset into a partition to read records
- Since records are persistent, they may be read more than one time
- Different consumer applications may be consuming from different offsets at once
- Consumers typically read from a specified Topic/Partition combo
