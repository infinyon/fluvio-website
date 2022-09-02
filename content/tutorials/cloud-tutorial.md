---
title: InfinyOn Cloud Tutorial with Fluvio
menu: Cloud and You
weight: 60
---


This is going to be a simple tutorial for setting up InfinyOn, and getting used
to the Cloud interface.

## Why and What?

Fluvio is great for ensuring that data gathered in one location shows up in a
central repository for storage and manipulation!

# Basic Setup

There are 3 main steps for setting up for the Cloud.

_[TODO: In depth documentation for account creation]_

### Install Fluvio CLI

The [Download](/download) link has the instructions for downloading and installing
the Fluvio CLI. For our example today of using the Cloud, this is all you need
from this section.

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

You can see it in action by creating a simple topic, and pushing data to it:

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

## Fluvio CLI

This is currently the easiest way to get data and interact with the Fluvio database.

The bread and butter of Fluvio is `fluvio produce <topic>` and `fluvio consume <topic>`

### Produce

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

_[TODO: explain what produce does]_

Produce is the main way to get data into the Fluvio database

### Consume

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

## InfinyOn Cloud Interface

If you wish to view the messages sent above to the greetings record, you can go
to your Cloud instance.
Here is a quick link to take you to the [greetings records](https://infinyon.cloud/account/clusters/default/topics/greetings/records).

The Cloud interface is still actively being upgraded, so we will only be using
it to passively view what is in the database.

<img src="../images/cloud-overview.jpg"
     alt="A screenshot of the InfinyOn cloud topic."
     style="justify: center; max-width: 500px" />

This is what the Cloud interface looks like right now

# Fluvio in Practice

Here are two simple fluvio projects to learn the basics of what is going on.

## Fluvio in the Terminal

A real simple timestamping comments script!

Obviously a real world example would be more complex, but to start, just a simple
script pushing time and comments to a remote record should do.

{{<code file="/code/zsh/timekeeper.sh" lang="zsh" copy=true >}}

_[TODO: explain what this code does]_

### Running

%copy first-line%
```bash
$ ./timekeeper.sh 'I love cupcakes'
```

Again, you can use either `consume` or the Cloud interface to view the results.

## More Advanced Python Script

{{<code file="/code/python/patch-uploader.py" lang="py" copy=true >}}

### Running

This one is a little bit more involved and requires some set up before you can run it and enjoy the fruits of your labors.

First, we need to create the topic.

%copy first-line%
```bash
$ fluvio topic create patch-autosave
topic "patch-autosave" created
```

Then we need to create the file we want to save and retrieve from the system:

%copy first-line%
```bash
$ echo "test\n123\n456\nI am the very model of a modern major general" > test
```
To show it working, we shall fill the topic with irrelevant data (in the real world, this would be entirely patch files instead.)

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

Now, let us see what the script does!

%copy first-line%
```bash
$ python ./patch-uploader.py
```

Nothing?

Not quite, a quick search of the directory will show that `test2` now exists!
and if we look at the cloud interface, we can see the contents of the first
file, `test`, is now in the database.


<img src="../images/cloud-patch-example.jpg"
     alt="Infinion Cloud with sample data in it."
     style="justify: center; max-width: 500px" />

Clearly in a real world usecase, it would be full of diff patch files, but this shall do for today's example.
