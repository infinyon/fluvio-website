---
title: Running examples
menu: Example
weight: 50
---


### Creating SPU

First set alias for SC controller
{{< cli yaml>}}
alias SC="kc get svc flv-sc-public -o jsonpath='{.status.loadBalancer.ingress[0].ip}'"
{{< /cli>}}

Then create a set of 3 SPUs using CLI.

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

### Creating a Topic

Let's create a topic with 1 partition and 3 replicas:

{{< cli yaml>}}
$ fluvio topic create --topic my-topic --partitions 1 --replication 3  --sc `SC`:9003
topic 'my-topic' created successfully
{{< /cli>}}

List topics:
{{< cli yaml>}}
$ fluvio topic list  --sc `SC`:9003
 NAME       TYPE      PARTITIONS  REPLICAS  IGNORE-RACK  STATUS       REASON 
 my-topic  computed      1          3           -       provisioned   
{{< /cli>}}

And show partition:
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


### Produce and consume a simple message to/from topic

Produce a message to partition 0 of my-topic:
{{< cli yaml>}}
$ fluvio produce --topic my-topic -p 0  --sc `SC`:9003 
hello world
Ok!
{{< /cli>}}

Consume the message from same partition (from beginning)
{{< cli yaml>}}
$ fluvio consume  --topic my-topic -p 0 -g --output text --sc `SC`:9003 
hello world
{{< /cli>}}

### Deleting the topic

{{< cli yaml>}}
$ fluvio topic delete  --topic my-topic  --sc `SC`:9003
topic 'my-topic' deleted successfully
{{< /cli>}}

Ensure the topic has been deleted:

{{< cli yaml>}}
$ fluvio topic list  --sc `SC`:9003
No topics found
{{< /cli>}}

Congratulation! You have successfully deployed Fluvio.

#### Next Steps
* [Deploy SPU Group]({{< relref "../tasks/deploy-spu-groups" >}})
* [Create Topics]({{< relref "../tasks/create-topics" >}})
