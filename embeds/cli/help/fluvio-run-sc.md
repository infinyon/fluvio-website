```
Run a new Streaming Controller (SC)

Usage: fluvio-run sc [OPTIONS] <--local <metadata path>|--k8|--read-only <READ_ONLY>>

Options:
      --local <metadata path>
          run in local mode
      --k8
          run on k8
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
      --secret-name <SECRET_NAME>
          Secret name used while adding to kubernetes
      --authorization-scopes <authorization scopes path>
          [env: X509_AUTH_SCOPES=]
      --authorization-policy <authorization policy path>
          [env: AUTH_POLICY=]
      --white-list <WHITE_LIST>
          only allow white list of controllers
  -h, --help
          Print help
```