---
title: SmartModule Transform
menu: Overview
toc: true
weight: 10
---

Fluvio SmartModules support the following in-line data transformation patterns:

* [Filter]({{<ref "#filter" >}})
* [Map]({{<ref "#map" >}})
* [FilterMap]({{<ref "#filtermap" >}})
* [ArrayMap]({{<ref "#arraymap" >}})


### Filter

A [Filter SmartModule]({{<ref "filter" >}}) takes an input record and allows you to check if the record value meets certain criteria.

If `false`, the record is discarded, and if `true`, the record is saved, and continues downstream.

<img src="/smartmodules/images/smartmodule-filter.svg" alt="SmartModule Filter" justify="center" height="180">

Checkout [`filter section`] for an example.

### Map
A [Map SmartModule]({{<ref "map" >}}) takes an input record allows you to apply any data transformations to the record before it continues downstream.

"Map" refers to the [programming language term](https://en.wikipedia.org/wiki/Map_(higher-order_function)), which simply is a function that is applied to all input data. 

<img src="/smartmodules/images/smartmodule-map.svg" alt="SmartModule Map" justify="center" height="180">

Checkout [`map section`] for an example.

### FilterMap

A [FilterMap SmartModule]({{<ref "filter-map" >}}) takes one input record and returns zero or one output record.

As the name may imply, FilterMap is the combination of [filter]({{<ref "#filter" >}}) and [map]({{<ref "#map" >}})

You can check for conditions in the data and if met, apply transformations. Or if the conditions are not met, discard the record.

<img src="/smartmodules/images/smartmodule-filtermap.svg" alt="SmartModule FilterMap" justify="center" height="180">

Checkout [`filter-map section`] for an example.

### ArrayMap

An [ArrayMap SmartModule]({{<ref "array-map" >}}) takes one input record and returns zero or many output records.

The Array in ArrayMap refers to [a JSON array](https://www.w3schools.com/js/js_json_arrays.asp).

Given a single record that is a JSON array, you may flatten the single input array. The result is the creation of several individual records, which you may additionally apply transformations before returning.

<img src="/smartmodules/images/smartmodule-arraymap.svg" alt="SmartModule ArrayMap" justify="center" height="180">

Checkout [`array-map section`] for an example.

[`filter section`]: {{< ref "filter" >}}
[`map section`]: {{< ref "map" >}}
[`filter-map section`]: {{< ref "filter-map" >}}
[`array-map section`]: {{< ref "array-map" >}}