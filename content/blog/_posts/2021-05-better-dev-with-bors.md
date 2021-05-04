---
title: Speeding up our development cycle with Bors
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
our development speed and confidence.

[Bors-ng]: https://bors.tech
