```
Load SmartModule into Fluvio cluster

Usage: smdk load [OPTIONS]

Options:
      --name <NAME>
          
      --package-path <PACKAGE_PATH>
          Optional path to SmartModule package directory
      --release <RELEASE>
          Release profile name [default: release-lto]
  -p, --package-name <PACKAGE_NAME>
          Optional package/project name
      --wasm-file <WASM_FILE>
          Optional wasm file path
  -c, --cluster <host:port>
          Address of cluster
      --tls
          Enable TLS
      --enable-client-cert
          TLS: use client cert
      --domain <DOMAIN>
          Required if client cert is used
      --ca-cert <CA_CERT>
          Path to TLS ca cert, required when client cert is enabled
      --client-cert <CLIENT_CERT>
          Path to TLS client certificate
      --client-key <CLIENT_KEY>
          Path to TLS client private key
  -P, --profile <profile>
          
      --dry-run
          Validate package config files, and connection to cluster. Skip SmartModule load to cluster
  -h, --help
          Print help information
```