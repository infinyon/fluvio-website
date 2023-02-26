```
Create a Topic with the given name
fluvio topic create [OPTIONS] <name>
Arguments:
  <name>  The name of the Topic to create
Options:
  -p, --partitions <partitions>         The number of Partitions to give the Topic [default: 1]
  -r, --replication <integer>           The number of full replicas of the Topic to keep [default: 1]
  -i, --ignore-rack-assignment          Ignore racks while computing replica assignment
  -f, --replica-assignment <file.json>  Replica assignment file
  -d, --dry-run                         Validates configuration, does not provision
      --retention-time <time>           Retention time (round to seconds) Ex: '1h', '2d 10s', '7 days' (default)
      --segment-size <bytes>            Segment size (by default measured in bytes) Ex: `2048`, '2 Ki', '10 MiB', `1 GB`
      --compression-type <compression>  Compression configuration for topic
      --max-partition-size <bytes>      Max partition size (by default measured in bytes) Ex: `2048`, '2 Ki', '10 MiB', `1 GB`
  -h, --help                            Print help (see more with '--help')
```