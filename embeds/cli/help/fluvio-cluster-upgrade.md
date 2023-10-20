```
Upgrades an already-started Fluvio cluster

Usage: fluvio cluster upgrade [OPTIONS]

Options:
      --develop
          use local image
      --chart-version <CHART_VERSION>
          k8: use specific chart version
      --image-version <IMAGE_VERSION>
          k8: use specific image version
      --registry <REGISTRY>
          k8: use custom docker registry
      --namespace <NAMESPACE>
          k8 namespace [default: default]
      --group-name <GROUP_NAME>
          k8 [default: main]
      --install-name <INSTALL_NAME>
          helm chart installation name [default: fluvio]
      --chart-location <CHART_LOCATION>
          Local path to a helm chart to install
      --chart-values <CHART_VALUES>
          chart values
      --use-k8-port-forwarding
          Uses port forwarding for connecting to SC during install
      --use-cluster-ip
          Uses port forwarding for connecting to SC during install
      --tls-client-secret-name <TLS_CLIENT_SECRET_NAME>
          TLS: Client secret name while adding to Kubernetes [default: fluvio-client-tls]
      --tls-server-secret-name <TLS_SERVER_SECRET_NAME>
          TLS: Server secret name while adding to Kubernetes [default: fluvio-tls]
      --spu-storage-size <SPU_STORAGE_SIZE>
          set spu storage size [default: 10]
      --skip-profile-creation
          
      --spu <SPU>
          number of SPU [default: 1]
      --rust-log <RUST_LOG>
          RUST_LOG options
      --log-dir <LOG_DIR>
          log dir [default: /usr/local/var/log/fluvio]
      --sys-only
          installing/upgrade sys only
      --local
          install local spu/sc(custom)
      --tls
          Whether to use TLS
      --domain <DOMAIN>
          TLS: domain
      --ca-cert <CA_CERT>
          TLS: ca cert
      --client-cert <CLIENT_CERT>
          TLS: client cert
      --client-key <CLIENT_KEY>
          TLS: client key
      --server-cert <SERVER_CERT>
          TLS: path to server certificate
      --server-key <SERVER_KEY>
          TLS: path to server private key
      --authorization-config-map <AUTHORIZATION_CONFIG_MAP>
          
      --skip-checks
          Whether to skip pre-install checks, defaults to false
      --setup
          Tries to setup necessary environment for cluster startup
      --proxy-addr <PROXY_ADDR>
          Proxy address
      --service-type <SERVICE_TYPE>
          Service Type
      --read-only <READ_ONLY>
          Start SC in read only mode
  -h, --help
          Print help
```