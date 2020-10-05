---
title: Getting Started with Fluvio Cloud
menu: Use Fluvio Cloud
toc: true
weight: 30
---

Fluvio Cloud is the fastest and easiest way to get started with Fluvio.
We'll walk you through the steps of creating a free account and connecting
to it with a Fluvio client.

## Prerequisites

Before getting started, make sure you have the [Fluvio CLI] installed, as
we'll be using that to connect to your account and download your connection
profile.

[Fluvio CLI]: ../fluvio-cli

## Creating a Fluvio Cloud account

Head on over to the [Fluvio Cloud signup page] to create an account.

[Fluvio Cloud signup page]: https://cloud.fluvio.io/signup

{{< image src="getting-started/cloud-signup.png" alt="Fluvio Cloud signup" justify="center" width="400" type="scaled-90" >}}

After filling out the form, you'll be greeted with a success message telling
you to verify your email. You'll need to complete this step in order to
continue.

{{< image src="getting-started/cloud-verification.png" alt="Fluvio Cloud verification" justify="center" width="600" type="scaled-90" >}}

You should get a confirmation that your account is ready to use

{{< image src="getting-started/cloud-confirmation.png" alt="Fluvio Cloud confirmation" justify="center" width="600" type="scaled-90" >}}

At this point, we can log in via the Fluvio CLI and start sending and receiving
messages to your Fluvio cluster. To log in with the CLI, you'll need to run the
`fluvio profile cloud sync` command, then type in your email and password when
prompted.

```bash
$ fluvio profile sync cloud --remote=https://cloud.fluvio.io
Fluvio Cloud email: batman@justiceleague.com
Password:
```

You'll be able to tell that everything worked if your current profile is set to
`fluvio-cloud`. You can check with this command:

```bash
$ fluvio profile current
fluvio-cloud
```

## Hello, Fluvio!

Congratulations, you've successfully set up Fluvio Cloud! Let's use the Fluvio CLI
to play with some basic functionality.

The first thing we need to do is create a Fluvio topic. A Topic is like a category
where related events live together. We can create a new topic with the following
command:

```bash
$ fluvio topic create greetings
topic "greetings" created
```

Now that we have a topic, we can produce some messages! Use the following command
to send a message to the `greetings` topic:

```bash
$ echo "Hello, Fluvio" | fluvio produce greetings
Ok!
```

Finally, we can consume messages back from the topic

```bash
$ fluvio consume greetings -B -d
Hello, Fluvio
```

Way to go! You're well on your way to writing real-time distributed apps with Fluvio!
Next, check out our [Tutorials page] to see real-world examples of Fluvio in action.

[Tutorials page]: /tutorials

#### Related Topics
---

- ["Hello World" in Node.js](/tutorials/hello-world-node/)
- ["Hello World" in Rust](/tutorials/hello-world-rust/)
