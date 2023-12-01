```
Run a new Streaming Processing Unit (SPU)

Usage: fluvio-run spu [OPTIONS]

Options:
  -i, --id <integer>
          SPU unique identifier
  -p, --public-server <host:port>
          Spu server for external communication
  -v, --private-server <host:port>
          Spu server for internal cluster communication
      --sc-addr <host:port>
          Address of the SC Server [env: FLV_SC_PRIVATE_HOST=]
      --log-base-dir <dir>
          [env: FLV_LOG_BASE_DIR=]
      --log-size <log size>
          [env: FLV_LOG_SIZE=]
      --index-max-bytes <integer>
          [env: FLV_LOG_INDEX_MAX_BYTES=]
      --index-max-interval-bytes <integer>
          [env: FLV_LOG_INDEX_MAX_INTERVAL_BYTES=]
      --peer-max-bytes <integer>
          max bytes to transfer between leader and follower [env: FLV_PEER_MAX_BYTES=] [default: 1000000]
      --smart-engine-max-memory <integer>
          [env: FLV_SMART_ENGINE_MAX_MEMORY_BYTES=]
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
  -h, --help
          Print help
```