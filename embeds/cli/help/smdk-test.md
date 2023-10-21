```
Test SmartModule

Usage: smdk test [OPTIONS] [KEY]

Arguments:
  [KEY]  Key to use with the test record(s)

Options:
      --text <TEXT>
          Provide test input with this flag
      --stdin
          Read the test input from the StdIn (e.g. Unix piping)
      --file <FILE>
          Path to test file. Default: Read file line by line
      --raw
          Read the file as single record
  -k, --key-value
          Print records in "[key] value" format, with "[null]" for no key
      --release <RELEASE>
          Release profile name [default: release-lto]
  -p, --package-name <PACKAGE_NAME>
          Optional package/project name
      --wasm-file <WASM_FILE>
          Optional wasm file path
  -e, --params <PARAMS>
          (Optional) Extra input parameters passed to the smartmodule module. They should be passed
          using key=value format Eg. fluvio consume topic-name --filter filter.wasm -e foo=bar -e
          key=value -e one=1
      --transforms-file <TRANSFORMS_FILE>
          (Optional) File path to transformation specification
  -t, --transform <TRANSFORM>
          (Optional) Pass transformation specification as JSON formatted string. E.g. smdk test
          --text '{}'
          --transform='{"uses":"infinyon/jolt@0.1.0","with":{"spec":"[{\"operation\":\"default\",\"spec\":{\"source\":\"test\"}}]"}}'
  -v, --verbose
          verbose output
  -r, --record <RECORD>
          Records which act as existing in the topic before the SmartModule starts processing.
          Useful for testing `lookback`. Multiple values are allowed
  -l, --lookback-last <LOOKBACK_LAST>
          Sets the lookback parameter to the last N records
  -h, --help
          Print help
```