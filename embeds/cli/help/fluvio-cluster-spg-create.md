```
Create a new managed SPU Group

fluvio cluster spg create [OPTIONS] <name>

Arguments:
  <name>  The name for the new SPU Group

Options:
  -r, --replicas <integer>     The number of SPUs to create in this SPG [default: 1]
      --min-id <integer>       Minimum SPU ID [default: 1]
      --rack <string>          Rack name
      --storage-size <string>  The amount of storage to assign to this SPG
  -h, --help                   Print help information
```