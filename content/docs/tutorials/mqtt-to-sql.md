---
title: Build MQTT to SQL Pipeline
weight: 40
---

At the end of this tutorial, we will see data starting from an MQTT broker and ending in a PostgreSQL table.

We'll use 2 connectors:
* [Inbound MQTT connector]({{<ref "/connectors/inbound/mqtt.md" >}})
* [Outbound SQL connector]({{<ref "/connectors/outbound/sql.md" >}})
    * There will be an example of combining multiple SmartModules, known as **SmartModule chaining**

The Outbound connector will be using a PostgreSQL database. It will listen to the topic for new records and insert them into a table.

You can use your own PostgreSQL instance, if it can be reached over the internet. But you can still follow along by creating a PostgreSQL database at a hosting service, such as [ElephantSQL](https://www.elephantsql.com/).

---

1. [Setup]({{<ref "#setup" >}})
    1. [Start MQTT connector]({{<ref "#start-mqtt-connector" >}})
        * [Install `mosquito` for sending test data]({{<ref "#install-mosquito---mqtt-client" >}})
    2. [Start SQL connector(s)]({{<ref "#start-sql-connectors" >}})
        * [Connector with no transformation]({{<ref "#sql-connector-with-no-transformation" >}})
        * [Connector with extra JSON to JSON transformation]({{<ref "#connector-with-json-to-json-transformation" >}})
        * [Install `pgcli` for validating DB inserts]({{<ref "#install-pgcli---postgresql-client" >}})
2. [The actual test]({{<ref "#the-actual-test" >}})
    * [Publish JSON to MQTT broker]({{<ref "#publish-json-to-mqtt-broker" >}})
    * [View output in PostgreSQL]({{<ref "#view-output-in-postgresql" >}})
3. [Move transformation to MQTT connector]({{<ref "#move-transformation-to-mqtt-connector" >}})
4. [Conclusion]({{<ref "#conclusion" >}})


## Setup
### Start MQTT Connector

This connector expects to take `json` input from the MQTT broker, from an MQTT topic named `ag-mqtt-topic`. These parameters will be reflected in the final JSON payload that gets produced to the fluvio topic `mqtt-topic`


MQTT connector config: `mqtt.yml`

{{<code file="embeds/tutorials/mqtt-to-sql/mqtt.yml" lang="yaml" copy=true >}}

#### Create MQTT connector

%copy first-line%
```shell
$ fluvio cloud connector create --config mqtt.yml
```

#### Install `mosquito` - MQTT client

First install [mosquito](https://mosquitto.org/download/) to follow later steps for sending JSON to our test MQTT broker

{{<idea>}}
On MacOS, you can install `mosquitto` with homebrew with the following command:

%copy first-line%
```shell
$ brew install mosquitto
```
{{</idea>}}

### Start SQL connector(s)

You can start one of both of the following connectors

1. **[Connector with no transformation]({{<ref "#sql-connector-with-no-transformation" >}})**
    1. Download SmartModule for example
    2. Example connector config
    3. Start connector
2. **[Connector with extra JSON to JSON transformation]({{<ref "#connector-with-json-to-json-transformation" >}})**
    1. Download SmartModules for example
    2. Example connector config
    3. Start connector


#### SQL Connector with no transformation

##### Download `json-sql` SmartModule

Example output

%copy first-line%
```shell
$ fluvio hub download infinyon/json-sql@0.1.0
downloading infinyon/json-sql@0.1.0 to infinyon-json-sql-0.1.0.ipkg
... downloading complete
... checking package
trying connection to fluvio router.dev.infinyon.cloud:9003
... cluster smartmodule install complete
```

##### SQL Connector with no transformation config

{{<code file="embeds/tutorials/mqtt-to-sql/sql.yml" lang="yaml" copy=true >}}

Start No transformation connector SQL connector

%copy first-line%
```shell
$ fluvio cloud connector create --config sql.yml
```

#### Connector with JSON to JSON transformation

Download the `jolt` and `json-sql` SmartModules used by this example connector

Example output

%copy first-line%
```shell
$ fluvio hub download infinyon/json-sql@0.1.0
downloading infinyon/json-sql@0.1.0 to infinyon-json-sql-0.1.0.ipkg
... downloading complete
... checking package
trying connection to fluvio router.infinyon.cloud:9003
... cluster smartmodule install complete
```

%copy first-line%
```shell
$ fluvio hub download infinyon/jolt@0.1.0
downloading infinyon/jolt@0.1.0 to infinyon-jolt-0.1.0.ipkg
... downloading complete
... checking package
trying connection to fluvio router.infinyon.cloud:9003
... cluster smartmodule install complete
```

##### Connector with JSON to JSON transformation config

{{<code file="embeds/tutorials/mqtt-to-sql/sql-chain.yml" lang="yaml" copy=true >}}

Start SQL connector with JSON transformation 

%copy first-line%
```shell
$ fluvio cloud connector create --config sql-chain.yml
```

#### Install `pgcli` - PostgreSQL client

Install `pgcli` to follow the later DB validation steps
[https://www.pgcli.com](https://www.pgcli.com/)

{{<idea>}}
On MacOS, you can install `pgcli` with homebrew with the following command:

%copy first-line%
```shell
$ brew install pgcli
```
{{</idea>}}


## The actual test

📋 **Using example JSON, this is the sequence of events that will occur**

1. (user) [Publish JSON to MQTT broker]({{<ref "#publish-json-to-mqtt-broker" >}})
2. (Inbound MQTT connector) Produce data to fluvio topic `mqtt-topic`
    1. Produce a transformed JSON object with config parameter data with the name of the MQTT topic embedded
3. (Outbound SQL connector) Consume the inbound record from topic `mqtt-topic` 
    1. Apply transformations to record (JSON to JSON connector only)
    2. Insert record into DB
4. (user) [Validate JSON record in PostgreSQL database]({{<ref "#view-output-in-postgresql" >}})

{{<idea>}}
👉 If you are starting with a new database, you will need to create the table before sending messages to MQTT. It is not created automatically.

Table create query

%copy%
```sql
create table topic_message(device_id int, record jsonb);
```
{{</idea>}}

This is what our input JSON to MQTT looks like

example JSON (formatted)
{{<code file="embeds/tutorials/mqtt-to-sql/input-json-pretty.json" lang="" copy=true >}}

### Publish JSON to MQTT broker

Run the following to send a test JSON message to the demo MQTT broker with `mosquito` ([Installation steps](h{{<ref "#the-actual-test" >}}


Command:

%copy first-line%
```shell
$ mosquitto_pub -h test.mosquitto.org -t ag-mqtt-topic -m '{"device": {"device_id":17, "name":"device17"}}'
```

Produced data in topic:

%copy first-line%
```shell
$ fluvio consume mqtt-topic -B
Consuming records from the beginning of topic 'mqtt-topic'
{"mqtt_topic":"ag-mqtt-topic","payload":{"device":{"device_id":17,"name":"device17"}}}
```

Produced data in topic:
Run the following to connect to PostgreSQL DB with `pgcli` ([Installation steps]({{<ref "#install-pgcli---postgresql-client" >}}))


### View output in PostgreSQL

Use `pgcli` to examine the database.

%copy first-line%
```shell
$ pgcli -U user -h db.postgreshost.example -p 5432 dbname 
```

Check that the JSON from MQTT has been inserted into table

%copy%
```sql
select * from topic_message;
```

Example output from both connectors

```txt
+-----------+-----------------------------------------------------------------------------------------------+
| device_id | record                                                                                        |
|-----------+-----------------------------------------------------------------------------------------------|
| 17        | {"payload": {"device": {"name": "device17", "device_id": 17}}, "mqtt_topic": "ag-mqtt-topic"} |
| 17        | {"device": {"name": "device17", "type": "mobile", "device_id": 17}}                           |
+-----------+-----------------------------------------------------------------------------------------------+
SELECT 2
Time: 0.080s
```

Output explanation:

In both cases, we’ve used the device_id key in the MQTT JSON as the value in the column of the same name.
The first row is from our No Transformation connector. The record data appears unchanged from what we saw in the topic.


Resulting record

{{<code file="embeds/tutorials/mqtt-to-sql/output-json-basic.json" lang="" copy=true >}}

The second row is from our JSON to JSON transformation connector
We’ve `shifted` the topic JSON data, so it more closely resembles the original JSON.

Then we enrich the payload by adding the `.device.type` key with the value mobile before inserting into the DB

{{<code file="embeds/tutorials/mqtt-to-sql/output-json-transform.json" lang="" copy=true >}}

## Move transformation to MQTT Connector

Transformations in the `transforms` section of SQL Connector config are deliberately decoupled from connectors. 
We can move a SmartModule from an Inbound to an Outbound connector and accomplish the same result. 
The decision depends on the shape of the data you want to store in a topic.
For Inbound connectors, the data is transformed before sending to Fluvio topic, while for Outbound, it happens after the data is sent to Fluvio topic
but before it is sent to the connector.

Let's try it.

Modify our `mqtt.yml` config with one transformation that we are moving from the SQL Connector:
{{<code file="embeds/tutorials/mqtt-to-sql/mqtt-chain.yml" lang="yaml" copy=true >}}

We don’t need this transformation on SQL Connector anymore, remove it from `sql-chain.yml` file:
{{<code file="embeds/tutorials/mqtt-to-sql/sql-pre-transformed.yml" lang="yaml" copy=true >}}

We need to re-create connectors:  

%copy first-line%
```shell
$ fluvio cloud connector delete fluvio-mqtt-connector
$ fluvio cloud connector create --config mqtt.yml
```

also, we delete one now obsolete SQL connector and re-create another without the transformation that we moved to MQTT:

%copy first-line%
```shell
$ fluvio cloud connector delete fluvio-sql-connector-chain
$ fluvio cloud connector delete fluvio-sql-connector
$ fluvio cloud connector create --config sql-chain.yml
```

And now, if we execute command:

%copy first-line%
```shell
$ mosquitto_pub -h test.mosquitto.org -t ag-mqtt-topic -m '{"device": {"device_id":17, "name":"device17"}}'
```

The new record differs from what we saw previously:

%copy first-line%
```shell
$ fluvio consume mqtt-topic -B
Consuming records from the beginning of topic 'mqtt-topic'
{"mqtt_topic":"ag-mqtt-topic","payload":{"device":{"device_id":17,"name":"device17"}}}
{"device":{"device_id":17,"name":"device17","type":"mobile"}}
```

We can see that the record was transformed before producing to Fluvio cluster.

However, in the database table, the new record equals to the previous one.

```txt
+-----------+-----------------------------------------------------------------------------------------------+
| device_id | record                                                                                        |
|-----------+-----------------------------------------------------------------------------------------------|
| 17        | {"payload": {"device": {"name": "device17", "device_id": 17}}, "mqtt_topic": "ag-mqtt-topic"} |
| 17        | {"device": {"name": "device17", "type": "mobile", "device_id": 17}}                           |
| 17        | {"device": {"name": "device17", "type": "mobile", "device_id": 17}}                           |
+-----------+-----------------------------------------------------------------------------------------------+
SELECT 3
Time: 0.080s
```

Although the final result is the same (the same records will end up in SQL database with the same content), choosing the proper side
of a pipeline where transformations should reside may significantly affect performance on high volumes of data.

## Conclusion

After setting up our end-to-end MQTT to SQL scenario, we were able to send JSON data to the MQTT broker and track the data to the PostgreSQL table.

We saw the results for the JSON just being inserted into the table with the `json-sql` SmartModule.

Using SmartModule chaining with the `jolt` and `json-sql` SmartModules, we observed that the resulting JSON was successfully transformed. 

We can choose on which side of a pipeline we wanted to transform our data without material impact to the result.
