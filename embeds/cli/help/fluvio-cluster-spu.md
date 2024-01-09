```
Manage and view Streaming Processing Units (SPUs)

SPUs make up the part of a Fluvio cluster which is in charge of receiving messages from producers, storing those messages, and relaying them to consumers. This command lets you see the status of SPUs in your cluster.

Usage: fluvio cluster spu <COMMAND>

Commands:
  register    Register a new custom SPU with the cluster
  unregister  Unregister a custom SPU from the cluster
  list        List all SPUs known by this cluster (managed AND custom)

Options:
  -h, --help
          Print help (see a summary with '-h')
```