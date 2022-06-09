---
title: Java SDK Installation Guide
menu: Installation guide
weight: 10
---

The Fluvio Java library should work with any version of Java greater than
version 8, but we have specifically tested it on Java 8, 11, 15, 16, and 17.
In order to see whether you have the Java Development Kit (JDK) installed
and to see what version it is, run the following:

%copy first-line%

```bash
$ javac --version
javac 16.0.1      # You might have something different
```

If you don't already have a JDK installed, [visit the Oracle website] in order
to download it. Choose the link for your platform, then run the installer and
follow the instructions. When the installation is complete, double-check that
you can execute Java from the command line:

[visit the Oracle website]: https://www.oracle.com/java/technologies/javase-jdk16-downloads.html

%copy first-line%

```bash
$ javac --version
```

There are two environment variables we need to make sure are right, your
`PATH` and another one called `JAVA_HOME`. If you were already able to execute
`javac --version`, you can skip this extra PATH setup, but you may still need
to set up JAVA_HOME.

### Setup PATH and JAVA_HOME

If you still can't execute `javac`, you may need to update your PATH.
Make sure you remember where you installed your JDK. We'll give examples of
common locations where it might get installed below. To update these variables,
we'll update a file called `~/.bashrc`.

{{< tabs tabTotal="2" tabID="1" tabName1="Mac" tabName2="Linux">}}

{{< tab tabNum="1" >}}

On Mac, we'll add a line to the end of your `~/.bashrc` that looks something like this:

%copy%

```bash
export PATH="/Library/Java/JavaVirtualMachines/jdk-16.0.1.jdk/Contents/Home/bin:${PATH}"
export JAVA_HOME="/Library/Java/JavaVirtualMachines/jdk-16.0.1.jdk/Contents/Home"
```

{{< /tab >}}

{{< tab tabNum="2" >}}

On Linux, "installing" Java essentially amounts to unzipping the download file in
a particular directory. I tend to put the JDK right into the home directory, e.g.
`~/jdk1.8.0_181/`. Wherever you unzipped your JDK, you'll want to use that path
when setting the following variables in `~/.bashrc`:

%copy%

```bash
export PATH="${HOME}/jdk1.8.0_181/bin:${PATH}"
export JAVA_HOME="${HOME}/jdk1.8.0_181/"
```

{{< /tab >}}

{{< /tabs >}}

-> Note that the `JAVA_HOME` path is essentially the same as the PATH, minus the `bin` at the end.

### Install Gradle

Gradle is a build tool that we'll be using for this project. It will help us to
download the Fluvio library and compile everything correctly. [Visit the gradle website]
to download and install it. When you're done, you should be able to run the
`gradle --version` command.

[Visit the gradle website](https://gradle.org/)