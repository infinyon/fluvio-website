---
title: Why we built Fluvio
author: 
    name: "The Fluvio Team"
description: Fluvio, a real-time data streaming platform for connected apps
date: 2020-10-27
slug: about-fluvio
url: /blog/2020/10/Why-we-built-Fluvio
---

![Fluvio](/blog/images/fluvio-social.jpg)

There is a growing need for collaboration apps where businesses communicate with customers and employees in real-time. This new class of apps accelerates decisions, improves customer experience, and opens new revenue opportunities.

We found that developers are struggling to build real-time collaboration apps that connect users with business context. Business context is derived from data that is often fragmented and owned by different teams in an organization. Real time data is a way of bringing data to users as fast as possible. Collaboration is about users working together towards a common goal. A real-time collaboration app makes data available to users at the time of communication to accelerate the speed of decisions.

There are a number of vertical collaboration products and platforms such as Zoom, Slack, and Teams that are ready-to-use communication platforms. These solutions, while great communication tools, do not account for business context. Developers wishing to build real-time collaboration apps must therefore either build plugins for these communication platforms or build their own real-time collaboration infrastructure. Plugin frameworks are often not rich enough to provide adequate access to the business context, and real-time collaboration infrastructure requires significant upfront investment of time and expertise, and most organizations do not have the resources to maintain and operate such a complex platform.

We believe there is an opportunity for a purpose built platform for real-time collaboration. A platform that is easy to operate, requires no upfront investment, and allows developers to join data, build business logic, and shorten the time required to roll-out real-time collaboration apps.

## Business Faces an Existential Challenge

The pandemic, while temporary, fundamentally changed the way businesses operate. Employees need to stay connected while working from home, customers demand solutions with immediate feedback and minimal physical interactions, and partners need real-time notification for deliveries, supply chain management or other coordinated activities.

It is no longer viable to maintain antiquated tools and processes that require continuous manual intervention and legacy communication channels to run day-to-day business activities. As the number of manual interventions increases, the time to resolution becomes unpredictable and may range from minutes to hours or even days. In modern organizations where systems are connected in real-time and activities are automatically propagated, the time to resolution is predictable and measured in milliseconds to seconds. 

### A Car Dealership Example

Imagine you pull up to a car dealership for an oil change appointment. The parking attendant greets you, places a number on your windshield and walks inside to inform the service adviser. A few minutes later the service adviser walks-up and takes you to his office to check your service records. The advisor is the master coordinator, he calls up the service department to ensure the parts are available, checks the system to see if your service is covered by warranty, etc. In most cases you’ll be promised a call later in the day. Next, you need to arrange your transportation with the loaner office. There is a long line, you wait your turn to find out they are out of cars, so you take an Uber instead.. A few hours later you receive a call from the service adviser and you are informed that the car must receive a mandatory service recall and the part is in backorder scheduled to arrive in a couple of days. What was supposed to be a short maintenance appointment turned into a multi-day ordeal.

This example is not meant to show car dealerships in a negative light, but rather to provide context on the shortcomings of businesses that use manual processes and primitive communication channels to interact with employees, vendors and customers. 

### A Vision for a Modern Car Dealership

Now imagine you pull up to a modern car dealership for an oil change appointment. A camera reads your license plate, matches your car to the VIN number and notifies the backend platform of your arrival. The platform looks up your service records, the warranty information, and mandatory recalls. It ensures that all parts are available, generates an invoice, checks the loaner location and sends you a welcome message. The message has the cost of service, the loaner location and the estimated pick-up time. You step out of the car, enter the loaner, and drive off. You are in and out of the car dealership in minutes.

This type of transformation to a modern business can be highly disruptive and many businesses may see it as an impossible undertaking. We believe proper planning and the right technology can make this transition possible for any organization.

## Transition to a Modern Business

In the article “Reimagining the post-pandemic organization”, McKinsey astutely observed that many organizations are trapped behind antiquated data systems where decisions are driven through manual processes instead of technology. Modern businesses must be able to collaborate with customers, partners and employees in real-time through a variety of communication channels, organizations boundaries, time zones, and legal jurisdictions. This transition requires new infrastructure that swaps out manual processes with collaboration centric real-time workflows. Infrastructure to link human actions with systems, to facilitate the transition from legacy systems with new technology.

## Introducing Fluvio, a Data Platform for Real-Time Collaboration

Fluvio is a high performance distributed platform that coordinates all real-time data exchanges in an organization. It deploys in minutes, operates hands-free, and covers a broad range of use cases. The platform is suitable for infrastructure services such as: log aggregation for servers, command center for IOT devices, middle tier for microservices, notification services for mobile devices; as well as fully-featured collaborative apps, with chat, voice, video, geo-tracking, and more.

The Fluvio data platform for real-time collaboration was built on these three pillars:

* Data Streams
* Collaboration
* Ease of Use

### Streaming Data

Streaming Data is data that is generated and delivered continuously. A data stream often consists of individual messages that describe events that have occurred. A key advantage of streaming data as opposed to the periodic delivery of "batches" of data is that the average time to take action on an event is much lower. Since streaming data is delivered immediately, events may also be processed and acted upon immediately. Therefore, a critical prerequisite to building real-time collaborative applications is having a scalable, durable, low-latency data streaming platform. We built Fluvio to meet this need.

Fluvio data streaming platform is built for speed, scale, retention, and resilience.

#### Low Latency

Fluvio takes advantage of all system cores and takes advantage of hardware IO interfaces to achieve low millisecond latency. This is a game changer for businesses that differentiate on speed, such as financial services, gaming and augmented reality companies, or service providers offering 5G services.

#### Horizontal Scale

As organizations grow and new services are deployed, the volume of real-time data is continuously on the rise. Fluvio is designed for parallelism and horizontal scale. At the core of this design are Streaming Processing Units (SPUs). A Fluvio cluster can handle hundreds of SPUs, where each SPU can serve thousands of consumers. This makes the platform capable of handling real-time data streaming needs from a small one-person prototype to the most demanding multi-team, multi-app, globally distributed environments.

Fluvio allows you to scale-up your cluster gradually rather than provisioning for peak. As the demand for real-time streaming grows, you can simply add SPUs and Fluvio will do the rest. 

Horizontal scale ensures you don’t need new real-time infrastructure to deploy new services or ask your IT department to work overtime when you roll out a marketing campaign that drives a high number of clients to your service.

#### Retention

The term retention is often found in the context of storage systems, where it defines the time period the data should be persisted. Organizations may have data retention policies that range from 5 to 10 years. In the context of current real-time data systems, retention periods are more often measured in seconds to minutes.

Fluvio extends the retention period for real-time data streams from seconds to years. Each data stream is an immutable store, where the messages are written in the order they are received. A long lived immutable store sets the foundation for a series of new features. The data streams can become the authoritative data source (aka. system of record (SOR)) for all events generated by different systems in the organization. These events can be played back anytime in the future to recover a persistent store, rebuild an in-memory database, play back historical data for new features, perform retrospective analysis, run audit reports, and more. 

For example, suppose a dealership stored all appointments in a database that was corrupted during a power outage. The last backup was performed the week earlier and 7 days of data had been lost. Since the database changes were also captured as events in a data stream, these events were played back and all appointments were recovered.

The translation of database operation into events is also referred to as change data capture (CDC). For additional information, check out our Fluvio CDC tutorial.

#### Resiliency

Resiliency is the ability to continue operating in the event of a failure. A platform responsible for real-time data services must be highly resilient. 

Fluvio data streams are replicated for durability and can survive multiple points of failure. Failures may result from system crashes, network outages, or other unknown reasons. Fluvio ensures that data streams remain available with minimal interruption for subsequent processing. In the event of a component failure, the cluster redistributes workloads and temporarily removes the failed component from the quorum.  Upon recovery, the cluster ensures the component is fully synchronized before it is allowed to re-join the quorum. For additional information on high availability, check out the Fluvio architecture.

Fluvio supports a wide range of deployment scenarios: single cloud, multi-cloud, cloud-to-data center, and more. If Fluvio is deployed in a single availability zone, it protects against server outages, when deployed across availability zones, it protects against zone outages, when deployed in hybrid mode it protects against data center outages. It is up to the users to define the deployment model most suitable for their environment.

### Collaboration

At 10,000 feet, a real-time collaboration app has 3 components: a collaboration interface, a real time streaming platform, and one or more business logic services. The collaboration interface allows users to communicate with each other and trigger an action. The real time streaming platform passes the action to one or more services, and returns a result. The services translate the request into a response based on the organization’s business logic.

#### Collaboration Interface

The design of the collaboration interface depends on the organization's needs. Some organizations prefer to integrate with purpose-built communication tools such as Slack or Teams. Others prefer to build custom interfaces where it can control the workflow end-to-end.

For purpose built tools, Fluvio exposes a programmable interface that allows the organization to create a custom connector. Connectors can be configured to produce actions and listen for responses, then publish the result. For organizations that prefer to roll-out their own collaboration interface, Fluvio’s programmable interface can serve as a single point of contact for the application to send and receive events. Fluvio has a sample Chat App to help developers bootstrap their first data streaming interface. 

#### Real Time Streaming Platform

A powerful use-case for a real-time data streaming platform is to connect services that typically operate independently and enable cross-service capabilities. Fluvio offers language native programmable interfaces that each service can utilize to send or receive messages to and from real-time data streams. In Fluvio, data streams are called topics. When multiple services need to communicate with each other, they connect through topics. 

For example, a user triggers an action on `topic1`, where it is picked up by one or more services. Each service may generate a result sent on `topic2`. Then service2 picks-up the action from `topic-2` and generates its own result on `topic-3`. Finally the user response service receives the message on `topic-3` and it returns the answer to the user. 

#### Business Logic Services

When organizations roll out their own real-time collaborative apps, they implicitly build services to implement the business logic.

Fluvio programmable interfaces are built for polyglot applications, where developers have the freedom to use their preferred programming languages to implement a service. Fluvio develops and maintains language native APIs for Rust, Node and Swift. Upcoming releases will roll-out additional native language APIs for Java, Python, Go, and common languages.

### Ease of Use

Fluvio is an open source distributed system with two core components: the cluster running streaming services, and the clients that communicate with the cluster. The clusters are available in two flavors: Fluvio Cloud and Fluvio Open Source. They are managed by Fluvio clients through the CLI or programmatic APIs.

#### Fluvio Cloud

Fluvio Cloud eliminates the need to install and operate a cluster. The installation, upgrading, and all other maintenance tasks are managed by our team. 

#### Fluvio Open Source
Fluvio clusters can be installed in private data centers, public clouds, or personal machines. Fluvio CLI has built-in pre-check tests and installation scripts for most common environments. To install Fluvio, first download Fluvio CLI from Github, then run the installer. 

#### Fluvio CLI
Fluvio CLI covers a broad range of operations: cluster installation, data stream provisioning, produce/consume operations and more. Checkout the CLI Guide for a detailed list of commands.

Fluvio CLI has built-in multi-cluster support, where the CLI can switch from one environment to another with one command. This feature is particularly useful when testing client software against different data sets. Developers can build the code against a local data set, then switch to a cloud instance to run the code against production data.

#### Programmatic APIs

Fluvio has programmatic APIs and role-based access for all operations. The operations are divided in two groups: admin and user. Operations such as cluster installation and topic management are reserved for administrators, whereas producer/consumer are open to users.

API interfaces are currently available in Node, Rust and Swift, while other languages will be implemented in upcoming releases. Checkout the API references for additional information.

#### Operational Efficiency

Fluvio uses declarative configuration management to simplify manageability and improve operational efficiency. 

With declarative configuration, the operator specifies the intended outcome, rather than how to accomplish it. For example a  topic with 3 partitions is a desired outcome. Given this outcome, Fluvio automatically distributes the topics across the SPUs to achieve optimal performance and reliability.

Anytime there are changes in the cluster, Fluvio uses reconciliation to ensure all outcomes can still be met. This allows the system to self-heal and recover from any conditions automatically without human intervention. For example an SPU loses power and it disconnects from the cluster in the middle of receiving a batch of records. A few minutes later, the SPU comes back online and the reconciliation loop kicks-in.  During reconciliation, the SPU is brought back in sync with the cluster before it resumes its operations.

## Conclusion

Modern business is happening in real-time. Customers demand immediate results, and business leaders require operational insight approaching clairvoyance. The transition from collaboration to real-time collaboration requires a foundation that can provide the data for business context in real-time.

Fluvio is the first purpose-built platform for real-time collaboration. With Fluvio, the focus is on business data. We bring context first, then apply collaboration features to enable real-time decisions. Our product is in early infancy and we look forward to working with you, the community, in providing us feedback as we iterate through the next wave of collaborative features.

Join us, create an account in our Fluvio Cloud, checkout our open source project in Github or connect with us in Discord.