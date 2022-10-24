---
title: Local Connectors
weight: 20
---

Local connectors are provided as Docker containers. Using our `connector-run` utility supports starting connectors in Kubernetes or Docker using a connector config file.

{{<caution>}}
Running local connectors is an advanced topic.<br>
Currently our `connector-run` utility requires a [Rust development environment](https://www.rust-lang.org/learn/get-started)
to build the CLI, and Docker or Kubernetes.
{{</caution>}}

## Create your first connector locally

This guide will walk you through creating an Inbound HTTP connector to ingest json data from and HTTP endpoint.

You will need a local Fluvio cluster for the connector to send data to.

%copy%
```shell
$ fluvio cluster start
```

For more details about starting a cluster, [see `fluvio cluster` CLI docs]({{<ref "/cli/local/cluster.md">}}).

And you will need a Rust environment with a copy of our [`fluvio-connectors` repo](https://github.com/infinyon/fluvio-connectors.git)

%copy%
```shell
$ git clone https://github.com/infinyon/fluvio-connectors.git
```

### Example HTTP connector
This is the config file for the [Inbound HTTP connector]({{<ref "/connectors/inbound/http.md">}}) in this guide.

{{<code file="code-blocks/yaml/catfacts-basic-connector.yaml" lang="yaml" copy=true >}}

In this config, we are creating a connector named `cat-facts`. It will request data from {{<link "https://catfact.ninja" "a cat fact API">}} once every 30 seconds and receive json data. The connector will store the json into a topic called `cat-facts-data`


#### Start a connector

You can start a connector in Kubernetes by running 

%copy%
```shell
$ cargo run --bin connector-run -- apply --config /path/to/catfacts-basic-connector.yaml
```

#### List all connectors

To list all connectors in Kubernetes, run `kubectl get pods --selector app=fluvio-connector`

%copy%
```shell
$ kubectl get pods --selector app=fluvio-connector
```

#### Look at connector logs

To view the logs of your connectors in Kubernetes, run `kubectl logs -l app=fluvio-connector`

%copy%
```shell
$ kubectl logs -l app=fluvio-connector
```
#### View data in topic

The HTTP connector should be receiving data and storing it in a topic with the name we specified.

%copy first-line%
```shell
$ fluvio topic list
  NAME            TYPE      PARTITIONS  REPLICAS  RETENTION TIME  COMPRESSION  STATUS                   REASON
  cat-facts-data  computed  1           1         7days           any          resolution::provisioned
```

To verify, you can consume from the topic with the `fluvio consume` CLI.

We are using the `-B` option to start from the beginning offset of the topic. Once you reach the end of the topic, you can see new data as it is sent to the topic. To exit this live view, press `Ctrl+C`.

{{<idea>}}
Using the `--disable-continuous` flag with `fluvio consume` will exit the stream once the last record has printed to screen
{{</idea>}}

```shell
$ fluvio consume cat-facts-data -B
{"fact":"Female felines are \\superfecund","length":31}
{"fact":"Cats only sweat through their paws and nowhere else on their body","length":65}
{"fact":"While many parts of Europe and North America consider the black cat a sign of bad luck, in Britain and Australia, black cats are considered lucky.","length":146}
^C
```

#### Delete a connector

When you want to stop the connector in Kubernetes, run

```shell
$ cargo run --bin connector-run -- delete --config /path/to/catfacts-basic-connector.yaml
```

Deleting your connector will not delete the topic used by the connector. If you want to delete the topic, you can run `fluvio topic delete cat-facts-data`

%copy first-line%
```shell
$ fluvio topic delete cat-facts-data
topic "cat-facts-data" deleted
```

### Conclusion

And that's the end of the guide.

Using `connector-run`, and `kubectl` we were able to manage our connectors in Kubernetes.

We created a basic Inbound HTTP connector, looked at the logs for the connector, viewed the HTTP response data in the Fluvio topic, and lastly we deleted the connector and topic.

You are ready to create your own connectors! Check out the docs for our supported Inbound and Outbound connector docs try to connect to your own data source.  


