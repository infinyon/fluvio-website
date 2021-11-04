---
title: "This Week in Fluvio #7"
date: 2021-09-23
weight: 20
---

Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

## Release of Fluvio `0.9.8`

### Improved progress indicator for `fluvio cluster start`

We've been making some improvements to the Fluvio CLI to make the experience nicer
for new users. This week, we've made some changes to the `fluvio cluster start` command
including the addition of an animated spinner and some better status messages during
the cluster installation.

Before this week, the command's output looked like this:

<video controls width="860px" title="The old command used plain print statements and no spinner">
  <source src="/news/images/0007/fluvio-cluster-start-old.mov">
</video>

With the release of `0.9.8`, the cluster installer now looks like this:

<video controls width="860px" title="The new command uses emojis, bold print, and an animated spinner">
  <source src="/news/images/0007/fluvio-cluster-start.mov">
</video>

### Introducing `fluvio cluster diagnostics`

Fluvio is still in alpha, and occasionally things go wrong. In the past when helping users,
we've found that we need to go back and forth multiple times to ask for logs and other
info that helps to debug issues. This was a very time-consuming process - until now.

This week, we've added a new command to the Fluvio CLI, `fluvio cluster diagnostics`. This
command will collect information such as logs from the Fluvio SC and SPUs, Kubernetes
metadata relating to Fluvio, and Fluvio metadata such as SPU and SPU Group state. It will
collect this information and place it into a `diagnostics-<datetime>.tar.gz` file. If you
find yourself stuck and need help debugging an error, we can ask you just once for this
diagnostic information, rather than needing an extended back-and-forth debugging session
in chat.

## Conclusion

For the full list of changes this week, be sure to check out [our CHANGELOG]. If you have any
questions or are interested in contributing, be sure to [join our Discord channel] and
come say hello!

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[wasmtime API since 0.28]: https://github.com/alexcrichton/rfcs-2/blob/new-api/accepted/new-api.md
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[join our Discord channel]: https://discordapp.com/invite/bBG2dTz
