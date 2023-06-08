---
title: SmartModule Hub
menu: Overview
toc: false
weight: 10
---


The SmartModule Hub service, is a where developers publish SmartModules, and users download and integrate them into their data pipelines. To use this service you need an [`InfinyOn Cloud`] account.

<img src="/smartmodules/images/hub.jpg" alt="SmartModule Hub" justify="center" style="width: 85%; min-width: 330px;" >

Each SmartModule in the Hub is uniquely identified by a `group`, `name`, and `version`.

For example, the following SmartModules is published by `InfinyOn`:

```bash
infinyon/sql-json@0.1.0
```

### Private/Public SmartModules

SmartModules published to the Hub can be `public` or `private`. Public SmartModules are visible and downloadable by anyone, whereas private SmartModules are only visible to the owner. 

-> This feature is under development; please reach out on [`Discord`] to request early access.

### Certified SmartModules

Certified SmartModules are publicly available to anyone with an Infinyon Cloud account.

Currently only InfinyOn SmartModules are certified. Please reach out on [`Discord`] if you are interested in the **SmartModule Certification** program.

[`InfinyOn Cloud`]: https://infinyon.cloud/
[`Discord`]: https://discord.gg/zHsWBt5Z2n
[`SMDK section`]: {{< ref "/smartmodules/smdk/overview" >}}
[`SMDK cli`]: {{< ref "/cli/smartmodules/smdk" >}}
[`Hub cli`]: {{< ref "/cli/smartmodules/hub" >}}
[`List`]: {{< ref "list" >}}
[`Download`]: {{< ref "download" >}}
