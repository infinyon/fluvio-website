---
title: Overview 
weight: 10
---

## Intro

Fluvio Connector Development Kit (CDK) is a command-line tool whose primary goal is to help developers build, test, deploy SmartConnectors, and publish them to the Hub.

[Source code](https://github.com/infinyon/fluvio/tree/master/crates/cdk)

## Installation

CDK only supports connectors developed in the Rust programming language. Other languages will be added in future releases.

### Requirements

Install Rust compiler and Cargo. See [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) for installation instructions.

1. **[Install CDK]({{< ref "install" >}})**
2. [Generate a SmartConnector]({{< ref "generate" >}})
3. [Build and Test]({{< ref "build-test" >}})
4. [Start and Shutdown]({{< ref "start-shutdown" >}})
5. [List and Logs]({{< ref "list-log" >}})
6. [Publish to SmartConnector Hub]({{< ref "publish" >}})