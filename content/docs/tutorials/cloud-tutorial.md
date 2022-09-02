---
title: Fluvio Tutorial [Work in Progress]
menu: Fluvio with InfinyOn Cloud Tutorial
weight: 60
---

## Why and What?

Fluvio is great for ensuring that data gathered from multiple sources shows
up in a central repository for storage and manipulation! Imagine a dozen sensors
all transmitting data to the same database. Fluvio, with its ability to partition
the database, would be able to handle the task with ease.

This tutorial will walk you through setting up an InfinyOn account, and using
Fluvio. Both as a standalone command, and as part of a larger program.

_[rewrite]_

## Basic Setup

There are 3 main steps for setting up for the Cloud: Installing the CLI,
registering for a Cloud account, and finally linking the two together.

The next section willl walk you through how to do that.

### Install Fluvio CLI

Download the Fluvio CLI with the following command.

%copy first-line%
```bash
$ curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

Follow the instructions in the installer to add Fluvio to your `$PATH`
environment variable.

### Create InfinyOn Cloud Account

Head over to the [InfinyOn Cloud sign up page](https://infinyon.cloud).

<img src="../images/cloud-signup.jpg"
     alt="A screenshot of the InfinyOn new account form, with Name, Organization, Email, and Password fields"
     style="justify: center; max-width: 300px" />

After filling out the form, you'll be greeted with a success message telling you
to verify your email. You'll need to complete this step in order to continue.

~> The link in the email will time out after a few minutes.

<img src="../images/cloud-verification.jpg"
     alt="A screenshot of the verification email received after completing the signup form, including a verification link"
     style="justify: center; max-width: 500px" />

You should get a confirmation that your account is ready to use.

<img src="../images/cloud-confirmation.jpg"
     alt="A screenshot of the prompt received after clicking the verification link, saying the account is ready to use"
     style="justify: center; max-width: 300px" />


At this point, you can log in via the Fluvio CLI and start sending and receiving
messages to your Fluvio cluster.

### Link InfinyOn Cloud to Fluvio CLI

Use the command `fluvio cloud login` to connect the InfinyOn Cloud to your
Fluvio CLI. It will ask for your account credentials, as seen below.

%copy first-line%
```bash
$ fluvio cloud login
InfinyOn Cloud email: John@example.com
Password:
```

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

The data you store is viewable from both the command line through Fluvio CLI and online
through the Cloud interface.

### Fluvio CLI

The Fluvio CLI is currently the easiest way to interact with the Fluvio database.

Two core commands of Fluvio you will want to be familiar with are `fluvio produce <topic>` and `fluvio consume <topic>`

_[flesh out information on Produce and Consume]_

#### Produce

`fluvio produce` is the main way to get data into the Fluvio database. While most of the time
you may be calling it through an API, this here is the CLI access to it.

%copy first-line%
```bash
$ fluvio produce greetings
> test
Ok!
> meow
Ok!
>
```

-> When quitting the interactive mode of `produce` press `^C`.

`fluvio produce <topic> [flags]` takes in either input from stdin, or from a file. The
stdin input can be piped into, as seen [in this tutorial](#fluvio-in-shell-scripting),
or filled interactively as seen just above. The above example is an example
of `produce`'s "interactive" mode where it reads from stdin until told to stop.

Some useful option to be aware of:

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

-> To quit the continuous mode of `consume` press `^C`.

`fluvio consume <topic> [flags]` by default prints to the terminal new records as
they appear. By default it runs nonstop until quit, the examples used here all
use the `-d` flag to tell it to stop.

Some useful option flags to be aware of:

* `d` – to halt consumption after reaching the end of the records available.
* `T[int]` – to consume only the T(default 10) most recent records.
* `B[int]` – to consume records B(default 0) after the start of the database.


### InfinyOn Cloud Interface

If you wish to view the messages sent above to the greetings record, you can go
to your Cloud instance.

If you are logged into your InfinyOn Cloud account, this quick link will take you
to the [greetings records](https://infinyon.cloud/account/clusters/default/topics/greetings/records).

The Cloud interface is still actively being upgraded, so we will only be using
it to passively view what is in the database.

<img src="../images/cloud-overview.jpg"
     alt="A screenshot of the InfinyOn cloud topic."
     style="justify: center; max-width: 500px" />

This is what the Cloud interface looks like for now. Stand by for an improved
interface!

## Fluvio in Action

Here are two simple Fluvio projects to learn the basics of what is going on.

### Fluvio in Shell Scripting

Here is a simple script that pushes timestamped comments to a remote database.

This code generates a string that contains the current time, and the contents of the first argument passed in.

Once that has happened, it checks to see if the hardcoded topic exists. If it
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

Again, you can use either `consume` or the Cloud interface to view the results.
Here we will just check it with the CLI.

%copy first-line%
```bash
$ fluvio consume timekeeper -dB
Consuming records from the beginning of topic 'timekeeper'
2022-09-01T18:46:12-07:00 : I love cupcakes
```

### A More Advanced Script — Python

Now that we've gotten comfortable with the CLI, let's have a go at making something
with the avaliable APIs. This script has two functions. The first takes a file
and the current time, and wraps it up as a JSON object. It then uploads the object
as a single record to the `patch-autosave` record. The second function reads in
the last five records from the database, and converts them to a list of JSON objects.
Once done, it saves the most recent record to a file. A quick `assert` shows that
both the original and new files contain the same data.

_[TODO: rewrite above to say what it *does* do]_

{{<code file="/code/python/patch-uploader.py" lang="py" copy=true >}}

##### To Run the Script

This one is a little bit more involved and requires some setup before you can
run it and enjoy the fruits of your labors.

First, we'll save the script to a file, here it's named `patch-uploader.py`.

Second, we need to create the topic.

%copy first-line%
```bash
$ fluvio topic create patch-autosave
topic "patch-autosave" created
```

Then we need to create the test file we want to save and retrieve from the system.

%copy first-line%
```bash
$ echo "test\n123\n456\nI am the very model of a modern major general" > test
```
Last step before running the script, we need to fill the topic with placeholder
data to simulate a fuller database (in the real world, this would be actual data
– patch files – instead).

%copy first-line%
```bash
fluvio produce patch-autosave
> 123!
Ok!
> test!
Ok!
> It is best to sleep it off
Ok!
> 457
Ok!
> explosion!
Ok!
```

Now, let us see if it works!

%copy first-line%
```bash
$ python ./patch-uploader.py
```

A quick search of the directory will show that the file `test2` now exists!
And if we look at the cloud interface, we can see the contents of the first
file, `test`, is now in the database.


<img src="../images/cloud-patch-example.jpg"
     alt="Infinion Cloud with sample data in it."
     style="justify: center; max-width: 500px" />

As extra credit you can create your own actual patch files and send them.

## Wrapping up

This has been a short introduction

_[TODO: trim the tutorial, it's too long]_

### References:

[Fluvio CLI Produce](/cli/commands/produce)

[Fluvio CLI Consume](/cli/commands/consume)

[Fluvio CLI topic](/cli/commands/topic)

[Fluvio CLI profile](/cli/installation/profile)
