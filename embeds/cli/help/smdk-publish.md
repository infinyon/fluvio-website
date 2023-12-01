```
Publish SmartModule to Hub

Usage: smdk publish [OPTIONS] [PACKAGE_META]

Arguments:
  [PACKAGE_META]
          

Options:
      --release <RELEASE>
          Release profile name
          
          [default: release-lto]

  -p, --package-name <PACKAGE_NAME>
          Optional package/project name

      --ipkg <IPKG>
          path to the ipkg file, used when --push is specified

      --public-yes
          don't ask for confirmation of public package publish

      --pack
          do only the pack portion

      --push
          given a packed file do only the push

      --remote <REMOTE>
          

  -h, --help
          Print help (see a summary with '-h')
```