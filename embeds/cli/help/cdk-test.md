```
Build and run the Connector in the current working directory

Usage: cdk test [OPTIONS] [-- <EXTRA_ARGUMENTS>...]

Arguments:
  [EXTRA_ARGUMENTS]...  Extra arguments to be passed to cargo

Options:
      --release <RELEASE>            Release profile name [default: release]
      --target <TARGET>              Provide target platform for the package. Optional. By default
                                     the host's one is used [default: aarch64-apple-darwin]
  -p, --package-name <PACKAGE_NAME>  Optional package/project name
  -c, --config <PATH>                Path to configuration file in YAML format [default:
                                     sample-config.yaml]
  -s, --secrets <PATH>               Path to file with secrets. Secrets are 'key=value' pairs
                                     separated by the new line character. Optional
  -h, --help                         Print help
```