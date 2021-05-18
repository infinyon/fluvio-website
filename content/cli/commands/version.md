---
title: Version
weight: 90
---

The `fluvio version` command prints out useful version and build information about
the Fluvio executable you're working with, as well as the plugins you have installed.

%copy first-line%
```bash
$ fluvio version
Fluvio CLI           : 0.8.2
Fluvio CLI SHA256    : e9460058ec0ea5b83293a27d360e044c32f5ed696e3ecde83d91308bb0c8ccee
Fluvio Platform      : 0.8.3 (minikube)
Git Commit           : 5952736f824945c970b4adedfb7db246d07f122a
OS Details           : Darwin 10.16 (kernel 20.4.0)
=== Plugin Versions ===
Fluvio Runner (fluvio-run)     : 0.2.0
Fluvio Cloud CLI (fluvio-cloud) : 0.1.5
```

If you ever need help solving a problem with Fluvio or believe you have encountered
a bug, this information will help us to identify where the problem is coming from,
so be sure to include it in bug reports!
