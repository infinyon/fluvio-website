---
title: Running a Fluvio cluster with Kubernetes
menu: Kubernetes
weight: 30
---
## Start Fluvio cluster 

You can start a Fluvio cluster by running `fluvio cluster start`.

If this is your first time starting a cluster in your session, be prepared to enter your password.

%copy first-line%
```bash
$ fluvio cluster start
✅ ok: Kubernetes config is loadable
✅ ok: Supported helm version is installed
[sudo] password for user:
✅ ok: Fixed: Minikube tunnel not found
✅ ok: Fluvio system charts are installed
✅ ok: Previous fluvio installation not found

Waiting up to 120 seconds for Fluvio cluster version check...
Successfully installed Fluvio!
```

{{< idea >}}
`minikube tunnel` is used to expose Fluvio's [SC]({{< ref "/docs/architecture/sc.md" >}}) and [SPU]({{< ref "/docs/architecture/spu.md" >}}) services using the Kubernetes service `LoadBalancer` type.

You can avoid entering your password during `fluvio cluster start` by manually opening the minikube tunnel in the background.

%copy first-line%
```bash
$ sudo true && (sudo nohup minikube tunnel >/tmp/tunnel.out 2>/tmp/tunnel.out &)
```
{{< /idea >}}

### Verify cluster is running

We can verify that our Fluvio components are running with `kubectl`

All Fluvio pods in the `default` namespace should be running.

%copy first-line%

```bash
$ kubectl get po
NAME                         READY   STATUS    RESTARTS   AGE
fluvio-sc-695cfb4cf5-89lss   1/1     Running   0          15s
fluvio-spg-main-0            1/1     Running   0          9s
```

Fluvio uses several services, but we should see that the `LoadBalancer` services have an external IP.

%copy first-line%

```bash
$ kubectl get svc
NAME                 TYPE           CLUSTER-IP       EXTERNAL-IP     PORT(S)             AGE
fluvio-sc-internal   ClusterIP      10.103.221.222   <none>          9004/TCP            45s
fluvio-sc-public     LoadBalancer   10.108.59.158    10.108.59.158   9003:30682/TCP      45s
fluvio-spg-main      ClusterIP      None             <none>          9005/TCP,9006/TCP   36s
fluvio-spu-main-0    LoadBalancer   10.109.119.81    10.109.119.81   9005:32297/TCP      36s
kubernetes           ClusterIP      10.96.0.1        <none>          443/TCP             114s
```

You can check that everything worked by listing out the cluster's SPUs:

The public endpoint of the SPU should match the External IP and port from the `kubectl` output.

%copy first-line%
```bash
$ fluvio cluster spu list
 ID  NAME    STATUS  TYPE       RACK  PUBLIC              PRIVATE
  0  main-0  Online  "managed"   -    10.109.119.81:9005  fluvio-spg-main-0.fluvio-spg-main:9006
```

## Hello, Fluvio!

Congratulations, you've successfully installed Fluvio on your local machine! 

Let's use the Fluvio CLI to play with some basic functionality.

The first thing we need to do is create a [topic]({{< ref "/cli/commands/topic.md" >}}).

%copy first-line%
```bash
$ fluvio topic create greetings
topic "greetings" created
```

Now that we have a topic, we can [produce]({{< ref "/cli/commands/produce.md" >}}) some messages!

Use the following command to send a message to the `greetings` topic:

%copy first-line%
```bash
$ echo "Hello, Fluvio" | fluvio produce greetings
Ok!
```

Finally, we can [consume]({{< ref "/cli/commands/consume.md" >}}) messages back from the topic

%copy first-line%
```bash
$ fluvio consume greetings -B -d
Hello, Fluvio
```

Way to go! You're well on your way to writing real-time distributed apps with Fluvio!

Next, check out our [Tutorials page] to see real-world examples of Fluvio in action.

[Tutorials page]: https://www.infinyon.com/tutorials 

#### Related Topics
----------------

- ["Hello World" in Java](https://www.infinyon.com/tutorials/java/hello-world/)
- ["Hello World" in Node.js](https://www.infinyon.com/tutorials/node/hello-world/)
- ["Hello World" in Python](https://www.infinyon.com/tutorials/python/hello-world/)
- ["Hello World" in Rust](https://www.infinyon.com/tutorials/rust/hello-world/)