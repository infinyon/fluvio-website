```
Publish Connector package to the Hub

Usage: cdk publish [OPTIONS] [PACKAGE_META]

Arguments:
  [PACKAGE_META]
          

Options:
      --release <RELEASE>
          Release profile name
          
          [default: release]

      --target <TARGET>
          Provide target platform for the package. Optional. By default the host's one is used
          
          [default: aarch64-apple-darwin]

  -p, --package-name <PACKAGE_NAME>
          Optional package/project name

      --ipkg <IPKG>
          path to the ipkg file, used when --push is specified

      --public-yes
          don't ask for confirmation of public package publish

      --no-build
          

      --pack
          do only the pack portion

      --push
          given a packed file do only the push

      --remote <REMOTE>
          

      --readme <README>
          Relative path to this connector package README
          
          [default: ./README.md]

  -h, --help
          Print help (see a summary with '-h')
```