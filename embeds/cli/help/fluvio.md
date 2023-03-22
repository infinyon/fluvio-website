```
Fluvio Command Line Interface
fluvio-cli [OPTIONS] <COMMAND>
Commands:
  consume       Read messages from a topic/partition
  produce       Write messages to a topic/partition
  topic         Manage and view Topics
  partition     Manage and view Partitions
  smartmodule   Create and manage SmartModules [aliases: sm]
  table-format  Create a TableFormat display specification [aliases: tf]
  hub           Work with the SmartModule Hub
  profile       Manage Profiles, which describe linked clusters
  cluster       Install or uninstall Fluvio cluster
  install       Install Fluvio plugins
  update        Update the Fluvio CLI
  version       Print Fluvio version information
  completions   Generate command-line completions for Fluvio
  run           Run Fluvio cluster components (SC and SPU)
  package       Package publishing and management
  cloud         Cloud Operations
Options:
  -c, --cluster <host:port>        Address of cluster
      --tls                        Enable TLS
      --enable-client-cert         TLS: use client cert
      --domain <DOMAIN>            Required if client cert is used
      --ca-cert <CA_CERT>          Path to TLS ca cert, required when client cert is enabled
      --client-cert <CLIENT_CERT>  Path to TLS client certificate
      --client-key <CLIENT_KEY>    Path to TLS client private key
  -P, --profile <profile>          
  -h, --help                       Print help
```