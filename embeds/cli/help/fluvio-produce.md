```
Write messages to a topic/partition
Usage: fluvio produce [OPTIONS] <topic>
Arguments:
  <topic>  The name of the Topic to produce to
Options:
  -v, --verbose
          Print progress output when sending records
      --key <KEY>
          Sends key/value records with this value as key
      --key-separator <KEY_SEPARATOR>
          Sends key/value records split on the first instance of the separator
      --raw
          Send all input as one record. Use this when producing binary files
      --compression <COMPRESSION>
          Compression algorithm to use when sending records. Supported values: none, gzip, snappy and lz4
  -f, --file <FILE>
          Path to a file to produce to the topic. Default: Each line treated as single record unless `--raw` specified. If absent, producer will read stdin
      --linger <LINGER>
          Time to wait before sending Ex: '150ms', '20s'
      --batch-size <BATCH_SIZE>
          Max amount of bytes accumulated before sending
      --isolation <ISOLATION>
          Isolation level that producer must respect. Supported values: read_committed (ReadCommitted) - wait for records to be committed before response, read_uncommitted (ReadUncommitted) - just wait for leader to accept records
      --delivery-semantic <DELIVERY_SEMANTIC>
          Delivery guarantees that producer must respect. Supported values: at_most_once (AtMostOnce) - send records without waiting from response, at_least_once (AtLeastOnce) - send records and retry if error occurred [default: at-least-once]
      --smartmodule <SMARTMODULE>
          Name of the smartmodule
      --smartmodule-path <SMARTMODULE_PATH>
          Path to the smart module
      --aggregate-initial <AGGREGATE_INITIAL>
          (Optional) Value to use as an initial accumulator for aggregate SmartModules
  -e, --params <PARAMS>
          (Optional) Extra input parameters passed to the smartmodule. They should be passed using key=value format Eg. fluvio produce topic-name --smartmodule my_filter -e foo=bar -e key=value -e one=1
      --transforms-file <TRANSFORMS_FILE>
          (Optional) Path to a file with transformation specification
  -t, --transform <TRANSFORM>
          (Optional) Transformation specification as JSON formatted string. E.g. fluvio produce topic-name --transform='{"uses":"infinyon/jolt@0.1.0","with":{"spec":"[{\"operation\":\"default\",\"spec\":{\"source\":\"test\"}}]"}}'
  -h, --help
          Print help
```