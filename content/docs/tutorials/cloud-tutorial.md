---
title: Fluvio Tutorial with InfinyOn Cloud
menu: Fluvio with InfinyOn Cloud Tutorial
weight: 60
---

This is going to be a simple tutorial for setting up InfinyOn Cloud, Fluvio, and
getting used to the interfaces available.

## Why and What?

Fluvio is great for ensuring that data gathered in one location shows up in a
central repository for storage and manipulation! Imagine a dozen sensors all
transmitting data to the same database. Fluvio with its ability to partition
the database, and the ease in which multiple systems can produce to it, would
easily be able to handle the task.

## Basic Setup

There are 3 main steps for setting up for the Cloud.

Installing the CLI, registering for a Cloud account, and finally linking the two together.

### Install Fluvio CLI

Download the Fluvio CLI with the following command.

%copy first-line%
```bash
$ curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

You may need to follow the instructions the installer points out for adding the
software to your `$PATH` environment variable.

### Create InfinyOn Cloud Account

Head over to the [InfinyOn Cloud sign up page](https://infinyon.cloud).

<img src="../images/cloud-signup.jpg"
     alt="A screenshot of the InfinyOn new account form, with Name, Organization, Email, and Password fields"
     style="justify: center; max-width: 300px" />

After filling out the form, you'll be greeted with a success message telling you
to verify your email. You'll need to complete this step in order to continue.
Be careful, there is a time out on the email, and you will have to re-sign up

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

To connect the Cloud to Fluvio, the command `fluvio cloud login` will do the job.
It will ask for your account credential, as seen below:

%copy first-line%
```bash
$ fluvio cloud login
InfinyOn Cloud email: John@example.com
Password:
```

To confirm that everything is linked up properly, please enter the following command.

%copy first-line%
```bash
$ fluvio profile list
     PROFILE   CLUSTER   ADDRESS                     TLS
  *  cloud     cloud     router.infinyon.cloud:9003  Verified
```

## A Small Taste

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

This is currently the easiest way to get data and interact with the Fluvio database.

The bread and butter of Fluvio is `fluvio produce <topic>` and `fluvio consume <topic>`

#### Produce

We saw `fluvio produce` in action above, but here is it again:

%copy first-line%
```bash
$ fluvio produce greetings
> test
Ok!
> meow
Ok!
> Ok!
```

-> To quit `produce`, either `^D` or `^C` works; this example used `^D`.

Produce is the main way to get data into the Fluvio database. While most of the time
you may be calling it through an API, this here is the CLI access to it.

[Read More](/cli/commands/produce)

#### Consume

Another important action to know is `fluvio consume`, seen here:

%copy first-line%
```bash
$ fluvio consume greetings -dB
Consuming records from the beginning of topic 'greetings'
Hello world!
test
meow
```

-> Just like `produce`, `consume` may need to be exited out with `^C`

`fluvio consume` is the main way to read out data from the Fluvio database. Either
in scripting, or through the use of the CLI, most actions will use `consume` in some way

[Read More](/cli/commands/consume)

### InfinyOn Cloud Interface

If you wish to view the messages sent above to the greetings record, you can go
to your Cloud instance.
Here is a quick link to take you to the [greetings records](https://infinyon.cloud/account/clusters/default/topics/greetings/records).

The Cloud interface is still actively being upgraded, so we will only be using
it to passively view what is in the database.

<img src="../images/cloud-overview.jpg"
     alt="A screenshot of the InfinyOn cloud topic."
     style="justify: center; max-width: 500px" />

This is what the Cloud interface looks like for now. It may become even better soon!

## Fluvio in Practice

Here are two simple fluvio projects to learn the basics of what is going on.

### Fluvio in the Terminal

A real simple timestamping comments script!

Obviously a real world example would be more complex, but to start, just a simple
script pushing time and comments to a remote record should do.

{{<code file="/code/zsh/timekeeper.sh" lang="zsh" copy=true >}}

_[TODO: transition zsh to bash]_

This code should be easy follow along with. It generates a string that contains
the current time, and the contents of the first argument passed in.

Once that has happened, it checks to see if the hardcoded topic exists. If it
does, good, it moves on. If not, then it begins calls upon `fluvio topic create`
to generate the new topic. Afterwards, it creates the call to `fluvio produce`
and sends the timestamped message off to the Cloud.


#### Running

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

### More Advanced Python Script

Now that we've gotten comfortable with the CLI, let's have a go at making something
with the APIs available. Python tends to be easy to read, so that is what we shall do.
The following code is something you might find in a larger project. Here it might
be an autosave feature, or maybe it is someone's attempt at not-quite real time
editing collaboration on the same directory.

{{<code file="/code/python/patch-uploader.py" lang="py" copy=true >}}

_[TODO: test code modification A]_

#### Running

This one is a little bit more involved and requires some set up before you can
run it and enjoy the fruits of your labors.

First, we need to create the topic.

%copy first-line%
```bash
$ fluvio topic create patch-autosave
topic "patch-autosave" created
```

Then we need to create the file we want to save and retrieve from the system.

%copy first-line%
```bash
$ echo "test\n123\n456\nI am the very model of a modern major general" > test
```
To show it working, we need to fill the topic with irrelevant data to simulate a
full datebase (in the real world, this would be entirely patch files instead).

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

Well, it looks like something has happened. A quick search of the directory will
show that `test2` now exists! And if we look at the cloud interface, we can see
the contents of the first file, `test`, is now in the database.


<img src="../images/cloud-patch-example.jpg"
     alt="Infinion Cloud with sample data in it."
     style="justify: center; max-width: 500px" />

Clearly in a real world usecase, it would be full of diff patch files, but this
shall do for today's example. Maybe as an exercise you can create your own actual
patch files and try sending them?

## Wrapping up


_[TODO: trim the tutorial, it's too long]_
