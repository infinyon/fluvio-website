---
title: Using a Local Connector with InfinyOn Cloud
weight: 1000
hidden: true
_build:
  render: never
---
## Example Use-Case: Using a Local Connector with InfinyOn Cloud

The quickest and easiest way to get started using Fluvio is via
[a free InfinyOn Cloud account]. In this example, we'll demonstrate how to run the
Fluvio Postgres Connector locally, capture changes from a local Postgres
database, and produce those change events to InfinyOn Cloud. You can also
read about [how to set up Fluvio and Postgres in minikube] if you're interested
in the end-to-end deployment.

To follow along, you'll need to download and install the following tools:

- [Fluvio CLI](https://fluvio.io/download)
- [Postgres CLI (psql)](https://www.postgresql.org/download/)
- [docker](https://docs.docker.com/get-docker/)

### Log into Fluvio with an InfinyOn Cloud account

To use this example, you'll need to have created [a free InfinyOn Cloud account],
which will take care of setting up a Fluvio cluster for us to get started with.
Once you have created and validated your account, use the Fluvio CLI to log in
to the account you just created:

%copy first-line%
```bash
$ fluvio cloud login
Infinyon Cloud email: you@example.com
Password:
```

### Create a Postgres server using Docker

Next, we'll want to create a Postgres database that we will monitor with the
Fluvio Postgres Connector. We'll use docker as an easy way to create a database
for this example.

Our Postgres instance will require some custom configuration options in order to
enable logical replication, which the Fluvio Postgres connector requires. Create
a new file called `postgres.conf` with the following contents:

%copy%
```bash
# postgres.conf
listen_addresses = '*'
wal_level = logical         # minimal, replica, or logical
max_wal_senders = 1         # max number of walsender processes
max_replication_slots = 1   # max number of replication slots
```

Since we're going to need Postgres and the Connector to be able to talk to one
another, we can set up a docker network to allow them to communicate:

%copy first-line%
```bash
$ docker network create postgres-net
```

Now, we'll use the following docker command to create our Postgres database:

%copy first-line%
```bash
$ docker run -d --name postgres-leader --net postgres-net -p5432:5432 -v "$PWD/postgres.conf":/etc/postgresql/postgresql.conf -e POSTGRES_PASSWORD=mysecretpassword postgres -c 'config_file=/etc/postgresql/postgresql.conf'
```

Briefly, here's what this command is doing:

- Creating a docker container from the official `postgres` docker image named `postgres-leader`
- Adding the `postgres-leader` container to the `postgres-net` network
- Forwarding port `5432` on the host machine to port `5432` in the Postgres container
- Placing the `postgres.conf` file we just created into the Postgres container and telling Postgres to use it
- Setting the Postgres superuser password to `mysecretpassword`

### Launching the Fluvio Postgres Connector

Next, we'll set up another Docker container to run the Fluvio Postgres Connector.
Before starting the connector, however, we'll want to make sure that we have created
a Fluvio Topic where the connector will produce all the events to.

%copy first-line%
```bash
$ fluvio topic create postgres
```

Now, we can use the following command to launch the connector itself:

%copy first-line%
```bash
$ docker run -d --name fluvio-connect-postgres --net postgres-net -v "$HOME/.fluvio/config:/home/fluvio/.fluvio/config" infinyon/fluvio-connect-postgres-source -- --url=postgres://postgres:mysecretpassword@postgres-leader:5432 --publication=fluvio --slot=fluvio --fluvio-topic=postgres
```

This command does the following:

- Creates a `fluvio-connect-postgres` container connected to the `postgres-net` network
- Places your `~/.fluvio/config` file into the container so the connector can use it to connect to Fluvio
- Launches the connector with the required `--url`, `--publication`, `--slot`, and `--topic` arguments

After running this command, we can check the docker logs to see if the connector is working:

%copy first-line%
```bash
$ docker logs -f fluvio-connect-postgres
2021-11-08T19:11:13.952673Z  INFO fluvio_connect_postgres::connect: Initializing PgConnector
2021-11-08T19:11:14.210479Z  INFO fluvio_connect_postgres::connect: Connected to Fluvio
2021-11-08T19:11:15.462171Z  INFO fluvio_connect_postgres::connect: No prior LSN discovered, starting PgConnector at beginning
2021-11-08T19:11:15.494943Z  INFO fluvio_connect_postgres::connect: Connected to Postgres
2021-11-08T19:11:15.501300Z  INFO fluvio_connect_postgres::connect: Producing event: {"wal_start":24095704,"wal_end":24095704,"timestamp":689713875500731,"message":{"type":"begin","final_lsn":24096336,"timestamp":689713834266075,"xid":734}}
2021-11-08T19:11:15.534719Z  INFO fluvio_connect_postgres::connect: Producing event: {"wal_start":24096440,"wal_end":24096440,"timestamp":689713875500960,"message":{"type":"commit","flags":0,"commit_lsn":24096336,"end_lsn":24096440,"timestamp":689713834266075}}
```

If we leave this window open, we'll be able to see the connector at work as it detects new changes
in the database and continues producing events.

Let's open a new terminal window and see if we can observe the events appear in our Fluvio topic:

```bash
$ fluvio consume postgres -B
Consuming records from the beginning of topic 'postgres'
{"wal_start":24095704,"wal_end":24095704,"timestamp":689713875500731,"message":{"type":"begin","final_lsn":24096336,"timestamp":689713834266075,"xid":734}}
{"wal_start":24096440,"wal_end":24096440,"timestamp":689713875500960,"message":{"type":"commit","flags":0,"commit_lsn":24096336,"end_lsn":24096440,"timestamp":689713834266075}}
```

Now, we're ready to add some data to our database and watch some events!

### Adding data to Postgres

Let's switch back to our `psql` window so that we can add some data and watch our connector in action.
As we create a new table and insert some data, you should be able to see new events appear in the
connector logs as well as in the Fluvio topic.

```bash
postgres=# CREATE TABLE dogs (name VARCHAR(20), species VARCHAR(20), state VARCHAR(20), sex CHAR(1));
```

```bash
postgres=# INSERT INTO dogs VALUES ('Lucy', 'chihuahua', 'TX', 'f');
```

```bash
postgres=# INSERT INTO dogs VALUES ('Piper', 'shih-tzu', 'NY', 'f');
```

```bash
postgres=# INSERT INTO dogs VALUES ('Winnie', 'chihuahua', 'NC', 'f');
```

If we look back at our Fluvio topic, we should be able to see all the new events
from Postgres. If we look at the `insert` events, we can see each new row of data
that we just created!

%copy first-line%
```bash
$ fluvio consume postgres -B
Consuming records from the beginning of topic 'postgres'
{"wal_start":24095704,"wal_end":24095704,"timestamp":689713875500731,"message":{"type":"begin","final_lsn":24096336,"timestamp":689713834266075,"xid":734}}
{"wal_start":24096440,"wal_end":24096440,"timestamp":689713875500960,"message":{"type":"commit","flags":0,"commit_lsn":24096336,"end_lsn":24096440,"timestamp":689713834266075}}
{"wal_start":24096776,"wal_end":24096776,"timestamp":689716518781113,"message":{"type":"begin","final_lsn":24202064,"timestamp":689716518778209,"xid":735}}
{"wal_start":24202568,"wal_end":24202568,"timestamp":689716518781202,"message":{"type":"commit","flags":0,"commit_lsn":24202064,"end_lsn":24202568,"timestamp":689716518778209}}
{"wal_start":24202800,"wal_end":24202800,"timestamp":689716528858655,"message":{"type":"begin","final_lsn":24202880,"timestamp":689716528854751,"xid":736}}
{"wal_start":0,"wal_end":0,"timestamp":689716528859099,"message":{"type":"relation","rel_id":16385,"namespace":"public","name":"dogs","replica_identity":"Default","columns":[{"flags":0,"name":"name","type_id":1043,"type_modifier":24},{"flags":0,"name":"species","type_id":1043,"type_modifier":24},{"flags":0,"name":"state","type_id":1043,"type_modifier":24},{"flags":0,"name":"sex","type_id":1042,"type_modifier":5}]}}
{"wal_start":24202800,"wal_end":24202800,"timestamp":689716528859294,"message":{"type":"insert","rel_id":16385,"tuple":[{"String":"Lucy"},{"String":"chihuahua"},{"String":"TX"},{"String":"f"}]}}
{"wal_start":24202928,"wal_end":24202928,"timestamp":689716528859383,"message":{"type":"commit","flags":0,"commit_lsn":24202880,"end_lsn":24202928,"timestamp":689716528854751}}
{"wal_start":24203216,"wal_end":24203216,"timestamp":689717362549983,"message":{"type":"begin","final_lsn":24203424,"timestamp":689717362548598,"xid":737}}
{"wal_start":24203216,"wal_end":24203216,"timestamp":689717362550064,"message":{"type":"insert","rel_id":16385,"tuple":[{"String":"Piper"},{"String":"shih-tzu"},{"String":"NY"},{"String":"f"}]}}
{"wal_start":24203472,"wal_end":24203472,"timestamp":689717362550109,"message":{"type":"commit","flags":0,"commit_lsn":24203424,"end_lsn":24203472,"timestamp":689717362548598}}
{"wal_start":24203528,"wal_end":24203528,"timestamp":689717372850185,"message":{"type":"begin","final_lsn":24203608,"timestamp":689717372848903,"xid":738}}
{"wal_start":24203528,"wal_end":24203528,"timestamp":689717372850275,"message":{"type":"insert","rel_id":16385,"tuple":[{"String":"Winnie"},{"String":"chihuahua"},{"String":"NC"},{"String":"f"}]}}
{"wal_start":24203656,"wal_end":24203656,"timestamp":689717372850406,"message":{"type":"commit","flags":0,"commit_lsn":24203608,"end_lsn":24203656,"timestamp":689717372848903}}
```

### Summary

Congratulations! At this point you have successfully set up the Fluvio Postgres Connector
using InfinyOn Cloud! In this example, we covered:

- how to log into InfinyOn Cloud with the Fluvio CLI
- how to start and configure a Postgres database in Docker
- how to launch the Fluvio Postgres connector in Docker, and
- how to view the events generated by the Connector in a Fluvio topic

Read on to the next example to learn how to set up a Managed Connector using
Kubernetes in minikube!