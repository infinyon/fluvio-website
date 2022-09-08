---
title: Using InfinyOn Cloud Backend — Connectors and SmartModules
menu: InfinyOn Cloud Tutorial
weight: 60
---

This tutorial will walk you through setting up an InfinyOn account, using Fluvio,
and learning the basics of connectors and SmartModules.

## Basic Setup

<img src="../images/Fluvio-Cloud-Flow.svg"
     alt="Signup flow for InfinyOn Cloud"
	 style="justify: center; max-width: 500px" />
	 
There are 3 main steps for setting up for the Cloud: Installing the CLI,
registering for a Cloud account, and finally linking the two together.

The following sections will walk you through how to do that.

### Install Fluvio CLI

Download the Fluvio CLI with the following command.

%copy first-line%
```bash
$ curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

{{< partial file="docs/path-env.html" >}}


### Create InfinyOn Cloud Account
Head over to the [InfinyOn Cloud sign up page](https://infinyon.cloud).

Currently, there are two mutually exclusive ways to create your Fluvio account:
Google sign in, and Email registration.

##### Log in with Google

If you wish to in the future sign in with OAuth2, then click on the `Sign in with Google` button.

<img src="../images/google-signup-part1.jpg"
     alt="A screenshot of step one of Signing up to InfinyOn with Google — click on the sign in button"
     style="justify: center; max-width: 300px" />

The link will take you to a Google login prompt and ask if you are sure you want
to log into InfinyOn Cloud.

<img src="../images/google-signup-part2.jpg"
     alt="a screenshot of step two of Signing up to InfinyOn with Google — associate account"
     style="justify: center; max-width: 300px" />

##### Create an InfinyOn Cloud Account

<img src="../images/cloud-signup.jpg"
     alt="A screenshot of the InfinyOn new account form, with Name, Organization, Email, and Password fields"
     style="justify: center; max-width: 300px" />

After filling out the form, you'll be greeted with a success message telling you
to verify your email. You'll need to complete this step in order to continue.

~> The link in the email will time out after a few minutes.

<img src="../images/cloud-verification.jpg"
     alt="A screenshot of the verification email received after completing the signup form, including a verification link"
     style="justify: center; max-width: 500px" />

You should now get a confirmation that your account is ready to use.

<img src="../images/cloud-confirmation.jpg"
     alt="A screenshot of the prompt received after clicking the verification link, saying the account is ready to use"
     style="justify: center; max-width: 300px" />


At this point, you can log in via the Fluvio CLI and start sending and receiving
messages to your Fluvio cluster.

### Link InfinyOn Cloud to Fluvio CLI

Depending which method you chose to create your account, please follow the relevant 
instructions below.

##### Connect with OAuth2

Use the command `fluvio cloud login --use-oauth2` to connect to the InfinyOn Cloud. 

%copy first-line%
```bash
$ fluvio cloud login --use-oauth2
A web browser has been opened at https://infinyon-cloud.us.auth0.com/activate?user_code=GLMC-QDDJ.
Please proceed with authentication.
```

It will open a login screen in your webbrowser if possible. If that is not
possible, it will print a URL to you to enter into your web browser.

<img src="../images/google-login-part1.jpg"
     alt="screenshot showing verification code that Fluvio is trying to connect with."
     style="justify: center; max-width: 300px" />

Verify that the code matches what was displayed at the end of the url in the
command prompt, then click the confirm button.

<img src="../images/google-login-part2.jpg"
     alt="screenshot showing Google requesting you log in with your google account"
     style="justify: center; max-width: 300px" />
	 
Next you will have to log into google. If you are already signed in, this step 
is automatically skipped.

<img src="../images/google-login-part3.jpg"
     alt="screenshot showing OAuth2 requesting access to your Google account"
     style="justify: center; max-width: 300px" />

One last verification that you wish to connect to InfinyOn Cloud, click accept.

 <img src="../images/google-login-confirm.jpg"
     alt="screenshot showing confirmation that OAuth2 connection is complete"
     style="justify: center; max-width: 300px" />

Congrats, everything is now set up!

##### Connect with username and password

Use the command `fluvio cloud login` to connect the InfinyOn Cloud to your
Fluvio CLI. It will ask for your account credentials, as seen below.
 
 %copy first-line%
 ```bash
$ fluvio cloud login
InfinyOn Cloud email: John@example.com
Password:
```

##### Confirming connection
Use the `fluvio profile list` command to confirm that your CLI is linked to the
InfinyOn Cloud instance.
 
%copy first-line%
```bash
$ fluvio profile list
     PROFILE   CLUSTER   ADDRESS                     TLS
  *  cloud     cloud     router.infinyon.cloud:9003  Verified
```

## A Quick Introduction to the Fluvio Interface

You can now see Fluvio in action by creating a simple topic, and pushing data to it:

%copy first-line%
```bash
$ fluvio topic create greetings
topic "greetings" created
```

%copy first-line%
```bash
$ echo 'Hello world!' | fluvio produce greetings

```

%copy first-line%
```bash
$ fluvio consume greetings -dB
Hello world!

```

The data you store is viewable from the command line through Fluvio CLI.

### Fluvio CLI

The Fluvio CLI is currently the easiest way to interact with the Fluvio database.

Two core commands of Fluvio you will want to be familiar with are `fluvio produce <topic>` and `fluvio consume <topic>`

#### Produce

`fluvio produce` is the main way to get data into the Fluvio database. While most of the time
you may be calling it through an API, here is how to access it through the CLI.

%copy first-line%
```bash
$ fluvio produce greetings
> test
Ok!
> meow
Ok!
>
```

-> To quit the continuous mode of `produce` press `CTRL-C`.

`fluvio produce <topic> [flags]` takes in either input from stdin, or from a file. The
stdin input can be piped into [like in this tutorial](#an-easy-fluvio-script--bash),
or filled continuously as seen just above. When running in continuous mode, it won't stop
until it is killed by the user.

Some useful option flags to be aware of:

* `-f <file name>` – to use a file as input to be read and uploaded as multiple records.
* `--raw` – to specify that the incoming data should be stored as a single record.

#### Consume

`fluvio consume` is the main way to read out data from the Fluvio database. Either
in scripting, or through the use of the CLI, most actions will use `consume` in some way.

Here is an example of Consume in action:

%copy first-line%
```bash
$ fluvio consume greetings -dB
Consuming records from the beginning of topic 'greetings'
Hello world!
test
meow
```

-> To quit the continuous mode of `consume` press `CTRL-C`.

`fluvio consume <topic> [flags]` by default prints to the terminal new records as
they appear. By default it runs nonstop until quit, the examples used here all
use the `-d` flag to tell it to stop.

Some useful option flags to be aware of:

* `d` – to halt consumption after reaching the end of the records available.
* `T[int]` – to specify that it should consume only the T(default 10) most recent records.
* `B[int]` – to specify to start consuming records B(default 0) after the start of the database.

## An Easy Fluvio Script — Bash

Here is a simple script that pushes timestamped comments to a remote database.

This code generates a string that contains the current time, and the contents of the first argument passed in.

Once that has happened, it checks to see if the hard coded topic exists. If it
does, good, it moves on. If not, then it calls upon `fluvio topic create`
to generate the new topic. Afterwards, it issues the call to `fluvio produce`
and sends the timestamped message off to the Cloud.

{{<code file="/code/bash/timekeeper.sh" lang="bash" copy=true >}}

##### To Run the Script

Save the above script to a file – here it is named `timekeeper.sh` – and set it
as executable.

You can then run the script with:

%copy first-line%
```bash
$ ./timekeeper.sh 'I love cupcakes'
topic "timekeeper" created
```

Again, you can use `consume` to view the results.

%copy first-line%
```bash
$ fluvio consume timekeeper -dB
Consuming records from the beginning of topic 'timekeeper'
2022-09-01T18:46:12-07:00 : I love cupcakes
```

## Connectors

_[TODO: introduce Connectors]_

### Adding Connectors to the Script

## SmartModules

_[TODO: Smart Filter introduction]_

### Adding SmartModules to the script

## Check out these Other Tutorials


### References:

[Fluvio CLI Produce](/cli/commands/produce)

[Fluvio CLI Consume](/cli/commands/consume)

[Fluvio CLI topic](/cli/commands/topic)

[Fluvio CLI profile](/cli/installation/profile)
