---
title: Cloud Connectors 
weight: 30
---

Connectors with [InfinyOn Cloud] is the best option for those who want to manage their data pipelines in one place.

Configuring connectors works the same way with InfinyOn Cloud, as it does locally.

You can create and maintain your connectors in a more streamlined way through the CLI with [`fluvio cloud`]().

## Create your first connector on InfinyOn Cloud

This guide will walk you through creating an Inbound HTTP connector to ingest json data from and HTTP endpoint.

To follow this guide you will need to sign up for [InfinyOn Cloud] and log into the CLI.

%copy%
```bash
fluvio cloud login
```

{{<idea>}}
Check out the [`fluvio cloud` CLI docs]() for more details about logging into the CLI. 
{{</idea>}}

### Example HTTP connector
This is the config file for the [Inbound HTTP connector]({{<ref "/connectors-old/inbound/http.md">}}) in this guide.

{{<code file="embeds/connectors/catfacts-basic-connector.yaml" lang="yaml" copy=true >}}

In this config, we are creating a connector named `cat-facts`. It will request data from {{<link "https://catfact.ninja" "a cat fact API">}} once every 30 seconds and receive json data. The connector will store the json into a topic called `cat-facts-data`


#### Start a connector

You can create a connector by using `fluvio cloud connector create` with the example connector.

%copy first-line%
```bash
$ fluvio cloud connector create --config catfacts-basic-connector.yml 
connector "cat-facts" (http-source) created
```
#### List all connectors

After the connector is created, you can list the connectors you've created, and view their current status.

%copy first-line%
```bash
$ fluvio cloud connector list
 NAME       TYPE         VERSION  CDK  STATUS
 cat-facts  http-source  0.1.0    V3   Running
```

#### Look at connector logs

If there is a need to debug the behavior of a connector, the logs are available by running `fluvio cloud connector logs cat-facts`

%copy first-line%
```bash
$ fluvio cloud connector logs cat-facts
connector-startup infinyon/http-source@0.1.0
2023-03-25T03:41:29.570294Z  INFO surf::middleware::logger::native: sending request    
2023-03-25T03:41:29.702213Z  INFO surf::middleware::logger::native: request completed    
2023-03-25T03:41:29.702258Z  INFO connector_startup::startup: downloading package url="https://hub.infinyon.cloud/hub/v0/connector/pkg/infinyon/http-source/0.1.0"
2023-03-25T03:41:29.702290Z  INFO surf::middleware::logger::native: sending request    
2023-03-25T03:41:29.993001Z  INFO surf::middleware::logger::native: request completed    
2023-03-25T03:41:30.108220Z  INFO connector_startup::startup: writing file file="connector.ipkg"
... checking package
2023-03-25T03:41:30.301199Z  INFO connector_startup::startup: connector binary from package path="./http-source"
2023-03-25T03:41:30.301224Z  INFO connector_startup::startup: Starting deployment
Connector runs with process id: 15
2023-03-25T03:41:30.303333Z  INFO http_source: Reading config file from: /home/fluvio/config.yaml
2023-03-25T03:41:30.303526Z  INFO http_source: starting processing
2023-03-25T03:41:30.304337Z  INFO fluvio::config::tls: Using verified TLS with certificates from paths domain="odd-butterfly-0dea7a035980a4679d0704f654e1a14e.c.cloud-dev.fluvio.io"
2023-03-25T03:41:30.308822Z  INFO fluvio::fluvio: Connecting to Fluvio cluster fluvio_crate_version="0.16.0" fluvio_git_hash="8d4023ee0dc7735aaa0c823dd2b235662112f090"
2023-03-25T03:41:30.369634Z  INFO connect: fluvio_socket::versioned: connect to socket add=fluvio-sc-public:9003
2023-03-25T03:41:30.412895Z  INFO connect:connect_with_config: fluvio::config::tls: Using verified TLS with certificates from paths domain="odd-butterfly-0dea7a035980a4679d0704f654e1a14e.c.cloud-dev.fluvio.io"
2023-03-25T03:41:30.473242Z  INFO connect:connect_with_config:connect: fluvio_socket::versioned: connect to socket add=fluvio-sc-public:9003
2023-03-25T03:41:30.582726Z  INFO dispatcher_loop{self=MultiplexDisp(12)}: fluvio_socket::multiplexing: multiplexer terminated
2023-03-25T03:41:30.632722Z  INFO fluvio_connector_common::monitoring: using metric path: /fluvio_metrics/connector.sock
2023-03-25T03:41:30.632795Z  INFO fluvio_connector_common::monitoring: monitoring started
2023-03-25T03:41:31.172075Z  INFO run:create_serial_socket_from_leader{leader_id=0}:connect_to_leader{leader=0}:connect: fluvio_socket::versioned: connect to socket add=fluvio-spu-main-0.acct-ce0c1782-ca61-4c54-a08c-3ba985524553.svc.cluster.local:9005
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

When you want to stop the connector, you can delete it with `fluvio cloud connector cat-facts`

%copy first-line%
```shell
$ fluvio cloud connector delete cat-facts
connector "cat-facts" deleted
```

Deleting your connector will not delete the topic used by the connector. If you want to delete the topic, you can run `fluvio topic delete cat-facts`

%copy first-line%
```shell
$ fluvio topic delete cat-facts
topic "cat-facts" deleted
```

### Conclusion

We created a basic Inbound HTTP connector, looked at the logs for the connector, and viewed the HTTP response data in the Fluvio topic. Lastly, we deleted the connector and topic.

You are ready to create your own connectors! Check out the docs for our supported Inbound and Outbound connectors to get started with your own data sources.


[InfinyOn Cloud]: https://infinyon.cloud
