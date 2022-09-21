---
title: Setup InfinyOn Cloud
menu: Setup InfinyOn Cloud
weight: 10
---

This tutorial will walk you through setting up an InfinyOn account, and using Fluvio.

## Basic Setup

<img src="../images/setup-cloud.png"
     alt="Signup flow for InfinyOn Cloud"
	 style="justify: center; max-width: 400px" />

There are three main steps for setting up for the Cloud:
* Installing the CLI
* registering for a Cloud account
* finally linking the two together

The following sections will walk you through how to do that.

### Install Fluvio CLI

Download the Fluvio CLI with the following command.

%copy first-line%
```bash
$ curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

{{< inline-embed file="path-env.md" >}}

### Create InfinyOn Cloud Account
Head over to the [InfinyOn Cloud sign up page](https://infinyon.cloud).

Currently, there are two mutually exclusive ways to create your Fluvio account:
Google sign in, and Email registration.

{{<h-list tabTotal="2" tabID="2" tabName1="Signup with Google" tabName2="Username & Password">}}

{{<h-item tabNum="1">}}


If you wish to avoid creating a new username and password for InfinyOn Cloud,
then click on the blue **Sign up with Google** button.

<img src="../images/google-signup-part1.jpg"
     alt="A screenshot of step one of Signing up to InfinyOn with Google — click on the sign in button"
     style="justify: center; max-width: 340px" />

The link will take you to a Google login prompt if you are not already logged into Google.

<img src="../images/google-signup-part2.jpg"
     alt="a screenshot of step two of Signing up to InfinyOn with Google — associate account"
     style="justify: center; max-width: 340px" />

Finally, Google will provide a confirmation screen confirming that you want to
log into InfinyOn Cloud with your Google account.

{{</h-item >}}

{{<h-item tabNum="2" >}}

Please fill out the form with the requested information: Name, Email address,
and Password to start the account creation process.

<img src="../images/cloud-signup.jpg"
     alt="A screenshot of the InfinyOn new account form, with Name, Organization, Email, and Password fields"
     style="justify: center; max-width: 340px" />

After filling out the form, you'll be greeted with a success message requesting
that you confirm your email address. You'll need to complete this step in order
to continue.

~> The link in the email will time out after a few minutes.

<img src="../images/cloud-verification.jpg"
     alt="A screenshot of the verification email received after completing the signup form, including a verification link"
     style="justify: center; max-width: 500px" />

You should now get a confirmation that your account is ready to use.

<img src="../images/cloud-confirmation.jpg"
     alt="A screenshot of the prompt received after clicking the verification link, saying the account is ready to use"
     style="justify: center; max-width: 340px" />

At this point, you can log in via the Fluvio CLI and start sending and receiving
messages to your Fluvio cluster.

{{</h-item>}}

{{</h-list>}}

### Link InfinyOn Cloud to Fluvio CLI

Depending which method you chose in account creation, you will have the option of
logging in with OAuth2, or username/password. Please follow the relevant steps
to link the Cloud to your Fluvio CLI.

{{<h-list tabTotal="2" tabID="3" tabName1="Google Oauth" tabName2="Username & Password">}}

{{<h-item tabNum="1">}}


Use the command `fluvio cloud login --use-oauth2` to connect to the InfinyOn Cloud.

%copy first-line%
```bash
$ fluvio cloud login --use-oauth2
A web browser has been opened at https://infinyon-cloud.us.auth0.com/activate?user_code=GLMC-QDDJ.
Please proceed with authentication.
```

Fluvio will open a login screen in your browser. If it is unable to do so, copy the
URL provided in the command prompt and enter it into your web browser manually.

Verify that the code matches what was displayed at the end of the URL in the
command prompt, then click the **Confirm** button.

<img src="../images/google-oauth-screen1.jpg"
     alt="screenshot showing verification code that Fluvio is trying to connect with."
     style="justify: center; max-width: 340px" />

If you are not already logged into Google, the next step is to sign in.

<img src="../images/google-oauth-screen2.jpg"
     alt="screenshot showing Google requesting you log in with your google account"
     style="justify: center; max-width: 340px" />

One last verification that you wish to authorize InfinyOn Cloud, click **Accept**.

<img src="../images/google-oauth-screen3.jpg"
     alt="screenshot showing OAuth2 requesting access to your Google account"
     style="justify: center; max-width: 340px" />

Congrats, everything is now set up!

<img src="../images/google-oauth-screen4.jpg"
     alt="screenshot showing confirmation that OAuth2 connection is complete"
     style="justify: center; max-width: 340px" />


{{</h-item>}}

{{<h-item tabNum="2">}}

Use the command `fluvio cloud login` to connect the InfinyOn Cloud to your
Fluvio CLI. It will ask for your account credentials, as seen below.

%copy first-line%
 ```bash
$ fluvio cloud login
InfinyOn Cloud email: John@example.com
Password:
```


{{</h-item>}}

{{</h-list>}}

## Checking that Fluvio works

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

The storage and retrieval of records from the topic as shown above are basic
actions of the Fluvio CLI.

Two core commands of Fluvio you will need to be familiar with are
`fluvio produce <topic>` and `fluvio consume <topic>`.

#### Produce

`fluvio produce` is the main way to get data into the Fluvio database. While
most of the time you may be calling it through an API, here is how to access it
through the CLI.

%copy first-line%
```bash
$ fluvio produce greetings
> test
Ok!
> meow
Ok!
>
```

-> To quit `produce` press `CTRL-C`.

`fluvio produce <topic> [flags]` can take input from stdin, or from a file. The
stdin input can be piped into, [as shown in this tutorial](#an-easy-fluvio-script--bash),
or filled continuously as seen just above. When taking input continuously, it
won't stop until it is terminated by the user.

Some useful option flags to be aware of:

* `-f <file name>`			– uses a file as input to be read and uploaded as multiple records.
* `--raw`					– specifies that the incoming data should be stored as a single record.
* `--key-separator <key>`	– specifies the string delineator to split the input into a key and value pairing.

#### Consume

`fluvio consume` is the main way to read data from the Fluvio database. Either
in scripting, or through the use of the CLI, most interactions will use `consume` in some way.

Here is an example of `consume` in action:

%copy first-line%
```bash
$ fluvio consume greetings -dB
Consuming records from the beginning of topic 'greetings'
Hello world!
test
meow
```

-> To quit `consume` press `CTRL-C`.

`fluvio consume <topic> [flags]` by default prints new records to the terminal as
they enter the database. By default it runs nonstop until quit – the examples
used here all use the `-d` flag to tell it to stop.

Some useful option flags to be aware of:

* `d`						– halts consumption after reaching the end of the records available.
* `T[int]`					– consumes only the T (default 10) most recent records.
* `B[int]`					– starts consuming records B (default 0) after the start of the database.
* `-p[int]`					– reads only from the partition p (default 0).
* `-k`						– displays the key portion of the key and value pairs.
* `-A`						– reads from all partitions available.
* `--smart-module <module>` – runs a [SmartModule](#smartmodules) module on the output, then displays the results.


Now you have a fully setup Fluvio system, and you know the basic commands to get Fluvio working.

## Check out these Other Tutorials

[Creating a Data Pipeline](../data-pipeline/)

## References

[Fluvio CLI Produce](/cli/commands/produce/)

[Fluvio CLI Consume](/cli/commands/consume/)

[Fluvio CLI topic](/cli/commands/topic/)

[Fluvio CLI profile](/cli/installation/profile/)
