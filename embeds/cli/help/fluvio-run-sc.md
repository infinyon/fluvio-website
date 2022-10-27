```
Run a new Streaming Controller (SC)

Usage: fluvio-run sc [OPTIONS]

Options:
      --local
          running in local mode only
      --bind-public <BIND_PUBLIC>
          Address for external service
      --bind-private <BIND_PRIVATE>
          Address for internal service
  -n, --namespace <namespace>
          
      --tls
          enable tls
      --server-cert <SERVER_CERT>
          TLS: path to server certificate
      --server-key <SERVER_KEY>
          TLS: path to server private key
      --enable-client-cert
          TLS: enable client cert
      --ca-cert <CA_CERT>
          TLS: path to ca cert, required when client cert is enabled
      --bind-non-tls-public <BIND_NON_TLS_PUBLIC>
          TLS: address of non tls public service, required
      --authorization-scopes <authorization scopes path>
          [env: X509_AUTH_SCOPES=]
      --authorization-policy <authorization policy path>
          [env: AUTH_POLICY=]
      --white-list <WHITE_LIST>
          only allow white list of controllers
  -h, --help
          Print help information
```