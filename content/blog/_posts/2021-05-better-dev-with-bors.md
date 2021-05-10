---
title: Increasing our development productivity and confidence with Bors
author:
    name: "Nick Mosher"
    github: "nicholastmosher"
description: "TBD"
date: 2021-04-20
slug: github-actions-best-practices
url: /blog/2021/04/github-actions-best-practices
img: blog/images/rust-workflows/github-actions-social.png
twitter-card: summary_large_image
code:
    height: 9000
---

Lately at InfinyOn, we've been doing a lot of work to improve our productivity and speed
up our development cycles. One of the easiest and most effective things we've done in
order to achieve this goal is to integrate the [Bors-ng] GitHub bot into our development
workflow. In this post, I'll talk about some of the problems we were facing, what Bors is
and why it was a good solution for our team, and how adopting it has helped us to increase
our development speed and confidence. I'll then describe how to set up Bors
for your own repository and what to expect out of the new development workflow.

[Bors-ng]: https://bors.tech

## What even is Bors?

Bors is a GitHub application/bot that you put in charge of merging your PRs into master.
Importantly, it merges commits in such a way that ensures that the exact code that lands
in master is the code that has been tested by your CI workflow, which, perhaps surprisingly,
is not how typical merges work. The problem with classic merges is that they can cause
problems such as [semantic merge conflicts], in which two separate PRs each make changes
that work in isolation, but which cause failures when they are merged together.
This problem occurs because regular GitHub CI workflows are run on the PR's branch
_before it's merged into master_, rather than after.

[semantic merge conflicts]: https://bors.tech/essay/2017/02/02/pitch/

To solve this problem, Bors merges branches by first creating a `staging` branch at the
head of master, then merging your branches into it. This creates a merged branch that
is equivalent to what would previously have been pushed directly to master, except now
there is an opportunity to run CI workflows on this already-merged branch to decide
whether it should be accepted or rejected. If the CI workflow fails, then Bors simply
does not update master. If CI passes, then Bors fast-forwards master to match the
merged commit on the staging branch - the same exact commit which has already been tested.

This style of CI workflow - merging, testing, then fast-forwarding - greatly increases
our confidence in the correctness of the code living in master. As we'll see,
we can also leverage Bors in order to create a Continuous Delivery pipeline in which
every commit to master guarantees the presence of a full set of release artifacts that
have been fully tested and are ready to ship at a moment's notice.
