---
title: Fluvio Release Channels
menu: Release Channels
weight: 10
---

## What is this for?

Occasionally users report issues in our Discord. After a fix was found and committed, users wanted a way to verify their issue had been resolved before a release.

We build and distribute testing binaries and images each time we merge a pull request. However, walking users through this installation was error-prone, and you couldn't easily swap back to stable. You needed to re-install.

Starting a cluster from an unofficial release wasn't a straightforward process either. The Fluvio CLI is geared towards production users using official releases. An unofficial release cluster required setting a few extra CLI options to `fluvio cluster start`. An easy step to forget.

Inspired by [Rust's concept of channels](https://rust-lang.github.io/rustup/concepts/channels.html), Fluvio's release channels address these issues for anyone to test the bleeding edge, with minimal thinking required. While also providing an easy path to switch back stable releases.

## Installation

If you have recently installed Fluvio through that method, there is nothing left to do.

Release channel support is provided by the [official installer script]({{<ref "/download">}}). The installer uses the `stable` channel by default. 

{{<caution>}}
Fluvio release channels were introduced for the CLI in `0.9.16`.
<br><br>
If you have an installation from a release earlier than `0.9.16`, you should re-install with the [official installer script]({{<ref "/download">}}) to install the Fluvio channel frontend.
{{</caution>}}

## Quick start: The Channels

There are 3 channels for users to use.

### Stable
This is the most recent release, and the default channel installed.

To switch to the `stable` channel:

%copy first-line%
```shell
$ fluvio version switch stable
```

### Latest
This is the most recent commit to the `master` branch

To switch to the `latest` channel:

%copy first-line%
```shell
$ fluvio version switch latest 
```

The first time you switch to this channel, the binary will be downloaded.

### Version
This is a specific version.

To switch to a version channel, you need to create it first.

At this step the binary will get downloaded.

%copy first-line%
```shell
$ fluvio version create X.Y.Z 
```

Then you can switch to the version channel

%copy first-line%
```shell
$ fluvio version switch X.Y.Z 
```

Where `X.Y.Z` is the version of a release you want to switch to (e.g. `0.9.18`)

## How it works?

Release channel support is set up at install time through the [installer script]({{<ref "/download">}}).

2 binaries are installed
* The Fluvio CLI
* A frontend binary to support switching channels

By default, the `stable` channel is selected.

This channel is registered in `~/.fluvio/channel` and is marked as the active channel.

Example channel config:

```toml
current_channel = "stable"
[channel.stable]
binary_location = "/home/username/.fluvio/bin/fluvio-stable"
extensions = "/home/username/.fluvio/extensions"
image_tag_strategy = "Version"

[channel.latest]
binary_location = "/home/username/.fluvio/bin/fluvio-latest"
extensions = "/home/username/.fluvio/extensions-latest"
image_tag_strategy = "VersionGit"
```

When you run `fluvio update`, the Fluvio binary of your current channel will update to the newest version of the channel.

{{<caution>}}
Only the `stable` and `latest` channels can be updated. Version channels don't support `fluvio update`  because those binaries will always be pinned to its version.
<br><br>
Follow the [version channel]({{<ref "#version">}}) steps to switch to a different version.
{{</caution>}}