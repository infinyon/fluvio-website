---
title: Key Features
hidden: true
---

A subset of key features are as follows:

* **Declarative Management**: Fluvio allows operators to declare desired state and the system will do the rest. No resource available, no worries, the objects are shown `in progress` until the resource constraints are resolved.

* **Ultra Low Latency**: Fluvio takes advantage of Rust's async runtime to improve system performance and achieve low latency. It utilizes all available cores and interacts directly with hardware I/O. Unlike other streaming platforms where garbage collection adds unpredictable latency, Fluvio latency is constant and predictable.

* **Low Memory Footprint**: Fluvio can be deployed in as little as 100Mb of memory. The deployment is highly optimized machine code and it does not require an intermediary virtual machine. Fluvio can be run on devices such as the Raspberry Pi and is suitable for IOT devices running at the edge.

* **Built-in Retention**: Fluvio uses long-lived immutable storage to persist data streams. Each data stream is copied to multiple servers and retained for hours or years. Fluvio clients have built-in failure detection and switch-over mechanisms to minimize downtime. Fluvio's retention mechanism allows clients to come and go without concerns over data loss.

* **Guaranteed Message Ordering**: Fluvio guarantees partition-level message ordering. Messages are stored and forwarded to consumers in the order they are received from the producers. Order guarantee is a critical requirement in many stateful systems.

* **Cloud Native by Design**: CNCF defines “Cloud Native” as an approach that utilizes cloud computing to build and run scalable applications in modern, dynamic environments. Cloud Native products are centered around Kubernetes, often called the “kernel of the cloud”. Fluvio is designed to work natively with Kubernetes. It uses Helm charts for installation, CRDs for provisioning, and Operators to interact with the KV store.

* **Developer Friendly**: Fluvio offers native language bindings in many modern programming languages such as Rust and Node. Future release will gradually introduce additional languages such as Swift/Objective-C, Python, and Go.

* **Powerful CLI**: Fluvio offers a powerful CLI that controls virtually all aspects of the system: installation, provisioning, data streaming, and monitoring. In addition, the CLI can manage any number of installations whether they are on your local system, in your data center, or in the public cloud.
