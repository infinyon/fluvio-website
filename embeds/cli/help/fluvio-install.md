```
Install Fluvio plugins

The Fluvio CLI considers any executable with the prefix `fluvio-` to be a CLI plugin. For example, an executable named `fluvio-foo` in your PATH may be invoked by running `fluvio foo`.

This command allows you to install plugins from Fluvio's package registry.

Usage: fluvio install [OPTIONS] [PACKAGE]

Arguments:
  [PACKAGE]
          The ID of a package to install, e.g. "fluvio/fluvio-cloud"

Options:
      --develop
          Install the latest prerelease rather than the latest release
          
          If the package ID contains a version (e.g. `fluvio/fluvio:0.6.0`), this is ignored

      --hub
          When this flag is provided, use the hub. Dev-only

      --use-hub-defaults
          Use local hub defaults. Implied if INFINYON_HUB_REMOTE or FLUVIO_CLOUD_PROFILE env vars are set - Dev-only

      --channel <CHANNEL>
          When this flag is provided, use the hub. Dev-only

      --target <TARGET>
          override default target arch determination

  -h, --help
          Print help (see a summary with '-h')
```