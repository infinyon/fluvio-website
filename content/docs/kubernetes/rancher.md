---
title: Install on Rancher
weight: 100
---

## Install Fluvio CLI

The Fluvio CLI is an all-in-one tool for setting up, interacting, and managing with Fluvio clusters. Install the Fluvio CLI by running the following command:

{{% inline-embed file="embeds/download-cli/curl-bash-copy.md" %}}

## Install Rancher Desktop

Navigate to the Rancher Desktop [installation page](https://docs.rancherdesktop.io/getting-started/installation/) and follow the instructions. Rancher Desktop will provide access to other utilities needed to run Fluvio such as `kubectl` and `helm`.

Please make sure that the container runtime is `dockerd (moby)`. That configuration can be changed in the `Kubernetes Settings` section on the sidebar.

<img src="../images/rancher-dockerd.png"
     alt="A screenshot of the Rancher Desktop using dockerd as container runtime"
     style="justify: center; max-width: 800px" />

## Start Fluvio cluster

You can start a Fluvio cluster by running `fluvio cluster start`.

%copy first-line%
```bash
$ fluvio cluster start --k8 --use-k8-port-forwarding
```

### Verify cluster is running

We can check the fluvio cluster by checking version and status with the following command:

```bash
$ fluvio version
```

## Hello, Fluvio!

Congratulations, you've successfully installed Fluvio on your local machine!

Let's use the Fluvio CLI to play with some basic functionality.

The first thing we need to do is create a [topic].

%copy first-line%
```bash
$ fluvio topic create greetings
topic "greetings" created
```

Now that we have a topic, we can [produce] some messages!

Use the following command to send a message to the `greetings` topic:

%copy first-line%
```bash
$ echo "Hello, Fluvio" | fluvio produce greetings
```

Finally, we can [consume] messages back from the topic

%copy first-line%
```bash
$ fluvio consume greetings -B -d
Consuming records from the beginning of topic 'greetings'
Hello, Fluvio
```

Way to go! You're well on your way to writing real-time distributed apps with Fluvio!

[topic]: {{< ref "/cli/cluster/topic.md" >}}
[produce]: {{< ref "/cli/client/produce.md" >}}
[consume]: {{< ref "/cli/client/consume.md" >}}

---

If you run into any problems along the way, make sure to check out our [troubleshooting]
page to find a fix.

[troubleshooting]: {{< ref "/docs/operations/troubleshoot.md" >}}
