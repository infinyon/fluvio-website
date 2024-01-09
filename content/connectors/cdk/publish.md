---
title: Publish to Connector Hub
menu: Publish to Hub
weight: 80
---

This section assumes `my-connector` project has been [generated]({{< ref "generate" >}}).

Connector Hub is a public repository for connectors. You can publish your connector as `private` to use on different computers or `pubic` to share it with the community.

### Publish Connector to Hub

Use `cdk publish` to publish your connector to the Hub. If run without arguments, it will pack everything needed into a package and push the package to the Hub. 

%copy first-line%
```bash
$ cdk publish
```

The connector is now available for download from the Hub.


### Show Hub Connectors

Run `cdk hub list` to list connectors in the Hub.

%copy first-line%
```bash
$ cdk hub list
  CONNECTOR                          Visibility 
  infinyon-labs/graphite-sink@0.1.2  public     
  infinyon/duckdb-sink@0.1.0         public     
  infinyon/http-sink@0.2.6           public     
  infinyon/http-source@0.3.0         public     
  infinyon/kafka-sink@0.2.7          public     
  infinyon/kafka-source@0.2.5        public     
  infinyon/mqtt-source@0.2.5         public     
  infinyon/sql-sink@0.3.3            public   
  acme/my-connector0.1.0             private
```

You will see all `public` connectors and your own `private` connectors.


### Download from Hub & Run

You can download and run any connector from the Hub. In this example, we'll create new directory to download and run `acme/my-connector0.1.0` connector:


%copy first-line%
```bash
$ mkdir test-conn; cd test-conn
```

Download the connector:

%copy first-line%
```bash
$ cdk hub download acme/my-connector0.1.0
```

Use connector `.ipkg` package file and run it with the `--ipkg` option:

%copy first-line%
```bash
$ cdk deploy start --ipkg acme/my-connector0.1.0.ipkg --config ../sample-config.yaml
```

This command assumes that the sample config is the parent directory.


#### Run an Official Connector

You can use the same step-by-step to download and run an official connecgtor. Check the connector documetation to ensure the configuration file is correct.


## Steps

1. [Generate a Connector]({{< ref "generate" >}})
2. [Build and Test]({{< ref "build-test" >}})
3. [Start and Shutdown]({{< ref "start-shutdown" >}})
4. [Troubleshooting]({{< ref "troubleshooting" >}})
5. [Secrets]({{< ref "secrets" >}})
6. **[Publish to Connector Hub]({{< ref "publish" >}})**
7. [Use Examples in Github]({{< ref "github-examples" >}})
