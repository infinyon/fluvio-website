# Fluvio Website

Fluvio website repository stores all documentation published in [fluvio.io](https://fluvio.io).

## Run Website on Local Machine

The website is generated using [Hugo Framework](https://gohugo.io/). To run the website on your local machine:

1. [Install Hugo](https://gohugo.io/getting-started/installing/)
2. Run Hugo
    ```
   ./hugo-start.sh
    ```
3. Website is rendered at
    ```
    http://localhost:1313/
    ```

4. Checkout [WRITERS.md](./WRITERS.md) for formatting and Fluvio customizations.

Hugo watches for file changes and automatically updates website.


## Public/Nightly Websites

[Netlify](https://www.netlify.com/) watches the following branches and automatically updates websites:

* `stable` updates [fluvio.io](https://fluvio.io)
* `master` updates [nightly.fluvio.io](https://nightly.fluvio.io)

## Connector-beta reference docs

A preview of the `fluvio-cms` CLI can be used to update the reference docs for connectors in the Hub.

Connectors are organized in`scripts/fluvio-cms/src/connectors/mod.rs`, and define:
- Protocol used by connector
- Direction of the data flow with respect to a Fluvio cluster.
- Location in Hugo repo where connector README is stored 
- Location in Hugo repo where the content template is stored

At the time of this writing, reference docs are downloaded from either the public github repo, or from a local clone, but this will transition to the Connector package data.

From public repo:
- Inbound HTTP
- Inbound MQTT

From local disk:
- Inbound Kafka (*)
- Outbound Kafka (*)
- Outbound SQL

(*) Kafka docs were manually separated from a single file into multiple

### Add new connector docs

2 areas need to be updated before `fluvio-cms` will support a new connector's docs

1. For new protocols, add to the `DataService` struct.
2. Depending on the connector's data directionality, add an entry to the `INBOUND` or `OUTBOUND` hashmap 

### Update existing connector docs

Downloading from the public git repo:

Ex. Download MQTT docs from github repo
```shell
$ cargo run -- connector --service mqtt --direction inbound
```

Using README from local git clone:

Ex. Copy SQL docs
```shell
$ cargo run -- connector --service sql --direction outbound --file /path/to/README
```