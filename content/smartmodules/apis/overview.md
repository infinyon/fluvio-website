---
title: SmartModule API Library
menu: Overview
toc: true
weight: 10
---

SmartModule APIs are programmable data streaming functions exposed to WebAssembly. SmartModules allows developers to manipulate data in real-time inside the Fluvio cluster, without the need for external servicees such as Lambda or Functions. The APIs when chained together in data pipelines, make it possible to implement and run full featured data streaming applications in Fluvio. 

The API Library offers a growing number of APIs for in-line data transformations:

* [Filter]({{<ref "#filter" >}})
* [Map]({{<ref "#map" >}})
* [FilterMap]({{<ref "#filtermap" >}})
* [ArrayMap]({{<ref "#arraymap" >}})
* [Aggregate]({{<ref "#aggregate" >}})


### Filter

A [Filter SmartModule]({{<ref "filter" >}}) takes an input record and allows you to check if the record value meets certain criteria.

If `false`, the record is discarded, and if `true`, the record is saved, and continues downstream.

<img src="/smartmodules/images/smartmodule-filter.svg" alt="SmartModule Filter" justify="center" height="180">

### Map
A [Map SmartModule]({{<ref "map" >}}) takes an input record allows you to apply any data transformations to the record before it continues downstream.

"Map" refers to the [programming language term](https://en.wikipedia.org/wiki/Map_(higher-order_function)), which simply is a function that is applied to all input data.


<img src="/smartmodules/images/smartmodule-map.svg" alt="SmartModule Map" justify="center" height="180">


### FilterMap

A [FilterMap SmartModule]({{<ref "filter-map" >}}) takes one input record and returns zero or one output record.

As the name may imply, FilterMap is the combination of [filter]({{<ref "#filter" >}}) and [map]({{<ref "#map" >}})

You can check for conditions in the data and if met, apply transformations. Or if the conditions are not met, discard the record.

<img src="/smartmodules/images/smartmodule-filtermap.svg" alt="SmartModule FilterMap" justify="center" height="180">


### ArrayMap

An [ArrayMap SmartModule]({{<ref "array-map" >}}) takes one input record and returns zero or many output records.

The Array in ArrayMap refers to [a JSON array](https://www.w3schools.com/js/js_json_arrays.asp).

Given a single record that is a JSON array, you may flatten the single input array. The result is the creation of several individual records, which you may additionally apply transformations before returning.

<img src="/smartmodules/images/smartmodule-arraymap.svg" alt="SmartModule ArrayMap" justify="center" height="180">


### Aggregate

An [Aggregate SmartModule]({{<ref "aggregate" >}}) ability to create a feedback loop, by providing a Accumulator Record that is persistent per stream. Each event may modify the Accumulator, and the changes will be available for next input to use.

The value of the Accumulator Record is also returned after each input. 

For example, if you're trying to Sum up stream of numbers, you would add each input value to the current value of the Accumulator Record.

<img src="/smartmodules/images/smartmodule-aggregate.svg" alt="SmartModule Aggregate" justify="center" height="220">
