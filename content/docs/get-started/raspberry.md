---
title: Raspberry Pi Installation
menu: Raspberry Pi
weight: 40
---

The Fluvio CLI may be used to produce and consume data from a Raspberry Pi.
To get started, use the installation script below:

{{% inline-embed file="embeds/download-cli/curl-bash-copy.md" %}}

We do not support running a Fluvio Cluster on Raspberry Pi, so in order
to stream data we'll need to connect to an existing cluster. The best way
to do this is by setting up an InfinyOn Cloud account, where you can
create a free Fluvio cluster to quickly start streaming data.

## Creating an InfinyOn Cloud account

Head on over to the <a href="https://infinyon.cloud" target="_blank">InfinyOn Cloud signup page</a> to create an account.

<img src="../images/cloud-signup.jpg"
alt="A screenshot of the InfinyOn new account form, with Name, Organization, Email, and Password fields"
style="justify: center; max-width: 300px" />

After filling out the form, you'll be greeted with a success message telling you to verify your email. You'll need to complete this step in order to continue.

<img src="../images/cloud-verification.jpg"
alt="A screenshot of the verification email received after completing the signup form, including a verification link"
style="justify: center; max-width: 500px" />

You should get a confirmation that your account is ready to use

<img src="../images/cloud-confirmation.jpg"
alt="A screenshot of the prompt received after clicking the verification link, saying the account is ready to use"
style="justify: center; max-width: 300px" />

At this point, we can log in via the Fluvio CLI and start sending and receiving messages to your Fluvio cluster. To log in with the CLI, you'll need to run the `fluvio cloud login` command, then type in your email and password when prompted.

%copy first-line%
```bash
$ fluvio cloud login
Fluvio Cloud email: batman@justiceleague.com
Password:
```

You'll be able to tell that everything worked if your current profile is set to `cloud`. You can check with this command:

%copy first-line%
```bash
$ fluvio profile current
cloud
```

At this point, you should be able to create a topic and produce and consume data!

%copy first-line%
```bash
$ fluvio topic create greetings
topic "greetings" created
```

%copy first-line%
```bash
$ echo "Hello, Fluvio" | fluvio produce greetings
```

%copy first-line%
```bash
$ fluvio consume greetings -B -d
Consuming records from the beginning of topic 'greetings'
Hello, Fluvio
```
