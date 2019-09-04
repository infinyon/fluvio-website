---
title: Send your First Message
menu: Example
weight: 60
---

This is a simple end-to-end example that describes how to provision a system and send a simple message from a Producer to a Consumer.

### Step 1: Create an SPU-Group

Set an alias for the Streaming Controller (SC) to simplify your operations:

{{< cli yaml>}}
$ alias SC="kc get svc flv-sc-public -o jsonpath='{.status.loadBalancer.ingress[0].ip}'"
{{< /cli>}}

Create an SPU-Group with 3 SPUs:

{{< cli yaml>}}
$ fluvio spu-group create --name group3 --replicas 3 --sc `SC`:9003
spu-group 'group3' created successfully
{{< /cli>}}

Ensure the 3 SPUs are online:

{{< cli yaml>}}
$ fluvio spu list  --sc `SC`:9003
ID  NAME      STATUS  TYPE     RACK  PUBLIC                                   PRIVATE 
  0  group3-0  online  managed   -    group3-0.default.svc.cluster.local:9005  group3-0.group3:9006 
  1  group3-1  online  managed   -    group3-1.default.svc.cluster.local:9005  group3-1.group3:9006 
  2  group3-2  online  managed   -    group3-2.default.svc.cluster.local:9005  group3-2.group3:9006 
{{< /cli>}}

### Step 2: Create a Topic

Create a topic with 1 partition and 3 replicas:

{{< cli yaml>}}
$ fluvio topic create --topic my-topic --partitions 1 --replication 3  --sc `SC`:9003
topic 'my-topic' created successfully
{{< /cli>}}

Ensure the topic has been provisioned:

{{< cli yaml>}}
$ fluvio topic list  --sc `SC`:9003
 NAME       TYPE      PARTITIONS  REPLICAS  IGNORE-RACK  STATUS       REASON 
 my-topic  computed      1          3           -       provisioned   
{{< /cli>}}

Describe topic to show the partition distribution:

{{< cli yaml>}}
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
{{< /cli>}}


### Step 3: Send a Message

Produce and consume a simple message.

#### Produce a Message

Produce a message to topic "my-topic" and partition 0:

{{< cli yaml>}}
$ fluvio produce --topic my-topic -p 0  --sc `SC`:9003 
hello world
Ok!
{{< /cli>}}

#### Consume a Message

Consume the message from same topic and  partition (from beginning):

{{< cli yaml>}}
$ fluvio consume  --topic my-topic -p 0 -g --output text --sc `SC`:9003 
hello world
{{< /cli>}}

### Step 4: Clean-up 

Delete the topic "my-topic" and it's associated partition:

{{< cli yaml>}}
$ fluvio topic delete  --topic my-topic  --sc `SC`:9003
topic 'my-topic' deleted successfully
{{< /cli>}}

Ensure the topic has been deleted:

{{< cli yaml>}}
$ fluvio topic list  --sc `SC`:9003
No topics found
{{< /cli>}}

Congratulation! You have successfully sent your first message!

{{< links "Related Topics" >}}
* [Install and Setup Minikube]({{< relref "minikube" >}})
* [Install Fluvio]({{< relref "install-fluvio" >}})
* [Install CLI]({{< relref "install-cli" >}})
{{< /links >}}