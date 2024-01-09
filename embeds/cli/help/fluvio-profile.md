```
Manage Profiles, which describe linked clusters

Each Profile describes a particular Fluvio cluster you may be connected to. This might correspond to Fluvio running on Minikube or in the Cloud. There is one "active" profile, which determines which cluster all of the Fluvio CLI commands interact with.

Usage: fluvio profile [COMMAND]

Commands:
  current         Print the name of the current context
  delete          Delete the named profile
  delete-cluster  Delete the named cluster
  list            Display the entire Fluvio configuration
  rename          Rename a profile
  switch          Switch to the named profile
  sync            Sync a profile from a cluster
  export          Export a profile for use in other applications
  add             Manually add a profile (advanced)

Options:
  -h, --help
          Print help (see a summary with '-h')
```