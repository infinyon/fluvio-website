---
title: SmartModule Hub
menu: Overview
toc: false
weight: 10
---
SmartModules can be published to [SmartModule Hub] and downloaded to multiple clusters. Once downloaded, SmartModules can be chained together to build powerful data transformation pipelines.

SmartModule Hub, powered by [`InfinyOn Cloud`], is a real-time apps store, where developers publish SmartModules, and users download and integrate them into their data pipelines. 

<img src="/smartmodules/images/hub.jpg" alt="SmartModule Hub" justify="center" style="width: 85%; min-width: 330px;" >

Thie sections outlines core concepts of the SmartModule Hub.
* [Unique Identifier](#unique-identifier)
* [Personas](#personas)
* [Certified SmartModules](#certified-smartmodules)
* [Private/Public SmartModules](#privatepublic-smartmodules)

If you want to skip ahead, checkout [SmartModule Operations](#smartmodule-operations). 

### Unique Identifier

Each SmartModule in the Hub is uniquely identified by a `group`, `name`, `version` aggregate. For example, the following SmartModules is published by `InfinyOn`:

```bash
infinyon/sql-json@0.1.0
```

SmartModule developers must choose a group `name` before they are allowed to publish to the Hub. Checkout [`SMDK section`] for more information. 

### Personas

The Hub use cases are centered around two distinct personas: `developers` and `users`. Developers create, test, and upload SmartModules, whereas users search and download SmartModules for their data pipelines. This separation enables non-developer pipeline operators to benefit from SmartModules availabe on the SmartModule Hub.

* **Developers** use [`SMDK cli`]: 

    ```bash
    $ smdk ...
    ```

* **Users** use [`Hub cli`]:

    ```bash
    $ fluvio hub ...
    ```

### Certified SmartModules

We built SmartModule Hub to democratize real-time application development. This mission mandates that anyone in the Fluvio community can upload SmartModules to the Hub. 
While we use advanced security techniques to guarantee ownership, and walled gardens to sandbox runtime functions, we cannot cannot guarantee the quality and intent of the community developed SmartModules.

**Certified SmartModules** alleviate this concern. A certified SmartModule ensures the SmartModule has been vetted by the InfinyOn team and deemed safe to use. 

Currently only InfinyOn SmartModules are certified. We intend to roll-out a certification process that enables the community to certify their own SmartModules. Please reach out on [`Discord`] if you are interested in the **SmartModule Certification** program.


### Private/Public SmartModules

SmartModules published to the Hub can be `public` or `private`. Public SmartModules are visible and downloadable by anyone, whereas private SmartModules are only visible to the owner. 

-> This feature is under development; please reach out on [`Discord`] to request early access.


## SmartModule Operations

* [`List`] SmartModules in the Hub
* [`Download`] SmartModules from the Hub.


[`InfinyOn Cloud`]: https://infinyon.cloud/
[`Discord`]: https://discord.gg/zHsWBt5Z2n
[`SMDK section`]: {{< ref "/smartmodules/smdk/overview" >}}
[`SMDK cli`]: {{< ref "/cli/smartmodules/smdk" >}}
[`Hub cli`]: {{< ref "/cli/smartmodules/hub" >}}
[`List`]: {{< ref "list" >}}
[`Download`]: {{< ref "download" >}}
