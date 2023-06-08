---
title: SmartModule Analytics 
menu: Overview
toc: true
weight: 10
_build:
  render: never
hidden: true
---

Fluvio SmartModules support the following in-line data analytics patterns:

* [Aggregate]({{<ref "#aggregate" >}})

### Aggregate

An [Aggregate SmartModule]({{<ref "aggregate" >}}) ability to create a feedback loop, by providing a Accumulator Record that is persistent per stream. Each event may modify the Accumulator, and the changes will be available for next input to use.

The value of the Accumulator Record is also returned after each input. 

For example, if you're trying to Sum up stream of numbers, you would add each input value to the current value of the Accumulator Record.

<img src="/smartmodules/images/smartmodule-aggregate.svg" alt="SmartModule Aggregate" justify="center" height="220">

Checkout [`aggregate section`] for an example.

[`filter section`]: {{< ref "filter" >}}
[`map section`]: {{< ref "map" >}}
[`filter-map section`]: {{< ref "filter-map" >}}
[`array-map section`]: {{< ref "array-map" >}}
[`aggregate section`]: {{< ref "aggregate" >}}