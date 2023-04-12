---
title: "This Week in Fluvio #51"
date: 2023-04-11
weight: 20
---
Fluvio is a distributed, programmable streaming platform written in Rust.

---
Welcome to the 51st edition of this week in Fluvio.

For this edition we have more details on **Community Contribution** 

## Community update
[Carson Rajcan](https://github.com/crajcan) presented to our team a really cool contribution to the Fluvio OSS.

In the project Carson developed the functionality to apply SmartModule transformation for Producer - [PR #3014](crecdfhttps://github.com/infinyon/fluvio/pull/3014)

Here is Carson's experience...

## The Problem

Get the Stream Processing Unit to apply SmartModule transformations for the Producer requests coming from the CLI (before commit).

At present, Fluvio SmartModules are only applied for Consumer requests after being read from disk. Shaping the data before it enters the topic spares the Consumer or other downstream services from this burden.

## Why This Problem?

While investigating another issue, I had recently gained some insight into how the Stream Processing Unit handles Producer requests, as well as how it uses the Fluvio Smart Engine to transform records for the Consumer.

Critically, I noticed that the Smart Engine was well encapsulated -- it was going to be easy to repurpose.

Also that the Producer request handler was well organized, it wasn't going to be a nightmare to plug in some additional functionality.

## The Workflow

Where was I going to start? Well, TDD is my friend.

There were plenty of test cases for Consumer SmartModule transformations that used the CLI. All I had to do was move the SmartModule options from the Consumer commands over to the Producer commands.

In this example, we are processing email addresses on the Consumer side

```
echo "FooBar@test.com" | fluvio produce emails
fluvio consume emails -B -d --smartmodule lowercase
```

In this example, we are still processing email addresses, but now it is possible to do on the Producer side.

```
echo "FooBar@test.com" | fluvio produce emails --smartmodule lowercase
fluvio consume emails -B -d 
```

Both result in the same output with the email address using only lowercase letters:

```
foobar@test.com
```

## The Task

With my TDD workflow ready to go, I made the changes from the outside-in.

### In The CLI
    1. Added the SmartModule options to the CLI Produce Command.
    2. Used those arguments to build SmartModuleInvocation(s), a type used to model SmartModules during network requests.
    3. Added the SmartModuleInvocation(s) to the ProduceRequest

### For the transfer 
    4. Had to define how the SmartModuleInvocation(s) would be encoded & decoded

### On the SPU
    5. Translated the SmartModuleInvocation(s) into a SmartModuleChain, a type which can be passed to the Smart Engine.
    6. Finally, I fed the SmartModuleChain and the Produce requests's records to the SmartEngine.

## Problems I Faced

### Types In A New Domain

Learning to translate between types in someone else's codebase can be challenging. There are many types to become familiar with in the Stream Processing Unit. Notably, the SPU uses a few different types to model Records. You end up seeing `Batch<Records>`, `Batch<RawRecords>`, `Batch<MemoryRecords>` quite often. It took me a while to figure out when and where to use each, and how to convert between them.

### Compression and SmartModules

What happens when a Producer sends compressed records and requests the SPU performs a SmartModule Transformation?

The records must be decompressed so they can be fed to the SmartEngine, then compressed again before storage. To pull this off I had to dig up the code that performs the compression, decompression and figure out how to utilize it while handling Producer requests.

### What Next?

SmartModules that perform filtering and aggregation can now be applied before commit to save storage. Time intensive SmartModule operations can be performed on write, rather than while consuming.

## Upcoming features
Thank you for your feedback on Discord. We are working on a public road map that should be out soon.

Keep the feedback flowing in [our Discord channel] and let us know if you'd like to see the video of Carson walking through the code.


## Open positions
[Sr. Rust Cloud Software Engineer](https://infinyon.com/careers/cloud-engineer-senior-level/)
[Developer Advocate](https://infinyon.com/careers/developer-advocate-mid-senior-level/)

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
