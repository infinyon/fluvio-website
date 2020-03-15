---
title: Quick Start Guide
menu: Quick Start
weight: 10
---

The quickest path to data streaming is with [Fluvio Cloud]({{< relref "../fluvio-cloud/cloud-platform" >}}). Every account gets a dedicated **[Fluvio Open Source]({{< relref "../fluvio-oss/oss-platform/" >}})** installation in the cloud, provisioned with 1 x [Streaming Controller]({{< relref "../architecture/sc/" >}}) (SC) and 3 x [Streaming Processing Units]({{< relref "../architecture/spu/" >}}) (SPU). 

{{< image src="quick-start.svg" alt="Fluvio - Cloud Streaming" justify="center" width="500">}}

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

All Fluvio clients, **CLI** and **API libraries**, require a **security profile** corresponding your Fluvio installation. 


## Download and configure the CLI

Fluvio Command Line Interface binaries are available for macOS and Linux versions:

1. Download binaries for your environment from [github](https://github.com/infinyon/fluvio/releases).  
2. Copy binary to your bin path and make it executable.

#### Check your installation

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


#### Check connection to Fluvio Cloud

Use the following CLI command to verify the **security profile** is installed correctly in your home directory:

{{< fluvio >}}
$ fluvio spu list
ID  NAME      STATUS  TYPE     RACK  PUBLIC               PRIVATE 
0  group3-0  online  managed   -    10.105.174.231:9005  flv-spg-group3-0.flv-spg-group3:9006 
1  group3-1  online  managed   -    10.105.169.200:9005  flv-spg-group3-1.flv-spg-group3:9006 
2  group3-2  online  managed   -    10.101.143.60:9005   flv-spg-group3-2.flv-spg-group3:9006 
{{< /fluvio >}}

**Fluvio CLI** connects to **Fluvio Cloud SC**, which in turn queries the **SPUs** for status information. Checkout the [Architecture]({{< relref "../architecture/overview" >}}) section for additional information.

**Congratulations!** You have successfully deployed your **Fluvio Cloud** environment!

## Create a topic and stream "Hello World"

Next, we use the CLI to create a topic, produce, and consume our first message.

#### Create a Topic

Create a _topic-1_ with 1 partition and a replication factor of 2.

{{< fluvio >}}
$ fluvio topic create --topic topic-1 --partitions 1 --replication 2 
topic 'topic-1' created successfully
{{< /fluvio >}}

Display topic replica distribution

{{< fluvio >}}
$ fluvio topic describe --topic topic-1
 Name                    :  topic-1
 Type                    :  computed 
 Partition Count         :  1 
 Replication Factor      :  2 
 Ignore Rack Assignment  :  - 
 Status                  :  provisioned 
 Reason                  :  - 
 Partition Map               
 -----------------           
     ID      LEADER      REPLICAS         LIVE-REPLICAS 
      0        0         [0, 1]           [0, 1] 
{{< /fluvio >}}


#### Produce "Hello World" on Topic-1

Produce "Hello word" message to _topic-1_ and partition _0_:

{{< fluvio >}}
$ fluvio produce -t topic-1 -p 0
Hello World!
Ok!
{{< /fluvio >}}

Produce additional messages with _-C, --continuous_ CLI flag.

#### Consume from Topic-1

Consume messages from beginning on _topic-1_ and partition _0_:

{{< fluvio >}}
$ fluvio consume -t topic-1 -p 0 -g
Hello World!
{{< /fluvio >}}

Consume continuously with _-C, --continuous_ CLI flag.

**Congratulations!** You have successfully streamed your fist message!

{{< links "Next Steps" >}}
* [Build a Node App]({{< relref "build-node-app" >}})
* [Build a Rust App]({{< relref "build-rust-app" >}})
{{< /links >}} 

