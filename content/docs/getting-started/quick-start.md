---
title: Quick Start Guide
menu: Quick Start
weight: 10
---

The quickest path to data streaming is with [Fluvio Cloud]({{< relref "../fluvio-cloud/cloud-platform" >}}). Every account gets a dedicated **[Fluvio Open Source]({{< relref "../fluvio-oss/oss-platform/" >}})** installation in the cloud, provisioned with 1 x [Streaming Controller]({{< relref "../architecture/sc/" >}}) (SC) and 3 x [Streaming Processing Units]({{< relref "../architecture/spu/" >}}) (SPU). 

{{< image src="quick-start.svg" alt="Fluvio - Cloud Streaming" justify="center" width="400">}}

Create an account, download Fluvio client library, and you are ready for data streaming.


##### Let's get started

* [Create a Fluvio Cloud account]({{< ref "#create-a-fluvio-cloud-account" >}})
* [Download and configure the CLI]({{< ref "#download-and-configure-the-cli" >}})
* [Create topic and stream "Hello World"]({{< ref "#create-a-topic-and-stream-hello-world">}})


## Create a Fluvio Cloud account

Each Fluvio Cloud installation is assigned a unique **security profile**. Profiles ensure only authorized clients are permitted to communicate with a specific cloud installation. Login to your **Fluvio Cloud Account** to download and install the security profile associated with your environment.

1. {{< target-blank title="SignUp" url="http://app.fluvio.io/signup" >}} for a new account.
        
    * Fill-in and submit the **New Account** form.
    * A verification message is sent to your email.

2. Check your email for the **verification** message.
        
    * Click the **Confirm Email** button.
    * The **Congratulation** page confirms your account is enabled.

3. Click **Login** to access your **Cloud Dashboard** 

    * You will receive an email when the **installation is ready**, it may take a few minutes.

4. Download your **Security Profile**

    * The **installation guide** is published on your {{< target-blank title="Fluvio Cloud Dashboard" url="http://app.fluvio.io" >}}  

All Fluvio clients, **CLI** and **API libraries**, require a **security profile** corresponding to the Fluvio installation you are looking to access. 


## Download and configure the CLI

Fluvio Command Line Interface binaries are available in macOS and Linux versions:

* Download binaries for your environment from [github release](https://github.com/infinyon/fluvio/releases).  
* Copy binary to your bin path and make it executable.

### Test CLI

Run following command to ensure Fluvio CLI is working correctly:

{{< fluvio >}}
$ fluvio --help
Fluvio Command Line Interface

fluvio <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    consume       Read messages from a topic/partition
    produce       Write messages to a topic/partition
    spu           SPU Operations
    spu-group     SPU Group Operations
    custom-spu    Custom SPU Operations
    topic         Topic operations
    advanced      Advanced operations
    help          Prints this message or the help of the given subcommand(s)
{{< /fluvio >}}

Congratulation! You have successfully deployed Fluvio.


## Create a topic and stream "Hello World"

...
