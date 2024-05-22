```
Read messages from a topic/partition

Usage: fluvio consume [OPTIONS] <topic>

Arguments:
  <topic>
          Topic name

Options:
  -p, --partition <integer>
          Partition id

  -A, --all-partitions
          Consume records from all partitions

  -d, --disable-continuous
          Disable continuous processing of messages

      --disable-progressbar
          Disable the progress bar and wait spinner

  -k, --key-value
          Print records in "[key] value" format, with "[null]" for no key

  -F, --format <FORMAT>
          Provide a template string to print records with a custom format. See --help for details.
          
          Template strings may include the variables {{key}}, {{value}}, {{offset}}, {{partition}} and {{time}} which will have each record's contents substituted in their place. Note that timestamp is displayed using RFC3339, is always UTC and ignores system timezone.
          
          For example, the following template string:
          
          Offset {{offset}} has key {{key}} and value {{value}}
          
          Would produce a printout where records might look like this:
          
          Offset 0 has key A and value Apple

      --table-format <TABLE_FORMAT>
          Consume records using the formatting rules defined by TableFormat name

  -B, --beginning
          Consume records from the beginning of the log

  -H, --head <integer>
          Consume records starting <integer> from the beginning of the log

  -T, --tail <integer>
          Consume records starting <integer> from the end of the log

      --start <integer>
          The absolute offset of the first record to begin consuming from

      --end <integer>
          Consume records until end offset (inclusive)

  -b, --maxbytes <integer>
          Maximum number of bytes to be retrieved

      --isolation <ISOLATION>
          Isolation level that consumer must respect. Supported values: read_committed (ReadCommitted) - consume only committed records, read_uncommitted (ReadUncommitted) - consume all records accepted by leader

      --suppress-unknown
          Suppress items items that have an unknown output type

  -O, --output <type>
          Output
          
          [possible values: dynamic, text, binary, json, raw, table, full-table]

      --smartmodule <SMARTMODULE>
          Name of the smartmodule

      --smartmodule-path <SMARTMODULE_PATH>
          Path to the smart module

      --aggregate-initial <AGGREGATE_INITIAL>
          (Optional) Value to use as an initial accumulator for aggregate SmartModules

  -e, --params <PARAMS>
          (Optional) Extra input parameters passed to the smartmodule. They should be passed using key=value format Eg. fluvio consume topic-name --smartmodule my_filter -e foo=bar -e key=value -e one=1

  -t, --transforms <TRANSFORMS>
          (Optional) Path to a file with transformation specification

      --transforms-line <TRANSFORMS_LINE>
          (Optional) Transformation specification as JSON formatted string. E.g. fluvio consume topic-name --transforms-line='{"uses":"infinyon/jolt@0.1.0","with":{"spec":"[{\"operation\":\"default\",\"spec\":{\"source\":\"test\"}}]"}}'

      --truncate
          Truncate the output to one line

  -c, --consumer <CONSUMER>
          Consumer id

  -h, --help
          Print help (see a summary with '-h')
```