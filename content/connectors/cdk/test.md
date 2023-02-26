---
title: Test 
weight: 50
---
## 1.2. Test

If the connector builds successfully, it’s time to `cdk test`:

```bash
$ cdk test --help
Build and run the Connector in the current working directory

Usage: cdk test [OPTIONS] --config <PATH> [-- <EXTRA_ARGUMENTS>...]

Arguments:
  [EXTRA_ARGUMENTS]...  Extra arguments to be passed to cargo

Options:
      --release <RELEASE>            Release profile name [default: release]
  -p, --package-name <PACKAGE_NAME>  Optional package/project name
  -c, --config <PATH>                Path to configuration file in YAML format
  -s, --secrets <PATH>               Path to file with secrets. Secrets are 'key=value' pairs separated by the new line character. Optional
  -h, --help                         Print help
```

The `test` command runs the connector’s binary (if it is not built, it builds it). 

The `--config <PATH>` argument is required. It is a path to the configuration file in YAML format. 

The configuration might have `secrets`. By default, `secrets` are resolved from environment variables unless a secrets file is provided via `-s, --secrets <PATH>` argument. The expected format of this file is `key=value` pairs separated by the new line character.

SmartConnector output will be redirected to the current terminal output.

To stop running SmartConnector in test mode, press Ctrl+C.
