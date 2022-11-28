---
title: Build HTTP to SQL Pipeline
weight: 30
---

## Connector Pipeline

<img src="../images/create-pipeline.png"
     alt="execution flow of InfinyOn pipeline"
	 style="justify: center; max-width: 400px" />

This tutorial expects you to already have the Fluvio CLI installed, and InfinyOn
Cloud set up. If neither of these is the case, please follow the [setup tutorial]({{<ref "/docs/tutorials/cloud-setup.md">}})!

There are two main steps for this tutorial:
* Creating an Inbound HTTP Connector to collect JSON
  * Receive data without any modifications
  * JSON to JSON transformation before send to topic
* Creating an Outbound SQL Connector to insert the input JSON into a database
  * Basic insert
  * JSON to JSON transformation before insert

We will be looking at the [Inbound HTTP Connector]({{<ref "/connectors/inbound/http.md">}}) setup, and connecting
to the <a href="https://catfact.ninja" target="_blank" rel="nofollow" > catfact.ninja</a> database to ingest and store JSON data into a topic.

The Outbound connector will be using a PostgreSQL database. It will listen to the topic for new records and insert them into a table.

You can use your own PostgreSQL instance, if it can be reached over the internet. But you can still follow along by creating a PostgreSQL database at a hosting service, such as [ElephantSQL](https://www.elephantsql.com/).

## Connectors

If you wish to automatically collect information from one source and send it to
Fluvio, or send data from Fluvio to location, Connectors are the way to go. When
given the information on the interface through the Connector configuration file,
Fluvio can poll a multitude of input types.

### Connector Config Layout

This is the template YAML connector file. To make it useful, it needs to be
populated – which we will do in the next step. See
[the documentation]({{<ref "/connectors/">}}) for the parameters available for use.

{{<code file="embeds/templates/connector-template.yaml" lang="yaml" copy="true">}}

Thankfully, filling it out is simple. For any connection, you need a name,
the connection type, and what topic to connect to.

### Inbound Connector

For the HTTP-specific parameters you will need to specify the link it is
polling, and the interval at which it polls.

{{<code file="embeds/connectors/catfacts-basic-connector.yaml" lang="yaml" copy="true">}}

This creates a connector named `cat-facts`, that reads from the website
`https://catfact.ninja/fact` every 30 seconds, and produces to the topic
`cat-facts-data`.

#### Testing the Inbound Connector

You can register the connector to Fluvio with `fluvio cloud connector create --config=<config-file.yaml>`

%copy first-line%
```bash
$ fluvio cloud connector create --config=catfacts-basic-connector.yml
```                               

You can use `fluvio cloud connector list` to view the status of the connector.

%copy first-line%
```shell
 $ fluvio cloud connector list
 NAME       TYPE         VERSION  STATUS
 cat-facts  http-source  0.4.1    Running
 ```

And `fluvio consume` to view the incoming data in the topic.

%copy first-line%
```bash
$ fluvio consume cat-facts-data -dT4
Consuming records starting 4 from the end of topic 'cat-facts-data'
{"fact":"A cat lover is called an Ailurophilia (Greek: cat+lover).","length":57}
{"fact":"British cat owners spend roughly 550 million pounds yearly on cat food.","length":71}
{"fact":"Fossil records from two million years ago show evidence of jaguars.","length":67}
{"fact":"Relative to its body size, the clouded leopard has the biggest canines of all animals\u2019 canines. Its dagger-like teeth can be as long as 1.8 inches (4.5 cm).","length":156}
```

#### Inbound Connector with JSON to JSON transformation before writing to topic
All Inbound Connectors support [transformations]({{<ref "../../../docs/concepts/transformations-chain.md">}}) which are applied before the data is sent to the topic.
We can extend our config file to add an additional JSON to JSON transformation to records.

{{<code file="embeds/connectors/catfacts-basic-connector-with-transform.yaml" lang="yaml" copy="true">}}

In this config, we add the field `source` with the static value `http` to every record. Note that if the field 
already exists, it will not be overwritten.

Before we create the connector we need to add [`infinyon/jolt@0.1.0`]({{<ref "../../smartmodules/certified/jolt.md" >}}) SmartModule to the cluster.
This SmartModule uses a domain specific language (DSL) called [Jolt](https://github.com/infinyon/fluvio-jolt), to specify a transformation of input JSON to another shape of JSON data.

Let's download this SmartModule from the [SmartModule Hub]({{<ref "/smartmodules/hub/overview.md">}}).

%copy first-line%
```shell
$ fluvio hub download infinyon/jolt@0.1.0
```

Then, we create a connector just like before

%copy first-line%
```shell
$ fluvio cloud connector create --config catfacts-basic-connector-with-transform.yaml
```

And `fluvio consume` to view the transformed data in the topic.

%copy first-line%
```bash
$ fluvio consume cat-facts-data-transformed -dT4
Consuming records starting 4 from the end of topic 'cat-facts-data-transformed'
{"fact":"The Amur leopard is one of the most endangered animals in the world.","length":68,"source":"http"}
{"fact":"Some cats have survived falls of over 65 feet (20 meters), due largely to their “righting reflex.” The eyes and balance organs in the inner ear tell it where it is in space so the cat can land on its feet. Even cats without a tail have this ability.","length":249,"source":"http"}
{"fact":"In Holland’s embassy in Moscow, Russia, the staff noticed that the two Siamese cats kept meowing and clawing at the walls of the building. Their owners finally investigated, thinking they would find mice. Instead, they discovered microphones hidden by Russian spies. The cats heard the microphones when they turned on.","length":318,"source":"http"}
{"fact":"Cats can be right-pawed or left-pawed.","length":38,"source":"http"}
```

### Outbound Connector

#### Setup
For the SQL Outbound connector example, we will need to create a table in our Postgres database.

Run this query in your database before starting any Outbound connectors.

%copy%
```sql
create table animalfacts(length integer, raw_fact_json jsonb)
```

We also need to run a few commands with `fluvio` to download some prepackaged SmartModules from the SmartModule Hub to attach to the Outbound Connector.

This SmartModule will do a basic mapping of the JSON input into a SQL statement for the Outbound SQL connector

%copy first-line%
```shell
$ fluvio hub download infinyon/json-sql@0.1.0
```

If you have not added `infinyon/jolt@0.1.0` SmartModule on previous steps, we need to add it as well:

%copy first-line%
```shell
$ fluvio hub download infinyon/jolt@0.1.0
```

For more info about the SmartModule Hub, check out the [Hub Overview page]({{<ref "/smartmodules/hub/overview.md">}})

#### Outbound SQL with basic SQL inserts
In this connector, we will listen in on the `cat-facts-data` topic. Whenever a new fact is produced to the topic, the Outbound SQL connector will insert the record into a table named `animalfacts`. The length in one column called `length` and the entire JSON in another column `raw_fact_json`.

{{<code file="embeds/tutorials/data-pipeline/sql-basic.yml" lang="yaml" copy="true">}}

And we create the Outbound connector just like the Inbound connector

%copy first-line%
```shell
$ fluvio cloud connector create --config sql-basic.yml
connector "simple-cat-facts-sql" (sql-sink) created
```

After a few seconds, we can see data in the PostgreSQL table,

```txt
> select * from animalfacts;
+--------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
| length | raw_fact_json                                                                                                                                                                                                                     |
|--------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 74     | {"fact": "A cat’s jaw can’t move sideways, so a cat can’t chew large chunks of food.", "length": 74}                                                                                                                              |
| 110    | {"fact": "Unlike humans, cats are usually lefties. Studies indicate that their left paw is typically their dominant paw.", "length": 110}                                                                                         |
| 114    | {"fact": "A commemorative tower was built in Scotland for a cat named Towser, who caught nearly 30,000 mice in her lifetime.", "length": 114}                                                                                     |
| 98     | {"fact": "Statistics indicate that animal lovers in recent years have shown a preference for cats over dogs!", "length": 98}                                                                                                      |
| 78     | {"fact": "Approximately 1/3 of cat owners think their pets are able to read their minds.", "length": 78}                                                                                                                          |
| 95     | {"fact": "At 4 weeks, it is important to play with kittens so that they do not develope a fear of people.", "length": 95}                                                                                                         |
| 46     | {"fact": "Jaguars are the only big cats that don't roar.", "length": 46}                                                                                                                                                          |
| 31     | {"fact": "Female felines are \\superfecund", "length": 31}                                                                                                                                                                        |
+--------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
```


#### Outbound SQL with JSON to JSON transformation before insert
In this connector, we will listen in on the `cat-facts-data` topic.

But before we insert into the database, we specify a transformation. The resulting JSON we see inserted in the table has the `length` removed, and adds `type: cat` to every JSON.

{{<code file="embeds/tutorials/data-pipeline/sql-transform.yml" lang="yaml" copy="true">}}

Create another connector with our transformations.

%copy first-line%
```shell
$ fluvio cloud connector create --config sql-transform.yml 
connector "transform-cat-facts-sql" (sql-sink) created
```

After a few seconds, we can see data in the PostgreSQL table with our configured transformations.


```sql
> select * from animalfacts;
+--------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
| length | raw_fact_json                                                                                                                                                                                                                     |
|--------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 58     | {"fact": "A cat can spend five or more hours a day grooming himself.", "type": "cat"}                                                                                                                                             |
| 110    | {"fact": "Unlike humans, cats are usually lefties. Studies indicate that their left paw is typically their dominant paw.", "type": "cat"}                                                                                         |
| 163    | {"fact": "Retractable claws are a physical phenomenon that sets cats apart from the rest of the animal kingdom. I n the cat family, only cheetahs cannot retract their claws.", "type": "cat"}                                    |
| 78     | {"fact": "Approximately 1/3 of cat owners think their pets are able to read their minds.", "type": "cat"}                                                                                                                         |
| 145    | {"fact": "A sexually-active feral tom-cat \\owns\\\" an area of about three square miles and \\\"\"sprays\\\"\" to mark his territory with strong smelling urine.\"\"\"", "type": "cat"}                                          |
| 149    | {"fact": "It has been scientifically proven that owning cats is good for our health and can decrease the occurrence of high blood pressure and other illnesses.", "type": "cat"}                                                  |
| 73     | {"fact": "In relation to their body size, cats have the largest eyes of any mammal.", "type": "cat"}                                                                                                                              |
+--------+-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+
```


### Deleting connectors

To stop the traffic from all the connectors, you run `fluvio cloud connector delete <connector name>`.

This will delete the connector, but not the topic is was attached to

%copy%
```shell
$ fluvio cloud connector delete cat-facts cat-facts-transformed simple-cat-facts-sql transform-cat-facts-sql
connector "cat-facts" deleted
connector "cat-facts-transformed" deleted
connector "simple-cat-facts-sql" deleted
connector "transform-cat-facts-sql" deleted
```

## Conclusion

We used the Inbound HTTP Connector to ingest JSON data from an endpoint and save it in a topic. We configured 
transformation of outgoing JSON records.

There was a brief introduction to the SmartModule Hub, which enabled the Outbound SQL connector to consume data.

With the Outbound SQL Connector, we utilized SmartModules in two different ways.
1. Basic insert into a table, with a simple mapping of JSON fields into columns
2. Configured transformation of the incoming JSON before following the same mapping process


## Check out these Other Tutorials

* [Setup InfinyOn Cloud]({{<ref "/docs/tutorials/cloud-setup.md">}})
* [SmartModules with `smdk`]({{<ref "/docs/tutorials/smartmodule-development.md">}})

## References

* [Fluvio CLI Produce]({{<ref "/cli/commands/produce.md">}})
* [Fluvio CLI Consume]({{<ref "/cli/commands/consume.md">}})
* [Fluvio CLI topic]({{<ref "/cli/commands/topic.md">}})
* [Fluvio CLI profile]({{<ref "/cli/client/profile.md">}})
* [Connectors]({{<ref "/connectors/">}})
* [Smart Modules]({{<ref "/smartmodules/">}})
* [SmartModule Rust API](https://docs.rs/fluvio-smartmodule/latest/fluvio_smartmodule/)
* [Transformations]({{<ref "../../../docs/concepts/transformations-chain.md">}})
