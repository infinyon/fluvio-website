```
Test SmartModule

Usage: smdk test [OPTIONS]

Options:
      --text <TEXT>
          
      --file <FILE>
          
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
  -h, --help
          Print help information
```