```
Download SmartModules - locally or to cluster (default)

Usage: fluvio hub smartmodule download [OPTIONS] <name>

Arguments:
  <name>
          SmartModule name: e.g. infinyon/jolt@v0.0.1

Options:
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
          

  -o, --output <PATH>
          Download package to local filesystem

      --ipkg
          given local package file, download to cluster

      --remote <REMOTE>
          

  -h, --help
          Print help (see a summary with '-h')
```