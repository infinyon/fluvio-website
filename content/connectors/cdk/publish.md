---
title:  Publish 
weight: 60
---
### 1.3. Publish

The final step in this scenario would be publishing the SmartModule to the Hub - where other users can download and integrate it into their data pipelines.

```bash
$ cdk publish --help
Publish Connector package to the Hub

Usage: cdk publish [OPTIONS] [PACKAGE_META]

Arguments:
  [PACKAGE_META]

Options:
      --public-yes
          don't ask for confirmation of public package publish

      --pack
          do only the pack portion

      --push
          given a packed file do only the push

      --remote <REMOTE>

  -h, --help
```

If run without arguments, it will pack everything needed into a package (`cdk build`must be executed beforehand) and push the package to the Hub.

If you need to inspect the package before the push:

```bash
$ cdk publish --pack
Package hub/http-source-0.1.0.ipkg created

# check the file and then push. The file is a Tar Gzip archive.
$ cdk publish --push hub/http-source-0.1.0.ipkg
```

`publish` command uses `hub/package-meta.yml` file with the metadata needed for the Hub. If it doesn’t exist, `cdk` creates it for you. Then, you can modify it, and the changes will be picked up on the subsequent command execution. 