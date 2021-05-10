---
title: Increasing our development confidence and productivity with Bors
author:
    name: "Nick Mosher"
    github: "nicholastmosher"
description: "TBD"
date: 2021-04-20
slug: github-actions-best-practices
url: /blog/2021/05/bors
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

## Hands-on: The Bors workflow

To give you a more concrete sense of how Bors operates, let me walk you through the
experience as a developer using Bors on a day-to-day basis. Essentially, we follow
these steps when working on a PR:

- Push code changes to your branch and open a PR
- Ensure your CI jobs are in a passing state
- Get reviews and approvals from team members
- When ready to merge, write a comment with the text "bors r+"

TODO: IMAGE OF BORS R+

Notice that you still have to supply your own CI job definition, and that your CI may
run each time new commits are pushed to a branch. The only difference in the development
process is when it comes time to actually merge the branch. Instead of using GitHub's
big green "Merge/Squash/Rebase" button, we simply tell Bors that we think this PR is
ready to merge.

When we say "bors r+", we tell Bors to add this PR to the "ready queue".
When there are one or more PRs in the ready queue, Bors will attempt to batch together
all the ready PRs, merge them into the `staging` branch (which, remember, begins at the
head of master), and run CI once again on the _merged_ staging branch. Bors will watch
the status of the CI jobs, and once all the required jobs have passed, it will push
the staging branch to master, which is guaranteed to be a fast-forward.

IMAGE OF BORS MERGE

## Increased Productivity

I want to touch on one of the nice side effects of using Bors to merge PRs. It has actually
helped us to reduce the amount of time we spend on preparing and babysitting PRs. Prior to
using Bors, one of the strategies we used to avoid semantic merge conflicts was to require
all branches to be "up-to-date with master" before merging. This is enforceable by GitHub
and essentially means that you need to rebase against master any time another change lands
before yours does. Because of this, we would often find ourselves trapped in a ruthless cycle:

- Get the PR tested, approved, and ready to go
- Get ready to press the Big Green Merge Button
- Find out another PR was merged first and need to rebase

This was especially painful because after rebasing, we would need to once again wait on our
CI jobs to pass and hope that we don't get beaten to the merge again. One way to avoid this
problem would have been to coordinate with team members before trying to merge PRs, but that
requires more time and synchronization across the entire team, and does not scale well.

Using Bors allows us to sidestep these issues entirely by simply letting it manage the merging
process. After sending "bors r+" on a PR, you can usually move on and work on the next thing
without needing to keep it in the back of your mind. The exception to this is if Bors finds
a merge conflict or semantic conflict between your PR and another one that came before yours in
the queue. Note, however, that in this scenario you already would have needed to fix regular
merge conflicts, and that Bors provides the benefit of notifying you when a semantic conflict
causes a failure, which previously would have failed after reaching master rather than before.
