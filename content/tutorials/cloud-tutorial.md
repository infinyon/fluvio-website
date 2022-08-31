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

_[TODO: In depth documentation for account creation]_

## InfinyOn Cloud and You

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
$ echo 'hello world!' | fluvio produce greetings

```

The data you store is viewable from both the command line and the Cloud Interface.

## InfinyOn Cloud Interface

_[TODO: introduce cloud interface as read only]_

## Fluvio CLI

_[TODO: introduce fluvio consume]_

# Fluvio in Practice

Here are two simple fluvio projects to learn the basics of what is going on.

## Fluvio in the Terminal

A real simple timestamping comments script!

Obviously a real world example would be more complex, but to start, just a simple
script pushing time and comments to a remote record should do.

{{<code file="/content/tutorials/timekeeper.sh" lang="bash" copy=true >}}

### Running

%copy first-line%
```bash
$ ./timekeeper.sh 'I love cupcakes'
```

If you wish to view the messages your script has sent, go to your cloud instance.
Here is a quick link to take you to the [timekeeper records](https://infinyon.cloud/account/clusters/default/topics/timekeeper/records).

_[explain how to view online]_

## More Advanced Rust Script


_[decide what the program should be]_

_[ideas:]_

* error logging in test software
* diff patches for an autosave?
* 

### Running

%copy first-line%
```bash
$ cargo run
```
