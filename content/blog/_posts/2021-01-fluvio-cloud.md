---
title: Announcing Fluvio Cloud Platform
author: 
    name: "The Fluvio Team"
description: Today we are pleased to announce Fluvio Cloud, the easiest way to get started with Fluvio.
metadata: NEWS
date: 2021-01-10
slug: announcing-fluvio-cloud-platform
url: /blog/2021/01/announcing-fluvio-cloud-platform
img: blog/images/fluvio-cloud/social/cloud.jpg
tile: blog/images/fluvio-cloud/social/cloud-tile.svg
twitter-card: summary_large_image
hidden: true
---

Outline:

<center><a class="btn btn-primary" href="/signup" role="button">Sign Up for Fluvio Cloud</a></center>

Cloud Introduction: We are announcing Fluvio Cloud, an easy way to get started with Fluvio

    Today we are pleased to announce Fluvio Cloud, the fastest and easiest way to get started using Fluvio. Fluvio Cloud is now in alpha, and you can create a free account using the link below:

    <center><a class="btn btn-primary" href="https://cloud.fluvio.io/signup" role="button">Sign Up for Fluvio Cloud</a></center>

    Fluvio is an open-source, high-performance distributed data streaming platform for real-time apps. Using Fluvio is simple, but like many distributed systems, operating and managing it is more difficult. Fluvio Cloud provisions and manages your Fluvio cluster for you, letting you get started right away. Getting started is as simple as creating an account and installing the [Fluvio CLI], our all-in-one tool for working with Fluvio.

    [Fluvio CLI]: /docs/getting-started/

Fluvio Mission: What big problems are we trying to solve?

    We believe that modern business requires real-time collaboration, analysis, and adaptation.
    Yet, the challenge that many groups are facing is that building real-time infrastructure is a painful, expensive, and error-prone endeavor. [NEEDS WORK]
    Our mission with Fluvio is to make building real-time applications easy.


- Motivation: Competitive modern businesses require real-time applications
  - Why do we want real-time applications in the first place?
    - Real-time applications enable live collaboration
    - Allow quicker business decisions
  - What do we want to see in a real-time platform?
    - Easy integration with services (e.g. polyglot, microservices, third-party)
  
- Problem: Existing data streaming solutions are hard to use
  - Problem: Existing data streaming infrastructure requires multiple components to be stitched together to be useful
- Problem: Existing streaming APIs are limited (no easy CLI, lacking good language bindings)
  - Problem: All existing streaming platforms are built on the JVM, harming performance
    - Solution: Use a distributed streaming platform written in Rust
    - This is the shining point that makes us unique on a technical level, we need to nail it
- Problem: How can I integrate many services and organizations in real-time?
  - Services built on stateful data stores (classic databases) have troubles scaling between many services
  - Solution: Use a real-time data streaming platform


Introduction Draft

    We believe that modern business requires real-time collaboration, analysis, and adaptation.
    Yet, the challenge that many groups are facing is that building real-time infrastructure is a painful, expensive, and error-prone endeavor.
    Our mission with Fluvio is to make building real-time applications easy.

    We set out to solve these problems by creating Fluvio, a general-purpose real-time application platform, built on a distributed streaming engine written in Rust. Rust is a systems programming language offering high performance and code safety, and offers us a much lower latency floor than the JVM could.

Fluvio Pillars: This will shape the rest of the blog
1. [Data Streams](#data-streams)
2. [Collaboration](#collaboration)
3. [Ease of Use](#ease-of-use)

Pillar: Data streams
- Overview: what is a data stream?
  - A continuous sequence of events
  - Consists of individual messages that describe events
  - Average time to act on stream events is very low
  - Are necessary for building real-time applications
  - Properties of Fluvio data streams:
    - Speed, scale, retention, resilience (next four paragraphs)
  

    Our first pillar is to use streams as our fundamental model of data.
    Data streams represent a continuous flow of messages that describe events that have occurred.
    The unit of a stream is a single message (sometimes called a record), and messages
    can be as small or as large as needed to fit the needs of the application.
    Fluvio's data streams are implemented as persistent logs, meaning that all messages get written
    to disk and have a strict ordering, indicated by each message's offset in the stream, which never changes.
    Treating data as streams in this way allows us to provide crucial advantages that are
    necessary for building production-ready real-time applictions. Namely: speed, scale, retention, and resiliency.

- Speed
  - Need to fully utilize the computer hardware
  - Leverage Rust for the low-level control to achieve this
  - Rust advantage over other streaming software is no garbage-collection


    One of the primary goals of a streaming platform is to deliever messages from
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



    An immediate question comes to mind: how big is a message?
    The answer will vary depending on the nature of the application being built.
    A general rule of thumb is that larger messages take longer to send.
    This allows for great flexibility when considering the performance and persistence requirements
    of different applications.
    For one, it allows us to optimize our platform for many different use-cases.



- Scale
  - Organizations need a foundation that can grow with business
  - Should not need to rewrite an app to scale with popularity
  - It should be easy to scale up and down with demand
  - Fluvio horizontally scales easily with SPUs
    - SPUs implemented with async Rust to fully leverage hardware
    - Built on Kubernetes and therefore equipped for massive scale
  - Business logic should scale (should be easy to add more services) -> move to "ease of use"
    

    Scalability is the ability of a system to gracefully handle massive
    volumes of data. Fluvio's data streaming architecture was designed
    with this in mind from the start. In Fluvio, streaming is performed
    by Streaming Processing Units (or SPUs), which are deployed
    cooperatively in a Kubernetes cluster. Naturally, each SPU has a
    particular capacity of storage, processing power, and bandwidth, so
    as the system demands increase, Fluvio scales horizontally by simply
    adding more SPUs to the mix. This process is automatically coordinated
    by the Streaming Controllers (SCs), giving the system an elastic
    behavior which can cost-effectively handle spike traffic without
    needing a long-term committment to peak capacity.

    On a more micro-level, Fluvio achieves high scalability by leveraging
    Rust's asynchronous programming model. This makes it easy to write code
    that multitasks, efficiently performing work that is ready to be done
    without wasting time on blocking work such as IO. This means that all
    of Fluvio's components are fully utilizing the available hardware,
    getting more done at a lower cost.


- Retention (how long do you want it)
  - It is desirable for data systems to retain data for years at a time
  - Most real-time systems can only retain data for minutes or hours
  - Fluvio data streams bring higher retention to real-time
    - Data streams are immutable logs
    - Messages are indexed in the order received
  - Long-lived data enables new features to be built on historical data
  - Know the state of the system at all times
  - Use cases
    - Recover persistent store
    - Rebuild in-memory database (Change Data Capture)
    - Test features with historical data
    - Retrospective analysis
    - Audit reports
  - <a href="https://en.wikipedia.org/wiki/System_of_record" target="_blank">System of Record (SOR)</a>
    

    For many applications, it can be very beneficial to retain a long-term history
    of the events that have occurred. In traditional databases, the only thing that
    is preserved is the sum of all past events - the "current" state. In contrast,
    Fluvio's data streams are implemented as immutable logs, meaning that we can
    not only view the current state, but also the entire state of the system at any
    point in time. This is incredibly powerful, as it grants us capabilities that
    otherwise wouldn't be possible. For example, we can test new product features
    using historical data (think ML use-cases), generate audit reports, perform
    retrospective analysis, and even rebuild in-memory databases.


- Resiliency (how confident do you want to be in being safe from system failure)
  - Built-in solution for replication (no need for replication service e.g. mysql)
  - Continue operating despite failures
  - Fluvio uses distributed-system tricks to be resilient
    - Data streams are replicated, so nodes may failover in case of error
    - SC coordinates all data streams, including rerouting during failures
  - Kubernetes allows quick replacement of failed nodes
  - High availability -> [Fluvio architecture](https://fluvio.io/docs/architecture)
  - Zone availability protects against data center outages
  - Fluvio supports a wide range of deployment scenarios: single cloud, multi-cloud, cloud-to-data center, and more. If Fluvio is deployed in a single availability zone, it protects against server outages. When deployed across availability zones, Fluvio protects against zone outages, and when deployed in hybrid mode it protects against data center outages. It’s up to the users to define the deployment model most suitable for their environment.
  

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
    that even a complete datacenter outage will not compromise the availability of
    Fluvio data streams.


Pillar: Composition
- Giving services context is a matter of composing relevant raw data into something that is actionable




// Pillar: Collaboration
// - Overview: what do we mean by collaboration? (live collaboration)
//   - User interfaces need to support live updates
//   - Backend needs to process frontend events and produce responses instantly
//   - Data streaming allows building microservices to interact with events
//   - Enable collaboration between services by composing data
//   - Giving services context is a matter of composing relevant raw data into something that is actionable
//   - A key thing about collaboration with data is that the primitives need to be composable
//   
// 
//     Our second pillar is all about collaboration, and in the context of real-time
//     applications, what we mean is _live_ collaboration. Live collaboration is all
//     about working together with as little delay and friction as possible - it should
//     be fast and easy. There are multiple ways to 
// 
// 
// - Interface
//   - Integrate with communication tools such as Slack or Teams
//   - Build a custom interface, e.g. dynamic web app
//   - Fluvio can drive either of these types of interface
//   - "Custom connectors" can attach to streams and produce events, listen for responses, etc.
//   - Sample programs:
//     - [Chat App](/tutorials/node/simple-chat/)
//     - [Bot Assistant](/tutorials/node/bot-assistant/)
// - Real-time Streaming Platform
//   - Data streams can allow independent services to work together
//   - Enables cross-service capabilities
//   - Fluvio offers multiple language clients for sending and receiving real-time messages
//   - Example multi-service flow:
//     - Services listen for relevant events on Fluvio topics
//     - When they receive an event, they perform an action
//     - Then, they generate a new event with the results of their action
//   - Dealership example services:
//     - Advisors: coordinate customer communication.
//     - Scheduling: look-up maintenance schedules and assign mechanics.
//     - Billing: charges customer credit cards and generates receipts.
//     1. When a customer arrives at the dealership, advisory service looks-up the car and publishes an event on the `advisory` topic.
//     2. Scheduling receives the event, schedules mechanics to perform the work, and generates an event on the `schedule` topic.
//     3. Billing receives the event, charges the customer credit card and sends an event with the amount paid on `billing` topic.
// - Business Logic Services
//   - Build services to read and write to real-time data streams
//   - Services can be polyglot since they only need to interact with the stream
//   - Edge components (user interface, embedded devices, third-party integrations) just interact with streams
// - Summary
//   - Fluvio supports both purpose-built and custom collaboration tools, letting users of both choose how to receive and manage events.
//   - Fluvio’s streaming platform lets independent services communicate through data streams/topics.
//   - Fluvio supports the creation of business logic services in common programming languages.

Pillar: Ease of Use
- Overview: Fluvio is easy to use
  - Open source with client and cluster components
  - Cluster may be run locally or in cloud
  - Declarative configuration management and reconciliation
- Clusters
  - Fluvio Cloud takes care of setup and maintenance
  - Fluvio open-source can be installed anywhere
    - Fluvio CLI can assist in cluster installation
- Clients
  - Fluvio CLI:
    - cluster management: start/delete, topic creation
    - producing and consuming messages
    - Profile management: interact with different clusters
      - Useful for dev/test/prod environments
  - Programmatic APIs:
    - Admin APIs similar to database admin:
      - Create/delete topics, manage SPUs, etc.
    - Client APIs are used for sending/receiving messages
    - Available in Rust and Node
- Operational Efficiency
  - Declarative configuration allows operator to specify target state
  - Cluster machinery will alter the system state to match the target state
  - An example target state is "Topic A should exist and have 3 partitions"
    - Fluvio will orchestrate SPUs to create the topic and distribute the partitions
  - Machinery can improve in performance and reliability over time for the same specs
  - Fluvio can reconcile cluster changes to ensure target state is still met
  - Allows system to self-heal and recover from error, e.g. a failed node
  - Example: One SPU loses power and disconnects from the cluster
    - Cluster may provision a new SPU to take its place
    - Later, the failed SPU comes online
      - Rather than blindly resuming old task, it re-syncs with system and resumes operations
- Summary
  - Fluvio clusters can be either cloud-based and automatically managed by the Fluvio team, or open-source.
  - Fluvio clients allow management of the clusters with either the Fluvio CLI or a programmatic API.
  - Fluvio ensures operational efficiency with declarative configuration management and reconciliation, making cluster management both intuitive and reliable.


Conclusion
- Modern business is happening in real-time
- Real-time applications need a foundation that is ready for the job
- Fluvio is the first purpose-built platform for real-time applications
- Get started using Fluvio Cloud
- Come talk to us on Discord

---



Sample use-cases
- Log aggregation for servers
- Command center for IOT devices
- Middle tier for microservices
- Notification services for mobile devices
- Fully-featured collaborative apps, with chat, voice, video, geo-tracking, and more.
