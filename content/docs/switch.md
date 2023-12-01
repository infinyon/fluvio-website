---
title: Switch version
weight: 30
---

{{% inline-embed file="embeds/cli/help/fvm-switch.md" %}}

## Supported release channels 

There are 3 channels for users to use.

### Stable
This is the most recent release, and the default channel installed.

To switch to the `stable` channel:

%copy first-line%
```shell
$ fvm switch stable
```

### Latest
This is the most recent commit to the `master` branch

To switch to the `latest` channel:

%copy first-line%
```shell
$ fvm switch latest 
```

The first time you switch to this channel, the binary will be downloaded.

### Version
This is a specific version.

To switch to a version channel, you need to create it first.

At this step the binary will get downloaded.

%copy first-line%
```shell
$ fvm install X.Y.Z 
```

Then you can switch to the version channel

%copy first-line%
```shell
$ fvm switch X.Y.Z 
```

Where `X.Y.Z` is the version of a release you want to switch to (e.g. `0.9.18`)