---
title: Announcing Fluvio Cloud Platform
author: 
    name: "The Fluvio Team"
description: Today we are pleased to announce Fluvio Cloud, the easiest way to get started with Fluvio.
metadata: NEWS
date: 2021-02-26
slug: announcing-fluvio-cloud-platform
url: /blog/2021/02/announcing-fluvio-cloud-platform
img: blog/images/fluvio-cloud/social/cloud.jpg
tile: blog/images/fluvio-cloud/social/cloud-tile.svg
twitter-card: summary_large_image
hidden: true
---


Today we are pleased to announce Fluvio Cloud, the fastest and easiest way to get started using Fluvio.
Fluvio Cloud is now in alpha, and you can create a free account using the link below:

<center><a class="btn btn-primary" href="https://cloud.fluvio.io/signup" target="_blank" role="button">Sign Up for Fluvio Cloud</a></center>

Fluvio is an open-source, high-performance distributed data streaming platform for real-time apps, written
in Rust. Fluvio Cloud provisions and manages your Fluvio cluster for you, letting you get started right away.
Getting started is as simple as creating an account and installing the [Fluvio CLI], our all-in-one
tool for working with Fluvio.

[Fluvio CLI]: /docs/getting-started/

# About Fluvio

We believe that modern business requires real-time collaboration, analysis, and adaptation.
Yet, building real-time infrastructure is a painful, expensive, and error-prone endeavor.
One of our goals with Fluvio is to make it easy to build real-time applications by providing a
real-time application development platform. Fluvio is built on three core principles that guide
us in achieving this goal:

1) **Data is modeled as streams of events**, and streams should deliver data as fast as possible
2) **APIs and tools should be easy to use**, so that anybody can build real-time applications

## Data Streams

Our first principle is to use streams as our fundamental model of data.
Data streams represent a continuous flow of messages that describe events that have occurred.
The unit of a stream is a single message (sometimes called a record), and messages
can be as small or as large as needed to fit the needs of the application.
Fluvio's data streams are implemented as persistent logs, meaning that all messages get written
to disk and have a strict ordering, indicated by each message's offset in the stream, which never changes.
Our data streaming engine has been written with a focus on four key properties that
make it ideal for building production-ready real-time applications: speed, scale, retention, and resiliency.

### Speed

One of the primary goals of a streaming platform is to deliver messages from
point A to point B as quickly as possible.
Fluvio's streaming architecture is designed to maximize performance, and has a key
advantage over other streaming platforms: it's written from the ground-up in Rust.
Rust is a modern systems programming language that produces executables that
perform competitively with C and C++ programs, but prevents many types of errors
such as memory and concurrency errors via compile-time checks.
Rust also features automatic memory management, but does not have a garbage collector,
meaning that it doesn't suffer from the [stop-the-world] problem that JVM streaming
platforms experience. This means that Fluvio will be able to provide lower-latency
message delivery at a higher confidence interval.

[stop-the-world]: https://en.wikipedia.org/wiki/Tracing_garbage_collection#Stop-the-world_vs._incremental_vs._concurrent

### Scalability

Scalability is the ability of a system to gracefully handle massive
volumes of data. Fluvio's data streaming architecture was designed
with this in mind from the start. In Fluvio, streaming is performed
by Streaming Processing Units (or SPUs), which are deployed
cooperatively in a Kubernetes cluster. Naturally, each SPU has a
particular capacity of storage, processing power, and bandwidth, so
as the system demands increase, Fluvio scales horizontally by simply
adding more SPUs to the mix. This process is automatically coordinated
by the Streaming Controllers (SCs), giving the system an elastic
behavior which can cost-effectively handle spikes in traffic without
needing a long-term commitment to peak capacity.

On a more micro level, Fluvio achieves high scalability by leveraging
Rust's asynchronous programming model. This makes it easy to write code
that multitasks, efficiently performing work that is ready to be done
without wasting time on blocking work such as IO. This means that all
of Fluvio's components are fully utilizing the available hardware,
getting more done at a lower cost.

### Retention

For many applications, it can be very beneficial to retain a long-term history
of the events that have occurred. In traditional databases, the only thing that
is preserved is the sum of all past events - the "current" state. In contrast,
Fluvio's data streams are implemented as immutable logs, meaning that we can
not only view the current state, but also the entire state of the system at any
point in time. This is incredibly powerful, as it grants us capabilities that
otherwise wouldn't be possible. For example, we can test new product features
using historical data (think ML use-cases), generate audit reports, perform
retrospective analysis, and even rebuild in-memory databases.

### Availability

Finally, a characteristic that we always need in production systems is high
availability. Fluvio employs a number of strategies to ensure that it is
resilient to many different types of failures, keeping data readily available
at all times. Primarily, Fluvio data streams natively support replication, meaning
that data is guaranteed to have live copies on at least `n` different nodes,
where `n` is the configured "replication factor". This means that even in the
case of spurious node failures, Fluvio can always provide failover so that
reads and writes are redirected to the available nodes. Additionally, the
cluster can heal itself: if a failed node comes back online, it will resynchronize
itself and continue running; or, if the node fails to come back online, it
simply gets replaced, and a new node takes its place. For additional disaster
protection, Fluvio can provide replication between availability zones, meaning
that even a complete data center outage will not compromise the availability of
Fluvio data streams.

## Ease of Use

Our last principle is Ease of Use. We believe that by making Fluvio easy to use,
we will create opportunities for more people and more projects to get into the
world of real-time applications. The biggest way that we do this is by crafting
tools and APIs that are pleasant to use and which hide the complexity of the
underlying distributed system. Since Fluvio is open-source, we also welcome
feedback and ideas from the community in order to help us build the best user
experience possible.

### Powerful CLI

Fluvio's two main user interfaces are the Fluvio CLI and the programmatic client libraries.
The Fluvio CLI is an all-in-one tool for managing and interacting with your Fluvio
clusters, and lets you play with data and inspect the state of the system. From the
CLI, you can view your topics and partitions, produce and consume data, and even
stand up and tear down whole clusters. The CLI natively supports multiple profiles,
so you can interact with multiple clusters just by switching the active profile. This can
be useful when working with multiple environments such as dev/test/prod or local/cloud.

### Language Native APIs

Our programmatic APIs are built to leverage the idioms of each language. Our Rust client
offers a fully async interface and makes use of the ecosystem's `Future` and `Stream` traits
to integrate smoothly with other async streaming code. In Node, we make use of `AsyncIterator`
in order to enable `for await` syntax for iterating over records. We will be adding more
language clients in the future, and we're always open to feedback. Feel free to join our
[Discord chat] and let us know what language support you'd like to see.

### Declarative Management

Our client APIs make Fluvio a great choice for developers, but how OPs friendly is it?
Fluvio is built to run natively on Kubernetes, employing operator patterns in order to
self-govern cluster components and keep the system healthy. Fluvio is configured declaratively,
meaning that human operators simply declare the target state of the system, and the cluster
handles the rest. This also includes continuous state reconciliation - if a node goes down
or if the state of the system diverges from the target state, Fluvio will automatically take
action to return to the target state, meaning fewer midnight pager-duty alerts.

# Conclusion

Thanks for reading to the end! We hope that you're as excited as we are for
the future of real-time applications. If you liked the blog, feel free to
share it around or to come let us know your thoughts! We have a community
[Discord chat] if you'd like to pop in. If you want to get started building
with Fluvio, the easiest way to begin is with a [free Fluvio Cloud account]
and to follow our [getting started guide]!.

[Discord chat]: https://discordapp.com/invite/bBG2dTz
[free Fluvio Cloud account]: https://cloud.fluvio.io/signup
[getting started guide]: https://www.fluvio.io/docs/getting-started/
