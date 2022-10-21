---
title: Cloud Usage
menu: Usage
toc: false
weight: 100
---

Query the CPU and memory usage of your SPUs.

## `fluvio cloud usage`

Example usage:

%copy first-line%
```bash
$ fluvio cloud usage 
 SPU     CPU       Memory 
 main-0  1066037n  3168Ki 

Note: CPU usage is expressed in nanocores. 1 nanocore is equal to 1 billionth of 1 core.
```
