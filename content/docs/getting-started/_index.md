---
title: Getting Started
folder: Getting Started
menu: Overview
toc: true
weight: 10
---

The quickest way to **real-time data** is with [Fluvio Cloud](/docs/fluvio-cloud). 
Each cloud account receives a dedicated `Fluvio` installation provisioned with 1 x [Streaming Controller](/docs/architecture/sc) (SC) and 3 x [Streaming Processing Units](/docs/architecture/spu) (SPU), and managed by the  <a href="https://infinyon.com" target="_blank">InfinyOn</a> team. 

{{< image src="getting-started/quick-start.svg" alt="Fluvio - Cloud Streaming" justify="center" width="500">}}

One data cloud installation can process all your real-time data streaming needs: from any number of producers, to any number of consumers, in any data formats, to any geo-location, and more. 

#### Three Steps to Data Streaming

1. [Create a Fluvio Cloud account](#create-a-fluvio-cloud-account)
2. [Download and configure the CLI](#download-and-configure-the-cli)
3. [Create topic and stream "Hello World"](#create-a-topic-and-stream-hello-world)


## Create a Fluvio Cloud account

Each Fluvio Cloud installation is assigned a unique _security profile_. Profiles ensure only authorized clients are permitted to communicate with a specific cloud installation. Login to your **Fluvio Cloud Dashboard** to download and install the security profile associated with your environment.

1. <a href="https://app.fluvio.io/signup" target="_blank">SignUp</a> for a new account.
        
    * Submit `New Account` form.
    * Check your email for the verification message.

2. Validate your email account.
        
    * Click the `Confirm Email` button.
    * _Congratulation_ page confirms your account is enabled.

3. Click `Login` to access your **Cloud Dashboard** 

    * _Installation is ready_ email confirms your environment is ready. It may take a few minutes.

4. Download your _Security Profile_

    * Your security profile is published in your <a href="http://app.fluvio.io" target="_blank">Fluvio Cloud Dashboard</a>

-> All Fluvio clients (CLI, Node API, Rust API) require a **security profile** to access a Fluvio cluster. 


## Download and configure the CLI

Fluvio Command Line Interface binaries are available for macOS and Linux versions:

1. Download binaries for your environment from [github](https://github.com/infinyon/fluvio/releases).  
2. Copy binary to your bin path and make it executable.

#### Check your installation

```bash
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
```


#### Check connection to Fluvio Cloud

Use the following CLI command to verify the **security profile** is installed correctly in your home directory:

```bash
$ fluvio spu list
ID  NAME      STATUS  TYPE     RACK  PUBLIC               PRIVATE 
0  group3-0  online  managed   -    10.105.174.231:9005  flv-spg-group3-0.flv-spg-group3:9006 
1  group3-1  online  managed   -    10.105.169.200:9005  flv-spg-group3-1.flv-spg-group3:9006 
2  group3-2  online  managed   -    10.101.143.60:9005   flv-spg-group3-2.flv-spg-group3:9006 
```

**Fluvio CLI** connects to **Fluvio Cloud SC**, which in turn queries the **SPUs** for status information. Checkout the [Architecture](/docs/architecture) section for additional information.

**Congratulations!** You have successfully deployed your **Fluvio Cloud** environment!

## Create a topic and stream "Hello World"

Next, we use the CLI to create a topic, produce, and consume your first message.

#### Create a Topic

Create a topic with 1 partition and a replication factor of 2.

```bash
$ fluvio topic create --topic my-topic --partitions 1 --replication 2 
topic 'my-topic' created successfully
```

Ensure topic has been provisioned by displaying its details.

```bash
$ fluvio topic describe --topic my-topic
 Name                    :  my-topic
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
```


#### Produce "Hello World" on my-topic

Produce "Hello word" message to _my-topic_ and partition _0_:

```bash
$ fluvio produce -t my-topic -p 0
Hello World!
Ok!
```

To produce multiple messages, use _-C, --continuous_ CLI flag.

#### Consume from my-topic

Consume messages from beginning on _my-topic_ and partition _0_:

```bash
$ fluvio consume -t my-topic -p 0 -g
Hello World!
```

To consume multiple messages, use _-C, --continuous_ CLI flag.

**Congratulations!** You have successfully streamed your first message!

#### Next Steps
----------------
* [Build a Node App](build-node-app)
* [Build a Rust App](build-rust-app)
