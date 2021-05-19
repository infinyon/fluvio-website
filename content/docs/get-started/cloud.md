---
title: Fluvio Cloud
menu: Cloud
weight: 40
---

Fluvio Cloud is the fastest and easiest way to get started with Fluvio. We'll walk you through the steps of creating a free account and connecting to it with a Fluvio client.

{{<idea>}}
Fluvio Cloud is currently in **alpha** and is not suitable for production environments.
{{</idea>}}

You can reach us on <a href="https://discordapp.com/invite/bBG2dTz" target="_blank">Discord</a> or in <a href="https://github.com/infinyon/fluvio/issues" target="_blank">Github</a>.

## Prerequisites

The Fluvio CLI (_command-line interface_) is an all-in-one tool for setting up, managing, and interacting with Fluvio clusters.

Before getting started, install the [Fluvio CLI]. We'll be using the CLI to interact with your cluster.

[Fluvio CLI]: {{< ref "/download/_index.md" >}}

## Creating a Fluvio Cloud account

Head on over to the  <a href="https://cloud.fluvio.io/signup" target="_blank">Fluvio Cloud signup page</a> to create an account.

<img src="/docs/get-started/images/cloud-signup.png" alt="Fluvio Cloud signup" justify="center" width="400">

After filling out the form, you'll be greeted with a success message telling you to verify your email. You'll need to complete this step in order to continue.

<img src="/docs/get-started/images/cloud-verification.png" alt="Fluvio Cloud verification" justify="center" width="600">

You should get a confirmation that your account is ready to use

<img src="/docs/get-started/images/cloud-confirmation.png" alt="Fluvio Cloud confirmation" justify="center" width="600">

At this point, we can log in via the Fluvio CLI and start sending and receiving messages to your Fluvio cluster. To log in with the CLI, you'll need to run the `fluvio cloud login` command, then type in your email and password when prompted.

```bash
$ fluvio cloud login
Fluvio Cloud email: batman@justiceleague.com
Password:
```

You'll be able to tell that everything worked if your current profile is set to `cloud`. You can check with this command:

```bash
$ fluvio profile current
cloud
```

If you installed fluvio locally it will be listed alongside `cloud`:

```bash
$ fluvio profile view
    PROFILE       CLUSTER       ADDRESS                          TLS 
    local         local         localhost:9003                   Disabled 
 *  cloud         cloud         router.cloud.fluvio.io:9003      Verified
```

-> Use **fluvio profile switch** command to switch between clusters.