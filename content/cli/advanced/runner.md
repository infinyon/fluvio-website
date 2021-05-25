---
title: Run
weight: 10
---

The `fluvio run` commands are used to directly invoke Fluvio's cluster
components, the [Streaming Controller (SC)] and [Streaming Processing Unit (SPU)].
For most use-cases, you'll probably want to use the [`fluvio cluster`] commands,
which are simpler to use and invoke these runner commands for you. However, if
you want to manually administrate your own cluster and have fine-grained control
over the components, then this is the section for you.

[Streaming Controller (SC)]: {{< ref "/docs/architecture/sc" >}}
[Streaming Processing Unit (SPU)]: {{< ref "/docs/architecture/spu" >}}
[`fluvio cluster`]: {{< ref "/cli/commands/cluster" >}}

## `fluvio run sc`

Directly invoke a Streaming Controller (SC) from the command line. 
An SC hosts a public server for interacting with clients, and a
private server for interacting with SPUs and other cluster components. Most of
the CLI options for the SC involve configuring these servers. See the
[SC Architecture] for more details.

[SC Architecture]: {{< ref "/docs/architecture/sc" >}}

```
fluvio-run-sc
Run a new Streaming Controller (SC)

USAGE:
    fluvio cluster run sc [FLAGS] [OPTIONS]

FLAGS:
        --tls                   enable tls
        --enable-client-cert    TLS: enable client cert
    -h, --help                  Prints help information

OPTIONS:
        --bind-public <bind-public>
            Address for external service

        --bind-private <bind-private>
            Address for internal service

    -n, --namespace <namespace>
        --server-cert <server-cert>
            TLS: path to server certificate

        --server-key <server-key>
            TLS: path to server private key

        --ca-cert <ca-cert>
            TLS: path to ca cert, required when client cert is enabled

        --bind-non-tls-public <bind-non-tls-public>
            TLS: address of non tls public service, required

        --authorization-scopes <authorization scopes path>
             [env: X509_AUTH_SCOPES=]

        --authorization-policy <authorization policy path>
             [env: AUTH_POLICY=]
```

---

## `fluvio run spu`

Directly invokes a Streaming Processing Unit (SPU) from the command line.
An SPU has a public server which is used to stream data to and from Fluvio Clients, and
a private server which is used to communicate with an SC. Most of the CLI options for
the SPU involve configuring these servers. See the [SPU Architecture] for more details.

[SPU Architecture]: {{< ref "/docs/architecture/spu" >}}

```
fluvio-run-spu
Run a new Streaming Processing Unit (SPU)

USAGE:
    fluvio cluster run spu [FLAGS] [OPTIONS]

FLAGS:
        --tls                   enable tls
        --enable-client-cert    TLS: enable client cert
    -h, --help                  Prints help information

OPTIONS:
    -i, --id <integer>                                 SPU unique identifier
    -p, --public-server <host:port>
            Spu server for external communication

    -v, --private-server <host:port>
            Spu server for internal cluster communication

        --sc-addr <host:port>
            Address of the SC Server [env: FLV_SC_PRIVATE_HOST=]

        --log-base-dir <dir>                            [env: FLV_LOG_BASE_DIR=]
        --log-size <log size>                           [env: FLV_LOG_SIZE=]
        --index-max-bytes <integer>
             [env: FLV_LOG_INDEX_MAX_BYTES=]

        --index-max-interval-bytes <integer>
             [env: FLV_LOG_INDEX_MAX_INTERVAL_BYTES=]

        --peer-max-bytes <integer>
            max bytes to transfer between leader and follower [env:
            FLV_PEER_MAX_BYTES=]  [default: 1000000]
        --server-cert <server-cert>
            TLS: path to server certificate

        --server-key <server-key>
            TLS: path to server private key

        --ca-cert <ca-cert>
            TLS: path to ca cert, required when client cert is enabled

        --bind-non-tls-public <bind-non-tls-public>
            TLS: address of non tls public service, required
```
