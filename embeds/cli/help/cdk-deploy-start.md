```
Start new deployment for the given connector config

Usage: cdk deploy start [OPTIONS] --config <PATH>

Options:
  -c, --config <PATH>          Path to configuration file in YAML format
  -s, --secrets <PATH>         Path to file with secrets. Secrets are 'key=value' pairs separated by
                               the new line character. Optional
      --ipkg <PATH>            Deploy from local package file
      --log-level <LOG_LEVEL>  Log level for the connector process [default: ]
  -h, --help                   Print help
```