```
Read messages from a topic/partition

Usage: fluvio consume [OPTIONS] <topic>

Arguments:
  <topic>  Topic name

Options:
  -p, --partition <integer>
          Partition id [default: 0]
  -A, --all-partitions
          Consume records from all partitions
  -d, --disable-continuous
          disable continuous processing of messages
      --disable-progressbar
          disable the progress bar and wait spinner
  -k, --key-value
          Print records in "[key] value" format, with "[null]" for no key
  -F, --format <FORMAT>
          Provide a template string to print records with a custom format. See --help for details
      --table-format <TABLE_FORMAT>
          Consume records using the formatting rules defined by TableFormat name
  -B [<integer>]
          Consume records starting X from the beginning of the log (default: 0)
  -o, --offset <integer>
          The offset of the first record to begin consuming from
  -T, --tail [<integer>]
          Consume records starting X from the end of the log (default: 10)
      --end-offset <integer>
          Consume records until end offset
  -b, --maxbytes <integer>
          Maximum number of bytes to be retrieved
      --suppress-unknown
          Suppress items items that have an unknown output type
  -O, --output <type>
          Output [possible values: dynamic, text, binary, json, raw, table, full-table]
      --smartmodule <SMARTMODULE>
          name of the smart module
      --smartmodule-path <SMARTMODULE_PATH>
          
      --aggregate-initial <AGGREGATE_INITIAL>
          (Optional) Path to a file to use as an initial accumulator value with --aggregate
  -e, --params <PARAMS>
          (Optional) Extra input parameters passed to the smartmodule module. They should be passed using key=value format Eg. fluvio consume topic-name --filter filter.wasm -e foo=bar -e key=value -e one=1
      --isolation <ISOLATION>
          Isolation level that consumer must respect. Supported values: read_committed (ReadCommitted) - consume only committed records, read_uncommitted (ReadUncommitted) - consume all records accepted by leader
  -h, --help
          Print help information (use `--help` for more detail)
```