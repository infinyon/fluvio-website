---
title: "This Week in Fluvio #58"
date: 2024-01-17
weight: 20
---
Fluvio is a lean and mean distributed stream processing system written in rust and web assembly.

---
We paused the weekly updates because we had several big changes in flight. We will refactor this blog to Fluvio Release Updates and remove the weekly cadence in the next iteration.

A lot has happened in the past couple of months that is relevant to update.

## New release
Fluvio moved to version 0.11.3 a couple of days ago. We have had a bunch of releases since we last published an update on our site.

Full Changelog of the release is here:
[Fluvio Changelog](https://github.com/infinyon/fluvio/blob/7fbe42ca06ead90f1821a551ff258b868f3fff3c/CHANGELOG.md)


## New features
Fluvio has a bunch of exciting updates. The community has been asking for a single binary deployment that can be run locally, using Docker, using Nomad. That meant we needed to decouple our tigh coupling with Kubernetes. We did that in 2023 and the community started building with Fluvio!

We have been busy making documentation updates since then. So this update was delayed. Below are the main updates:

- Fluvio now has a version manager that manages multiple versions, installs, updates, etc. We are calling it Fluvio Version Manager(fvm)
- fvm can be installed by simply running:
```bash
curl -fsS https://hub.infinyon.cloud/install/install.sh | bash
```
- You can deploy Fluvio as a single compiled binary using **fvm**. The installer takes care of everything!
- You can run a local self-hosted Fluvio cluster by simply running
```bash
fluvio cluster start
```
which will start a self hosted fluvio clsuter using local resources

In light of this our Quick Start docs has been updated to reflect the changes. Getting started with ```fluvio``` is the easiest it has ever been.

[Updated Quick Start Link](https://www.fluvio.io/docs/)

## Upcoming features
There are some exciting community projects that are in development:

- There are some awesome new contributions in the works which inclused integrations with Spider Web Crawler, OpenSearch, ElasticSearch, Qdrant, Surreal, OpenDAL etc.
- We have an oversubscribed ```developer preview``` for Stateful Service Development Kit which makes stateful stream processing a reality.
- We have docs on docker based deployment. [Link](https://www.infinyon.com/docs/tutorials/docker-installation/)


## Good First Issues

If you are excited to contribute to Fluvio Open Source, here are 3 good first issues that you can consider:
- [connector: fluvio-http-source, add an option to read data from a websocket](https://github.com/infinyon/fluvio/issues/3829)
- [MQTT Connector:  Prefix auto generated Client ID to prevent connection drops](https://github.com/infinyon/fluvio/issues/3825)
- [`fluvio cluster delete` should prompt with cluster and endpoint name confirmation](https://github.com/infinyon/fluvio/issues/3493)

All the best.

---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions
