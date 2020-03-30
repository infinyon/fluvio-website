---
title: Build a Node App
weight: 20
---

In this guide we’ll cover: how to set up your {{< target-blank title="Node.js" url="https://nodejs.org" >}} environment, and how to build a simple data streaming App.

## Install Node.js

Node.js installation varies depending on your operating system.

{{< api-table >}}
|   Operating System     |         Instructions           |
|---|---|
| MacOS      | Use the official installer from {{< target-blank title="Node.js" url="https://nodejs.org" >}} to install on **macOS**.  |
| Windows    | Use the official installer from {{< target-blank title="Node.js" url="https://nodejs.org" >}} to install on **Windows**. |
| Linux     | Use the instructions provided by your **Linux** package manager. <br/> Node.js maintains a list of supported packages {{< target-blank title="here" url="https://nodejs.org/en/download/package-manager" >}}.  |
{{< /api-table >}}


## Build a simple data streaming App

In this section we'll provide a step-by-step on how to build a simple data streaming app. If you'd like to download the app instead, skip ahead to [Download Fluvio data streaming App]({{< relref "#download-fluvio-data-streaming-app" >}}).


#### Start a new Node project

1. Create a directory

{{< code >}}
$ mkdir fluvio-app
$ cd fluvio-app
{{< /code >}}

2. Create a new node project to create a new package.json file for the app

{{< code json >}}
$ npm init -y
Wrote to /Users/user/fluvio-app/package.json:

{
  "name": "fluvio-app",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [],
  "author": "fluvio <user@fluvio.io> (fluvio.io)",
  "license": "Apache 2.0"
}
{{< /code >}}



## Download Fluvio data streaming App

The app is available for download on {{< target-blank title="github" url="https://github.com/infinyon/node-demo-apps" >}}. 


Set an alias for the Streaming Controller (SC) to simplify your CLI setup:

#### For Minikube
{{< fluvio >}}
$ alias SC="kubectl get svc flv-sc-public -o jsonpath='{.status.loadBalancer.ingress[0].ip}'"
{{< /fluvio >}}

#### For AWS EKS
{{< fluvio >}}
$ alias SC="kubectl get svc flv-sc-public -o jsonpath='{.status.loadBalancer.ingress[0].hostname}'"
{{< /fluvio >}}

{{< caution >}}
It may take many seconds for AWS external load balancer to propagate hostname for service endpoint after creating of new SPU.   If CLI fails to look up SPU, please retry after a minute or two. 
{{< /caution >}}

### Step 2: Create an SPU-Group



Create an SPU-Group with 3 SPUs:

{{< fluvio >}}
$ fluvio spu-group create --name group3 --replicas 3 --sc `SC`:9003
spu-group 'group3' created successfully
{{< /fluvio >}}

Ensure the 3 SPUs are online:

{{< fluvio >}}
$ fluvio spu list  --sc `SC`:9003
ID  NAME      STATUS  TYPE     RACK  PUBLIC                                   PRIVATE 
  0  group3-0  online  managed   -    group3-0.default.svc.cluster.local:9005  group3-0.group3:9006 
  1  group3-1  online  managed   -    group3-1.default.svc.cluster.local:9005  group3-1.group3:9006 
  2  group3-2  online  managed   -    group3-2.default.svc.cluster.local:9005  group3-2.group3:9006 
{{< /fluvio >}}

### Step 3: Create a Topic

Create a topic with 1 partition and 3 replicas:

{{< fluvio >}}
$ fluvio topic create --topic my-topic --partitions 1 --replication 3  --sc `SC`:9003
topic 'my-topic' created successfully
{{< /fluvio >}}

Ensure the topic has been provisioned:

{{< fluvio >}}
$ fluvio topic list  --sc `SC`:9003
 NAME       TYPE      PARTITIONS  REPLICAS  IGNORE-RACK  STATUS       REASON 
 my-topic  computed      1          3           -       provisioned   
{{< /fluvio >}}

Describe topic to show the partition distribution:

{{< fluvio >}}
$ fluvio topic describe --sc `SC`:9003
 Name                    :  my-topic 
 Type                    :  computed 
 Partition Count         :  1 
 Replication Factor      :  3 
 Ignore Rack Assignment  :  - 
 Status                  :  provisioned 
 Reason                  :  - 
 Partition Map               
 -----------------           
     ID      LEADER      REPLICAS       LIVE-REPLICAS 
      0        1         [1, 2, 0]      [0, 2] 
{{< /fluvio >}}


### Step 4: Send a Message

Produce and consume a simple message.

#### Produce a Message

Produce a message to topic "my-topic" and partition 0:

{{< fluvio >}}
$ fluvio produce --topic my-topic -p 0  --sc `SC`:9003 
hello world
Ok!
{{< /fluvio >}}

#### Consume a Message

Consume the message from same topic and  partition (from beginning):

{{< fluvio >}}
$ fluvio consume  --topic my-topic -p 0 -g --output text --sc `SC`:9003 
hello world
{{< /fluvio >}}

### Step 5: Clean-up 

Delete the topic "my-topic" and it's associated partition:

{{< fluvio >}}
$ fluvio topic delete  --topic my-topic  --sc `SC`:9003
topic 'my-topic' deleted successfully
{{< /fluvio >}}

Ensure the topic has been deleted:

{{< fluvio >}}
$ fluvio topic list  --sc `SC`:9003
No topics found
{{< /fluvio >}}

Congratulation! You have successfully sent your first message!

{{< links "Related Topics" >}}
* [Install Fluvio]({{< relref "install-fluvio" >}})
* [Install CLI]({{< relref "install-cli" >}})
{{< /links >}}