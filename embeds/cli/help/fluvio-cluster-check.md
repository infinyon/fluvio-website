```
Check that all requirements for cluster startup are met.

This command is useful to check if user has all the required dependencies and permissions to run fluvio on the current Kubernetes context.

It is not intended to be used in scenarios where user does not have access to Kubernetes resources (eg. Cloud)

Usage: fluvio cluster check [OPTIONS]

Options:
      --fix
          Attempt to fix recoverable errors

  -h, --help
          Print help (see a summary with '-h')
```