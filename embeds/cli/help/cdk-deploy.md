```
Deploy the Connector from the current working directory

Usage: cdk deploy [OPTIONS] [-- <EXTRA_ARGUMENTS>...] <COMMAND>

Commands:
  start     Start new deployment for the given connector config
  shutdown  Shutdown the Connector's deployment
  list      Print the list of all deployed connectors
  log       Print the connector's logs
  help      Print this message or the help of the given subcommand(s)

Arguments:
  [EXTRA_ARGUMENTS]...  Extra arguments to be passed to cargo

Options:
      --release <RELEASE>            Release profile name [default: release]
      --target <TARGET>              Provide target platform for the package. Optional. By default
                                     the host's one is used [default: aarch64-apple-darwin]
  -p, --package-name <PACKAGE_NAME>  Optional package/project name
  -h, --help                         Print help
```