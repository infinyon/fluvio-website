```
Create a Topic with the given name

fluvio topic create [OPTIONS] [name]

Arguments:
  [name]
          The name of the Topic to create

Options:
  -p, --partitions <partitions>
          The number of Partitions to give the Topic
          
          Partitions are a way to divide the total traffic of a single Topic into separate streams which may be processed independently. Data sent to different partitions may be processed by separate SPUs on different computers. By dividing the load of a Topic evenly among partitions, you can increase the total throughput of the Topic.
          
          [default: 1]

  -r, --replication <integer>
          The number of full replicas of the Topic to keep
          
          The Replication Factor describes how many copies of the Topic's data should be kept. If the Topic has a replication factor of 2, then all of the data in the Topic must be fully stored on at least 2 separate SPUs.
          
          This applies to each Partition in the Topic. If we have 3 partitions and a replication factor of 2, then all 3 of the partitions must exist on at least 2 SPUs.
          
          [default: 1]

  -i, --ignore-rack-assignment
          Ignore racks while computing replica assignment

  -f, --replica-assignment <file.json>
          Replica assignment file

  -d, --dry-run
          Validates configuration, does not provision

      --retention-time <time>
          Retention time (round to seconds) Ex: '1h', '2d 10s', '7 days' (default)

      --segment-size <bytes>
          Segment size (by default measured in bytes) Ex: `2048`, '2 Ki', '10 MiB', `1 GB`

      --compression-type <compression>
          Compression configuration for topic

      --max-partition-size <bytes>
          Max partition size (by default measured in bytes) Ex: `2048`, '2 Ki', '10 MiB', `1 GB`

  -c, --config <PATH>
          Path to topic configuration file
          
          All configuration parameters can be specified either as command-line arguments or inside the topic configuration file in YAML format.

  -h, --help
          Print help (see a summary with '-h')
```