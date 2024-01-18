---
title: Fluvio Release Channels
menu: Release Channels
weight: 10
---

The current version of release channels was introduced in 0.10.16 with the introduction of [`fvm`]({{<ref "/cli/install/fluvio-version-manager.md">}})


## What is this for?

Occasionally users report issues in our Discord. After a fix was found and committed, users wanted a way to verify their issue had been resolved before a release.

We build and distribute testing binaries and images each time we merge a pull request. However, walking users through this installation was error-prone, and you couldn't easily swap back to stable. You needed to re-install.

Starting a cluster from an unofficial release wasn't a straightforward process either. The Fluvio CLI is geared towards production users using official releases. An unofficial release cluster required setting a few extra CLI options to `fluvio cluster start`. An easy step to forget.

Inspired by [Rust's concept of channels](https://rust-lang.github.io/rustup/concepts/channels.html), Fluvio's release channels address these issues and provide an easy way to switch between development and stable releases.

## Installation

Release channel support is provided by `fvm`. The installer uses the `stable` channel by default.

%copy first-line%
```shell
$ curl -fsS https://hub.infinyon.cloud/install/install.sh | bash
```


## Quick start: The Channels

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