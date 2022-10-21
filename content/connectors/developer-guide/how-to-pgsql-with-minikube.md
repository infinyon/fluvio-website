---
title: Collect CDC events from Postgres with Fluvio
weight: 1000
hidden: true
---
## Advanced Use-Case: Using a Managed Connector with Minikube

For this example, we're going to set up Postgres and Fluvio together in Kubernetes,
then launch a managed Fluvio Postgres connector to continuously produce CDC
events from Postgres into a Fluvio topic. If you'd like to follow along, here
are the tools we'll be using:

- [minikube](https://minikube.sigs.k8s.io/docs/start/)
- [kubectl](https://kubernetes.io/docs/tasks/tools/)
- [The Fluvio CLI](https://fluvio.io/download)
- [The Postgres CLI (psql)](https://www.postgresql.org/download/)
  - **Note**: Even though we'll be launching the Postgres server in Kubernetes,
    we'll still need `psql` locally to execute commands and queries in our database.

### Launching Postgres and Fluvio in Kubernetes

The very first thing we'll want to do is start minikube. For this example, we're
going to use some extra parameters when running `minikube start` in order to set
up nodeports. This will allow us to use `psql` to connect to our Postgres instance
inside minikube.

%copy first-line%
```bash
$ minikube start --extra-config=apiserver.service-node-port-range=1024-65535
```

Once minikube is running, the next thing we'll want to do is start up a Fluvio cluster.
We can start a Fluvio cluster on our minikube instance by running the following command
from the Fluvio CLI:

%copy first-line%
```bash
$ fluvio cluster start
📝 Running pre-flight checks
     ✅ Kubernetes config is loadable
     ✅ Supported helm version is installed
     ✅ Fluvio system charts are installed
     ✅ Previous fluvio installation not found
🛠️  Installing Fluvio
     ✅ Fluvio app chart has been installed
🔎 Found SC service addr: 192.168.99.111:59461
👤 Profile set
🤖 SPU group launched (1)
     ✅ All SPUs confirmed
🎯 Successfully installed Fluvio!
```

At this point, the Fluvio CLI has installed the Fluvio cluster into our minikube
instance. We can see the Fluvio pods using `kubectl`:

```bash
$ kubectl get pods
NAME                         READY   STATUS    RESTARTS   AGE
fluvio-sc-5476656749-z4dmv   1/1     Running   0          2m
fluvio-spg-main-0            1/1     Running   0          2m
```

Next, we want to set up a Postgres database that we can watch for changes using
the Fluvio Postgres connector. For this example, we'll launch this database inside
minikube as well so that it's easy for the connector to find and communicate with
Postgres.

In order to be able to connect to the Postgres "replication stream", we need to
start up our Postgres server with some custom configurations. To make this setup
easy, I've put together some Kubernetes object definitions that take care of
everything for us. You can copy and paste the following command in order to
launch Postgres with all the required settings:

%copy%
```bash
kubectl apply -f - <<EOF
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: postgres-conf
data:
  postgres.conf: |
    listen_addresses = '*'
    wal_level = logical         # minimal, replica, or logical
    max_wal_senders = 1         # max number of walsender processes
    max_replication_slots = 1   # max number of replication slots
---
apiVersion: v1
kind: Pod
metadata:
  name: "postgres-leader"
  labels:
    app: PostgresLeader
spec:
  containers:
  - name: postgres-leader
    image: "postgres:14.0"
    args:
      - "-c"
      - "config_file=/etc/postgresql/postgres.conf"
    env:
    - name: POSTGRES_PASSWORD
      value: mysecretpassword
    volumeMounts:
    - name: postgres-conf
      mountPath: /etc/postgresql/
  volumes:
  - name: postgres-conf
    configMap:
      name: postgres-conf
      items:
      - key: postgres.conf
        path: postgres.conf
---
apiVersion: v1
kind: Service
metadata:
  name: "postgres-leader-service"
spec:
  type: NodePort
  selector:
    app: PostgresLeader
  ports:
  - name: "postgres-leader-port"
    protocol: TCP
    nodePort: 5432
    port: 5432
EOF
```

Briefly, here is what these objects are doing:

- The ConfigMap at the top is holding our custom Postgres configuration. This
  is the part that will allow us to enable logical replication in Postgres.
- The Pod definition describes how to launch Postgres and how to use the
  ConfigMap defined earlier. This also includes the superuser password for
  Postgres. Remember to always use a secure password in production and to use
  Kubernetes Secrets when doing anything more than testing.
- The Service definition at the end is used to help us communicate with the
  Postgres pod. The service name, "postgres-leader-service", will be used as
  the hostname when we specify the "Postgres URL" in our connector configuration.

After running this command, we should be able to see the Postgres pod and service
alongside the Fluvio ones:

%copy first-line%
```bash
$ kubectl get all -A
NAMESPACE     NAME                                   READY   STATUS    RESTARTS        AGE
default       pod/fluvio-sc-5476656749-z4dmv         1/1     Running   0               26m
default       pod/fluvio-spg-main-0                  1/1     Running   0               26m
default       pod/postgres-leader                    1/1     Running   0               13m
...

NAMESPACE     NAME                              TYPE        CLUSTER-IP       EXTERNAL-IP   PORT(S)                  AGE
default       service/fluvio-sc-internal        ClusterIP   10.110.228.22    <none>        9004/TCP                 26m
default       service/fluvio-sc-public          NodePort    10.98.115.226    <none>        9003:59461/TCP           26m
default       service/fluvio-spg-main           ClusterIP   None             <none>        9005/TCP,9006/TCP        26m
default       service/fluvio-spu-main-0         NodePort    10.108.146.160   <none>        9005:25331/TCP           26m
default       service/postgres-leader-service   NodePort    10.106.160.4     <none>        5432:54320/TCP           13m
...
```

Next, we'll look at a few Postgres commands we need to run in order to enable
logical replication.

### Configuring Postgres for Logical Replication

In order for our Fluvio connector to work with our Postgres database, we need to log
into the database and configure two items:

- A logical replication "slot", which represents the stream the connector reads from, and
- A "publication", which describes which tables should have their changes recorded in the stream.

To set both of these up, we'll need to log into Postgres using the `psql` command. If you've
followed this guide exactly, your database should have the superuser username "postgres" and
password "mysecretpassword". To connect to it, we'll need to specify minikube's IP address as
follows:

%copy first-line%
```bash
$ psql -h "$(minikube ip)" -U postgres
Password for user postgres: mysecretpassword
psql (14.0)
Type "help" for help.

postgres=#
```

Now that we're logged in, we have the `postgres=#` prompt. In future snippets, be sure to
only copy and paste the text AFTER the postgres prompt, or else you'll receive an error.

The next step is to create a new logical replication slot. You can give this slot any name
you like, but you'll need to remember what it was called because we need to give that name
to the Fluvio Postgres connector. For now, we'll just call the slot `fluvio`. Create the
slot using the following command:

```bash
postgres=# SELECT pg_create_logical_replication_slot('fluvio', 'pgoutput');
 pg_create_logical_replication_slot
------------------------------------
 (fluvio,0/16FACD8)
(1 row)
```

We can see that the logical replication slot was created successfully. Next, we need to
create a publication. The publication will act like a filter that chooses which tables
have their changes replicated in the slot. For this example, we'll create a publication
that captures all table changes. If you're interested in learning more about publications,
you can [read the documentation on them here].

For our purposes, we'll create the publication with the following command:

```bash
postgres=# CREATE PUBLICATION fluvio FOR ALL TABLES;
```

At this point, we've finished all the setup that we need on the Postgres side. The next
thing we'll do is set up the Fluvio Postgres connector, but I recommend you keep the
`psql` window open. We'll eventually want to come back to it in order to add some tables
and data to the database and see the activity in the connector.

### Launching the Fluvio Postgres Inbound connector

To launch our Fluvio Postgres connector, we'll be using the `fluvio connector create` command.
To use this, we first need to create a configuration file that describes the connector's
settings. The full set of options for the Fluvio Postgres connector can be found in the
[Configuration Options](#configuration-options) section of this page. Create a new file called
`connect.yml` and paste the following contents into it:

%copy%
```yml
# connect.yml
version: 0.2.0
name: fluvio-postgres
type: postgres-source
topic: postgres
parameters:
  url: postgres://postgres:mysecretpassword@postgres-leader-service:5432
  publication: fluvio
  slot: fluvio
  topic: postgres
```

In this config, we're specifying that we want to use a Fluvio topic called `postgres`, so
let's go ahead and create that now:

%copy first-line%
```bash
$ fluvio topic create postgres
```

Now, we can launch the connector with the following command:

%copy first-line%
```bash
$ fluvio connector create --config=./connect.yml
```

After running this command, we should be able to check our Kubernetes pods and see a new
`fluvio-postgres` pod:

```bash
$ kubectl get all -A
NAMESPACE     NAME                                   READY   STATUS    RESTARTS       AGE
default       pod/fluvio-postgres-589f99cf9d-xlm64   1/1     Running   0              104s
default       pod/fluvio-sc-5476656749-z4dmv         1/1     Running   0              15h
default       pod/fluvio-spg-main-0                  1/1     Running   0              15h
default       pod/postgres-leader                    1/1     Running   0              53m
...

NAMESPACE     NAME                              TYPE        CLUSTER-IP       EXTERNAL-IP   PORT(S)                  AGE
default       service/fluvio-sc-internal        ClusterIP   10.110.228.22    <none>        9004/TCP                 15h
default       service/fluvio-sc-public          NodePort    10.98.115.226    <none>        9003:59461/TCP           15h
default       service/fluvio-spg-main           ClusterIP   None             <none>        9005/TCP,9006/TCP        15h
default       service/fluvio-spu-main-0         NodePort    10.108.146.160   <none>        9005:25331/TCP           15h
default       service/postgres-leader-service   NodePort    10.104.48.19     <none>        5432:5432/TCP            53m
...
```

If we're interested in the status of the connector, we can check its logs:

```bash
$ kubectl logs -f fluvio-postgres-589f99cf9d-xlm64
2021-11-08T14:42:39.533699Z  INFO fluvio_connect_postgres::connect: Initializing PgConnector
2021-11-08T14:42:39.536438Z  INFO fluvio_connect_postgres::connect: Connected to Fluvio
2021-11-08T14:42:40.546049Z  INFO fluvio_connect_postgres::connect: No prior LSN discovered, starting PgConnector at beginning
2021-11-08T14:42:40.574974Z  INFO fluvio_connect_postgres::connect: Connected to Postgres
2021-11-08T14:42:40.580730Z  INFO fluvio_connect_postgres::connect: Producing event: {"wal_start":24095992,"wal_end":24095992,"timestamp":689697760578772,"message":{"type":"begin","final_lsn":24096624,"timestamp":689695494531563,"xid":734}}
2021-11-08T14:42:40.604243Z  INFO fluvio_connect_postgres::connect: Producing event: {"wal_start":24096728,"wal_end":24096728,"timestamp":689697760578932,"message":{"type":"commit","flags":0,"commit_lsn":24096624,"end_lsn":24096728,"timestamp":689695494531563}}
```

> **Note** You will need to use the exact name of your pod, which will probably be different.

From the logs, we can tell that the connector has started up successfully. In this case, the
connector did not find any prior messages in the `postgres` Fluvio Topic, so it has begun reading
the Postgres replication slot from the beginning. If we were to delete and recreate the connector, we
would see a slightly different message telling us that the connector is resuming from its previous
position in the Postgres replication stream.

### Testing the connector with data

At this point, our connector is up and running, and will emit new events every time a change happens
in our Postgres database! Let's check out our Topic and see what we've received so far.

```bash
$ fluvio consume postgres -B
Consuming records from the beginning of topic 'postgres'
{"wal_start":24095992,"wal_end":24095992,"timestamp":689697760578772,"message":{"type":"begin","final_lsn":24096624,"timestamp":689695494531563,"xid":734}}
{"wal_start":24096728,"wal_end":24096728,"timestamp":689697760578932,"message":{"type":"commit","flags":0,"commit_lsn":24096624,"end_lsn":24096728,"timestamp":689695494531563}}
```

As we can see, each record is a JSON-formatted object describing the event that took place. The
message types for these events are described more thoroughly in [the Data Events section].

Let's keep this consumer window open, but head on back to the `psql` window that we kept open
from before (or re-open it if you closed it). Now let's try creating a table and inserting some
data in order to see what events appear in our topic.

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

As you run these commands, you should be able to see activity in the Connector's logs,
and events should begin showing up in the Fluvio topic. If everything worked as
expected, your topic should look something like this:

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

### Manually Setting Replication and Publication with `psql`

In your `connector.yaml` args if you opt to `skip_setup`, the postgres inbound 
connector will assume you have created a logical replication slot and a
publication. You can do this via `psql`.

Now we should be able to connect to Postgres at `localhost:5432` using the `psql`
command. At this point, we need to run some one-time setup commands using `psql`.
Use the following command to open the `psql` prompt:

%copy first-line%
```bash
$ psql -h localhost -U postgres
Password for user postgres: mysecretpassword
psql (14.0)
Type "help" for help.

postgres=#
```

> For the next two commands, be sure not to copy the `postgres=#` prompt itself, only the
text that follows.

There are two setup commands we need to run. The first one creates a "logical replication slot"
which is what allows the Fluvio Postgres Connector to stream the change events from Postgres.
The second command creates what's called a "publication", which is used as a way to choose
_which_ of the tables in your database will have changes captured.

To create the logical replication slot, run the following command:

```bash
postgres=# SELECT pg_create_logical_replication_slot('fluvio', 'pgoutput');
 pg_create_logical_replication_slot
------------------------------------
 (fluvio,0/1715178)
(1 row)
```

Next, we'll create a publication that captures all tables in the database.
If you're interested in learning more about publications, you can
[read the documentation on them here].

```bash
postgres=# CREATE PUBLICATION fluvio FOR ALL TABLES;
CREATE PUBLICATION
```

Now that our Postgres database is configured properly, we can move on and
launch our Fluvio Postgres connector! I recommend leaving the `psql` window open
and continuing the next steps in a new terminal, we will be coming back to `psql`
after the connector is running.


### Summary

Congratulations! You've just learned how to launch and configure Postgres in minikube,
as well as use Fluvio's Managed Connectors to capture the activity in the database.
Be sure to check out [Data Events section] and the [Configuration Options section] for
more details on how to use the Postgres Connector and what to expect from the data stream.


[a free InfinyOn Cloud account]: https://infinyon.cloud/signup
[logical replication message format]: https://www.postgresql.org/docs/10/protocol-logicalrep-message-formats.html
[how to set up Fluvio and Postgres in minikube]: {{<ref "#advanced-use-case-using-a-managed-connector-with-minikube">}}
[read the documentation on them here]: https://www.postgresql.org/docs/10/logical-replication-publication.html
[Data Events section]: #data-events
[Configuration Options section]: #configuration-options
