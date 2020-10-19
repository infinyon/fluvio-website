---
title: Convert MySQL table changes into events streams (Rust)
toc: true
---

In this tutorial, we're going to be talking about **Change Data Capture** (or CDC).
We'll give a brief overview of what CDC is, then we'll talk about the component
steps in a CDC setup. Then, we'll take a look at how we can implement our own
CDC system using Fluvio.

Roughly speaking, CDC is a way to record updates to a database in a structured way,
so that those updates may be replayed on another database instance to create a
perfect copy. We typically refer to the database being copied as the "leader", and
any database which is being replicated from it as a "follower". Note that there can
be only one leader, but potentially many followers.

In this CDC tutorial, we'll be using a [Rust MySQL Binlog] library to watch for
updates to our leader database. When we see an update, we'll send it as a message
on a Fluvio stream. We'll call the program that does this the "Producer", since
it "produces" messages to the Fluvio topic. We'll also create a second program
that watches for messages to appear on our Fluvio topic and then write those
messages to the binlog of the follower database. This program will be called the
"Consumer", since it "consumes" messages from Fluvio.

To get the most out of this tutorial, be sure to clone our [CDC demo repo] and follow
along with the setup and commands.

[Rust MySQL Binlog]: https://github.com/EasyPost/rust-mysql-binlog
[CDC demo repo]: https://github.com/infinyon/fluvio-demo-apps-rust

## Requirements

This tutorial requires you to have completed the following prerequisites:

- Install [Docker]
- Install the `mysql` client for [Mac] or Linux ([deb] or [rpm])
- Complete the [Fluvio getting started guide]
  - Note: You may use [Fluvio Cloud] or install [Fluvio locally], but you don't need both

[Docker]: https://docs.docker.com/engine/install/
[Mac]: https://formulae.brew.sh/formula/mysql-client
[deb]: https://dev.mysql.com/doc/mysql-apt-repo-quick-guide/en/
[rpm]: https://dev.mysql.com/doc/mysql-linuxunix-excerpt/5.6/en/linux-installation-yum-repo.html
[Fluvio getting started guide]: https://fluvio.io/docs/getting-started
[Fluvio Cloud]: https://fluvio.io/docs/getting-started/fluvio-cloud/
[Fluvio locally]: https://fluvio.io/docs/getting-started/fluvio-local/

## Docker MySQL deployment

This example requires two instances of MySQL to be up and running: One to be a
leader that produces events, and the other to be a follower that consumes events.
We'll use docker images to help set up these MySQL instances with the right
configurations.

Start a Docker container for Mysql Producer:

```bash
$ ./docker/install.sh -n mysql-producer -d ~/mysql-cdc/mysql-producer -p 3080
./docker/install.sh -n mysql-producer -d ~/mysql-cdc/mysql-producer -p 3080
 ✅ mkdir -p /Users/aj/mysql-cdc/mysql-producer - ok
 ✅ docker build . -t mysql-80 - ok
f110fc2c66ec599f95fd7b799dd1ea5a20a8abfa5772bee1505dc90af9f0bcb4
 ✅ docker mysql-producer - running
```

Start another container for Mysql Consumer:

```bash
$ ./docker/install.sh -n mysql-consumer -d ~/mysql-cdc/mysql-consumer -p 3090
 ✅ mkdir -p /Users/aj/mysql-cdc/mysql-consumer - ok
 ✅ docker build . -t mysql-80 - ok
af1c2fa69a06ee922f6f4302655a07721e3a01379e8c750f70e42c6b2e09f943
 ✅ docker mysql-consumer - running
 ```


At this point, you should be able to see both docker containers running. The leader
database is connected to port `3080`, and the follower is connected to `3090` on your
host machine.

```bash
$ docker ps
CONTAINER ID   IMAGE      COMMAND                  CREATED       STATUS       PORTS                               NAMES
17c60cbbfc09   mysql-80   "docker-entrypoint.s…"   3 hours ago   Up 3 hours   33060/tcp, 0.0.0.0:3090->3306/tcp   mysql-consumer
f5ec27a58476   mysql-80   "docker-entrypoint.s…"   3 hours ago   Up 3 hours   33060/tcp, 0.0.0.0:3080->3306/tcp   mysql-producer
```

## Creating a Fluvio Topic

In Fluvio, every message is sent to a Topic. A topic is a sort of category for events that
are related. For this example, we're going to create a topic that will receive all of our
MySQL events. Run the following command to create the topic:

```bash
$ fluvio topic create rust-mysql-cdc
```

## Start the Producer and Consumer

Now we'll launch the CDC Producer, which will watch for any SQL commands that are executed
in the leader MySQL instance and produce corresponding Fluvio events.

```bash
cdc-mysql$ cargo run --bin cdc-producer -- ./producer_profile.toml
```

In another terminal window, we'll launch the CDC Consumer, which listens for new Fluvio
events and replicates them in the follower MySQL instance.

```bash
cdc-mysql$ cargo run --bin cdc-consumer -- ./consumer_profile.toml
```

## Connect to Mysql

Now you're ready to start interacting with your databases. In two separate terminal
windows, we'll open up the `mysql` command line. In one of those windows, we'll start
running queries on the leader database, and in the other window, we'll connect to the
follower database to see that all the changes get propagated.

### Connect to Producer (leader DB)

The leader database is bound to port 3080. You can connect to it using the `mysql`
command as follows:

```bash
$ mysql -h 0.0.0.0 -P 3080 -ufluvio -pfluvio4cdc!
...
mysql >
```

The producer profile has a filter that registers only changes applied to the "flvDb"
database. Let's create the database now:

```bash
mysql> CREATE DATABASE flvDb;
Query OK, 1 row affected (0.01 sec)

mysql> use flvDb;
Database changed
```

### Connect to Consumer (follower DB)

The follower database is bound to port 3090. In the second terminal window, connect
to the follower using the `mysql` command:

```bash
$ mysql -h 0.0.0.0 -P 3090 -ufluvio -pfluvio4cdc!
...
mysql >
```

Since the CDC Consumer is already running, the "flvDb" database we just created on
the leader will also be created on the follower!

```bash
mysql> SHOW DATABASES;
+--------------------+
| Database           |
+--------------------+
| flvDb              |
| information_schema |
+--------------------+
2 rows in set (0.00 sec)

mysql> use flvDb;
Database changed
```

## MYSQL Test Commands

In the producer terminal, generate mysql commands and see then propagated to the consumer database;

### Producer

```bash
mysql> CREATE TABLE pet (name VARCHAR(20), owner VARCHAR(20), species VARCHAR(20), sex CHAR(1), birth DATE);
Query OK, 0 rows affected (0.03 sec)

mysql> show tables;
+-----------------+
| Tables_in_flbDb |
+-----------------+
| pet             |
+-----------------+
1 row in set (0.00 sec)
```

### Consumer

Create table has been propagated to the consumer:

```bash
mysql> show tables;
+-----------------+
| Tables_in_flbDb |
+-----------------+
| pet             |
+-----------------+
1 row in set (0.00 sec)
```

### Producer - Other Commands

Let's add some rows and modify fields in `pet` table:

```
INSERT INTO pet VALUES ('Puffball','Diane','hamster','f','1999-03-30');
INSERT INTO pet VALUES ('Jack','Peter','dog','m','1999-03-30');
UPDATE pet SET birth = '1989-08-31' WHERE name = 'Jack';
ALTER TABLE pet ADD COLUMN last_vaccine DATE;
DELETE from pet where name='Puffball';
INSERT INTO pet VALUES ('Spot', 'Jane', 'dog', 'm', '2010-11-2', Null);
UPDATE pet SET last_vaccine='2020-6-10' WHERE name='Spot';

mysql> select * from pet;
+------+-------+---------+------+------------+--------------+
| name | owner | species | sex  | birth      | last_vaccine |
+------+-------+---------+------+------------+--------------+
| Jack | Peter | dog     | m    | 1989-08-31 | NULL         |
| Spot | Jane  | dog     | m    | 2010-11-02 | 2020-06-10   |
+------+-------+---------+------+------------+--------------+
```


### Consumer - Other Commands

The commands have been propagated to the consumer:

```
mysql> select * from pet;
+------+-------+---------+------+------------+--------------+
| name | owner | species | sex  | birth      | last_vaccine |
+------+-------+---------+------+------------+--------------+
| Jack | Peter | dog     | m    | 1989-08-31 | NULL         |
| Spot | Jane  | dog     | m    | 2010-11-02 | 2020-06-10   |
+------+-------+---------+------+------------+--------------+
```


## Fluvio Events

CDC producer generates the events on the **Fluvio topic** as defined in the producer profile. By default the topic name is **rust-mysql-cdc**. 

To view the events generated by the CDC producer, start run **fluvio consumer** command:

```
$ fluvio consume rust-mysql-cdc -B
{"uri":"flv://mysql-srv1/flvDb","sequence":0,"bn_file":{"fileName":"binlog.000003","offset":233},"operation":{"Query":"CREATE DATABASE flvDb"}}
{"uri":"flv://mysql-srv1/flvdb","sequence":1,"bn_file":{"fileName":"binlog.000003","offset":423},"operation":{"Query":"CREATE TABLE pet (name VARCHAR(20), owner VARCHAR(20), species VARCHAR(20), sex CHAR(1), birth DATE)"}}
{"uri":"flv://mysql-srv1/flvdb/pet","sequence":2,"bn_file":{"fileName":"binlog.000003","offset":832},"columns":["name","owner","species","sex","birth"],"operation":{"Add":{"rows":[{"cols":[{"String":"Puffball"},{"String":"Diane"},{"String":"hamster"},{"String":"f"},{"Date":{"year":1999,"month":3,"day":30}}]}]}}}
{"uri":"flv://mysql-srv1/flvdb/pet","sequence":3,"bn_file":{"fileName":"binlog.000003","offset":1146},"columns":["name","owner","species","sex","birth"],"operation":{"Add":{"rows":[{"cols":[{"String":"Jack"},{"String":"Peter"},{"String":"dog"},{"String":"m"},{"Date":{"year":1999,"month":3,"day":30}}]}]}}}
{"uri":"flv://mysql-srv1/flvdb/pet","sequence":4,"bn_file":{"fileName":"binlog.000003","offset":1461},"columns":["name","owner","species","sex","birth"],"operation":{"Update":{"rows":[{"before_cols":[{"String":"Jack"},{"String":"Peter"},{"String":"dog"},{"String":"m"},{"Date":{"year":1999,"month":3,"day":30}}],"after_cols":[{"String":"Jack"},{"String":"Peter"},{"String":"dog"},{"String":"m"},{"Date":{"year":1989,"month":8,"day":31}}]}]}}}
{"uri":"flv://mysql-srv1/flvdb","sequence":5,"bn_file":{"fileName":"binlog.000003","offset":1647},"operation":{"Query":"ALTER TABLE pet ADD COLUMN last_vaccine DATE"}}
{"uri":"flv://mysql-srv1/flvdb/pet","sequence":6,"bn_file":{"fileName":"binlog.000003","offset":2001},"columns":["name","owner","species","sex","birth","last_vaccine"],"operation":{"Delete":{"rows":[{"cols":[{"String":"Puffball"},{"String":"Diane"},{"String":"hamster"},{"String":"f"},{"Date":{"year":1999,"month":3,"day":30}},"Null"]}]}}}
{"uri":"flv://mysql-srv1/flvdb/pet","sequence":7,"bn_file":{"fileName":"binlog.000003","offset":2316},"columns":["name","owner","species","sex","birth","last_vaccine"],"operation":{"Add":{"rows":[{"cols":[{"String":"Spot"},{"String":"Jane"},{"String":"dog"},{"String":"m"},{"Date":{"year":2010,"month":11,"day":2}},"Null"]}]}}}
{"uri":"flv://mysql-srv1/flvdb/pet","sequence":8,"bn_file":{"fileName":"binlog.000003","offset":2631},"columns":["name","owner","species","sex","birth","last_vaccine"],"operation":{"Update":{"rows":[{"before_cols":[{"String":"Spot"},{"String":"Jane"},{"String":"dog"},{"String":"m"},{"Date":{"year":2010,"month":11,"day":2}},"Null"],"after_cols":[{"String":"Spot"},{"String":"Jane"},{"String":"dog"},{"String":"m"},{"Date":{"year":2010,"month":11,"day":2}},{"Date":{"year":2020,"month":6,"day":10}}]}]}}}
```

## Other MYSQL Commands

For additional mysql commands, checkout [MYSQL-COMMANDS](https://github.com/infinyon/fluvio-demo-apps-rust/blob/master/cdc-mysql/MYSQL_COMMANDS.md)