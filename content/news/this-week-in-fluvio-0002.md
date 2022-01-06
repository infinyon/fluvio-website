---
title: "This Week in Fluvio #2"
date: 2021-08-12
weight: 20
---

Welcome to the second edition of This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

[Fluvio open source]: https://github.com/infinyon/fluvio

{{< banner >}}

## New Release - Fluvio v0.9.1

In `0.9.1`, we added support for running a Fluvio cluster locally on an
Apple M1 machine (architecture `aarch64-apple-darwin`). This means that if
you are running an M1-powered device, you can now install Fluvio and run
a cluster directly on your machine, without needing to deploy to some
Kubernetes instance somewhere.

On M1, you can now run the standard `install.sh` script and get both the
Fluvio CLI and the `fluvio-run` cluster binary:

%copy first-line%
```bash
$ curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

After this has run (and you've set up your PATH), you can now run a local
cluster on M1 with the following command:

%copy first-line%
```bash
$ fluvio cluster start --local
```

[See our complete getting-started guide for Mac] for a full set of instructions
for getting set up from scratch.

[See our complete getting-started guide for Mac]: /docs/get-started/mac/

## Release of Fluvio `0.9.2`

That's right, we had _two_ point releases this week. `0.9.2` was shipped
because of a bug we discovered with a version handshake between Fluvio clients
and servers that is supposed to ensure compatibility.

Basically, each build of Fluvio "knows" what version it is, e.g. `0.9.0` or `0.9.1`.
When a Fluvio client and server first connect, they make sure that they each
have versions that are compatible with each other. Unfortunately, we had a build
problem where the server (Fluvio SC) did not pick up the latest version number,
and therefore the client thought it was talking to an old server! It turns out that
this bug was present since the update from `0.8.5` to `0.9.0`, so the `0.9.x` clients
thought they were talking to an `0.8.x` server. This version gap spanned a major version
bump, so the client rejected the connections! We suspect this bug happened because we did
not have any code changes to the SC for `0.9.1` or `0.9.2`, so it actually did not get
recompiled with the new version number baked in. All this to say, we are now back on
track with matching versions in the client/server builds.

In addition to these compatibility fixes, `0.9.2` also included some internal fixes
to the Streaming Processing Units (SPUs) that should make them more reliable when being
deployed in a Kubernetes cluster.

## Conclusion

That's it for this week, short and sweet! If you have any questions or would like
to get involved, feel free to [join our Discord channel].

Until next week!

[join our Discord channel]: https://discordapp.com/invite/bBG2dTz
