---
title: "This Week in Fluvio #25"
date: 2022-03-10
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## New Release - Fluvio v0.9.21

### Producer compression

Producers can now compress data before sending it to the Fluvio cluster, which will save time and disk space.

Let's show it in action. We have an example data file with 1 million csv records (38MB)

```shell
# Snippet from random.csv
[...]
equally,wish,distance,collect,our,author
avoid,again,crew,become,author,spend
quite,blanket,carefully,property,experiment,beauty
wash,usual,stronger,met,original,want
shine,needs,balloon,result,moon,leaf
[...]
```

```shell
$ wc random.csv 
  999999  1000000 39623138 random.csv

$ ls -alh random.csv 
-rw-rw-r-- 1 fluvio fluvio 38M Mar 14 21:00 random.csv
```

#### Compression in action

Producers support 3 compression algorithms: `gzip`, `snappy` and `lz4`.

Now we will produce our data into separate topics and measure the time it takes to complete.

The execution times will be listed below the example commands.

##### No compression test 

%copy first-line%
```shell
$ fluvio topic create uncompressed
```

%copy first-line%
```shell
$ time fluvio produce -f random.csv uncompressed
```

%copy first-line%
```shell
$ time fluvio consume uncompressed -B -d > /dev/null
```

##### Gzip compression test 

%copy first-line%
```shell
$ fluvio topic create compressed-gzip
```

%copy first-line%
```shell
$ time fluvio produce -f random.csv --compression gzip compressed-gzip
```

%copy first-line%
```shell
$ time fluvio consume compressed-gzip -B -d > /dev/null
```

##### Snappy compression test 

%copy first-line%
```shell
$ fluvio topic create compressed-snappy
```

%copy first-line%
```shell
$ time fluvio produce -f random.csv --compression snappy compressed-snappy
```

%copy first-line%
```shell
$ time fluvio consume compressed-snappy -B -d > /dev/null
```

##### LZ4 compression test 

%copy first-line%
```shell
$ fluvio topic create compressed-lz4
```

%copy first-line%
```shell
$ time fluvio produce -f random.csv --compression lz4 compressed-lz4
```

%copy first-line%
```shell
$ time fluvio consume compressed-lz4 > /dev/null
```

#### Producer + Consumer compression time results

We can see that time taken to produce the data has a noticeable reduction using compression.

There is a slight time penalty for consuming compressed data, but a relatively shorter cost than producing.


| Compression algorithm | Time to produce 1M records | Time to consume 1M records |
|-----------------------|----------------------------|----------------------------|
| None                  | 6.530s                     | 1.300s                     |
| gzip                  | 7.294s                     | 1.674s                     |
| snappy                | 5.476s                     | 1.546s                     |
| lz4                   | 6.121s                     | 1.415s                     |


For more compression usage info, [check out the CLI producer docs]({{<ref "/cli/commands/produce#example-5-producing-using-a-compression-algorithm-gzip">}})

We'll look at the effect the compression has on disk usage next.

### Partition sizes

The disk space used by partitions is now available to view with `fluvio partition list`.

Continuing with the compression example, we'll view disk usage of our sample data for the various compression algorithms.

```shell
$ fluvio partition list
 TOPIC              PARTITION  LEADER  REPLICAS  RESOLUTION  SIZE     HW       LEO      LRS  FOLLOWER OFFSETS 
 compressed-gzip    0          0       []        Online      21.4 MB  1000000  1000000  0    [] 
 compressed-lz4     0          0       []        Online      34.7 MB  1000000  1000000  0    [] 
 compressed-snappy  0          0       []        Online      32.9 MB  1000000  1000000  0    [] 
 uncompressed       0          0       []        Online      44.5 MB  1000000  1000000  0    [] 
```

The `uncompressed` topic uses more disk space than any of the other topics, as expected. Followed in disk usage by `lz4`, `snappy` and finally `gzip`, which compressed our sample data slightly more than 50%.

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions