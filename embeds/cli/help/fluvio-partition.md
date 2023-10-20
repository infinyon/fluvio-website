```
Manage and view Partitions

Partitions are a way to divide the total traffic of a single Topic into separate streams which may be processed independently. Data sent to different partitions may be processed by separate SPUs on different computers. By dividing the load of a Topic evenly among partitions, you can increase the total throughput of the Topic.

Usage: fluvio partition <COMMAND>

Commands:
  list  List all of the Partitions in this cluster

Options:
  -h, --help
          Print help (see a summary with '-h')
```