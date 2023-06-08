---
title: Regex 
weight: 40
---

This is a [filter-type]({{<ref "../transform/filter.md" >}}) SmartModule that tests the input record against a provided regular expression. The record is returned if there is a match.

### Usage example
To demonstrate how **Regex** SmartModule works we will use [SMDK]({{<ref "../smdk/build-test.md" >}}) tool. 

First, we need to download it to our cluster:

%copy first-line%
```shell
$ fluvio hub download infinyon/regex-filter@0.1.0
```

Second, we create a file `transform.yaml` defining our regular expression:

%copy%
```yaml
# transform.yaml
transforms:
- uses: infinyon/regex-filter@0.1.0
  with:
    regex: "[Cc]at" 
```

Let's use `smdk test` to see it in action:


%copy first-line%
```shell
$ smdk test --text '{"fact": "Cats have supersonic hearing"}' --transforms-file ./transform.yaml
{"fact": "Cats have supersonic hearing"}
```

%copy first-line%
```shell
$ smdk test --text '{"fact": "Dogs have sweat glands at the bottom of their paws"}' --transforms-file ./transform.yaml
[No output returned]
```