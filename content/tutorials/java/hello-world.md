---
title: '"Hello, World! 🎉" in Java'
toc: true
---
{{< lang-selector >}}

In this tutorial, you will learn how to create a topic, build a producer/consumer in Java, and sends a "Hello, World! 🎉" message.

## Prerequisites



### Create a Topic using the Fluvio CLI

In Fluvio, we send all of our messages to something called a Topic, which
is like a category for related messages. For this tutorial, we'll create
a topic called `hello-java` using the following command:

```bash
$ fluvio topic create hello-python
```

### Check Java

The Fluvio Java library should work with any version of Java greater than
version 8, but we have specifically tested it on Java 8, 11, 15, and 16.
In order to see whether you have the Java Development Kit (JDK) installed
and to see what version it is, run the following:

```bash
$ javac --version
javac 16.0.1      # You might have something different
```

If you have `javac` > 8.x installed, feel free to skip the next step!

#### Install Java and Gradle

If you don't already have a JDK installed, [visit the Oracle website] in order
to download it. Choose the link for your platform, then run the installer and
follow the instructions. When the installation is complete, double-check that
you can execute Java from the command line:

[visit the Oracle website]: https://www.oracle.com/java/technologies/javase-jdk16-downloads.html

```bash
$ javac --version
```

There are two environment variables we need to make sure are right, your
`PATH` and another one called `JAVA_HOME`. If you were already able to execute
`javac --version`, you can skip this extra PATH setup, but you may still need
to set up JAVA_HOME.

##### Setup PATH and JAVA_HOME

If you still can't execute `javac`, you may need to update your PATH.
Make sure you remember where you installed your JDK. We'll give examples of
common locations where it might get installed below. To update these variables,
we'll update a file called `~/.bashrc`.

On Mac, we'll add a line to the end of your `~/.bashrc` that looks something like this:

```bash
export PATH="/Library/Java/JavaVirtualMachines/jdk-16.0.1.jdk/Contents/Home/bin:${PATH}"
export JAVA_HOME="/Library/Java/JavaVirtualMachines/jdk-16.0.1.jdk/Contents/Home"
```

On Linux, "installing" Java essentially amounts to unzipping the download file in
a particular directory. I tend to put the JDK right into the home directory, e.g.
`~/jdk1.8.0_181/`. Wherever you unzipped your JDK, you'll want to use that path
when setting the following variables in `~/.bashrc`:

```bash
export PATH="${HOME}/jdk1.8.0_181/bin:${PATH}"
export JAVA_HOME="${HOME}/jdk1.8.0_181/"
```

Note that the `JAVA_HOME` path is essentially the same as the PATH, minus the `bin` at the end.

### Install Gradle

Gradle is a build tool that we'll be using for this project. It will help us to
download the Fluvio library and compile everything correctly. [Visit the gradle website]
to download and install it. When you're done, you should be able to run the
`gradle --version` command.

## Setting up the project

Now we're going to use `gradle` to help us set up the project. Let's create a
new directory for the project and run `gradle init` inside.

```bash
$ mkdir fluvio-java-app && cd fluvio-java-app
$ gradle init

Select type of project to generate:
  1: basic
  2: application
  3: library
  4: Gradle plugin
Enter selection (default: basic) [1..4] 2

Select implementation language:
  1: C++
  2: Groovy
  3: Java
  4: Kotlin
  5: Scala
  6: Swift
Enter selection (default: Java) [1..6] 3

Split functionality across multiple subprojects?:
  1: no - only one application project
  2: yes - application and library projects
Enter selection (default: no - only one application project) [1..2] 1

Select build script DSL:
  1: Groovy
  2: Kotlin
Enter selection (default: Groovy) [1..2] 1

Select test framework:
  1: JUnit 4
  2: TestNG
  3: Spock
  4: JUnit Jupiter
Enter selection (default: JUnit 4) [1..4] 1

Project name (default: java): fluvio-java-app
Source package (default: fluvio.java.app):

> Task :init
Get more help with your project: https://docs.gradle.org/7.0/samples/sample_building_java_applications.html

BUILD SUCCESSFUL in 31m 43s
2 actionable tasks: 2 executed
```

This creates a handful of files and directories for us, let's take a look at them.

```bash
$ ls -la
drwxr-xr-x  10 nick  wheel   320 Apr 28 16:51 .
drwxrwxrwt  24 root  wheel   768 Apr 28 16:50 ..
-rw-r--r--   1 nick  wheel   154 Apr 28 16:51 .gitattributes
-rw-r--r--   1 nick  wheel   103 Apr 28 16:51 .gitignore
drwxr-xr-x   7 nick  wheel   224 Apr 28 16:51 .gradle
drwxr-xr-x   4 nick  wheel   128 Apr 28 16:51 app
drwxr-xr-x   3 nick  wheel    96 Apr 28 16:19 gradle
-rwxr-xr-x   1 nick  wheel  5766 Apr 28 16:19 gradlew
-rw-r--r--   1 nick  wheel  2763 Apr 28 16:19 gradlew.bat
-rw-r--r--   1 nick  wheel   377 Apr 28 16:51 settings.gradle
```

It also created a main Java file for us, let's take a look at that too:

```bash
$ cat app/src/main/java/fluvio/java/app/App.java
/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package fluvio.java.app;

public class App {
    public String getGreeting() {
        return "Hello World!";
    }

    public static void main(String[] args) {
        System.out.println(new App().getGreeting());
    }
}
```

Gradle has set up a simple "Hello World" program so that we can test out the
project setup. Let's make sure we can run it. To execute this program, run the following:

```bash
$ ./gradlew run
Starting a Gradle Daemon, 1 incompatible Daemon could not be reused, use --status for details

> Task :app:run
Hello World!

BUILD SUCCESSFUL in 7s
2 actionable tasks: 2 executed
```

-> After the initial `gradle init`, we will run all of our Gradle commands using `./gradlew <command>`

Awesome, now that we have the base Java project set up, let's see how we can start
using Fluvio in our application.

### Installing Project Dependencies

In order to use the Fluvio library, we need to add it to our project. We can do that
by editing the `app/build.gradle` file. Let's take a look at it now:

```groovy
/*
 * This file was generated by the Gradle 'init' task.
 *
 * This generated file contains a sample Java application project to get you started.
 * For more details take a look at the 'Building Java & JVM projects' chapter in the Gradle
 * User Manual available at https://docs.gradle.org/7.0/userguide/building_java_projects.html
 */

plugins {
    // Apply the application plugin to add support for building a CLI application in Java.
    id 'application'
}

repositories {
    // Use Maven Central for resolving dependencies.
    mavenCentral()
}

dependencies {
    // Use JUnit test framework.
    testImplementation 'junit:junit:4.13.1'

    // This dependency is used by the application.
    implementation 'com.google.guava:guava:30.0-jre'
}

application {
    // Define the main class for the application.
    mainClass = 'fluvio.java.app.App'
}
```

We need to do two things to add Fluvio to this project:

- We need to tell Gradle where the Fluvio library lives (i.e. which repository)
- We need to tell Gradle the name and version of the Fluvio library

To do this, we'll update the `build.gradle` file like the following:

```diff
  /*
   * This file was generated by the Gradle 'init' task.
   *
   * This generated file contains a sample Java application project to get you started.
   * For more details take a look at the 'Building Java & JVM projects' chapter in the Gradle
   * User Manual available at https://docs.gradle.org/7.0/userguide/building_java_projects.html
   */
  
  plugins {
      // Apply the application plugin to add support for building a CLI application in Java.
      id 'application'
  }
  
  repositories {
      // Use Maven Central for resolving dependencies.
      mavenCentral()
+     maven {
+         name = "GitHub Packages"
+         url = uri("https://maven.pkg.github.com/infinyon/fluvio-client-java")
+         credentials {
+             username = System.getenv("GITHUB_ACTOR")
+             password = System.getenv("GITHUB_TOKEN")
+         }
+     }
  }
  
  dependencies {
      // Use JUnit test framework.
      testImplementation 'junit:junit:4.13.1'
  
      // This dependency is used by the application.
      implementation 'com.google.guava:guava:30.0-jre'
+
+     implementation 'com.infinyon:fluvio-16:0.0.1'
  }
  
  application {
      // Define the main class for the application.
      mainClass = 'fluvio.java.app.App'
  }
```

### Writing the Producer


##### This code performs the following actions:

- _Import `fluvio`;_
- _Create a new Fluvio Client Instance;_
- _Create a connection to a local Fluvio Cluster;_
- _Create a new topic producer for `hello-python`;_
- _Listen for input typed into the terminal;_
- _Send typed input to the fluvio cluster;_


### Writing the Consumer


##### This code performs the following actions:

- _Import `fluvio` module;_
- _Create a new Fluvio Client Instance;_
- _Create a connection to a local Fluvio Cluster;_
- _Create a new topic consumer for `hello-python`;_
- _Listen for events sent by a topic producer;_

## Running the Demo

Now that the code is written, we're ready to run our `Hello, World! 🎉` example. Run the following commands in separate terminals.

### Running the Producer


### Running the Consumer


## Congratulations!

You've now completed the Fluvio "Hello, World! 🎉" tutorial.

Head over to the Fluvio Java documentation to learn more about the library
and available options.

## Read the `fluvio` Docs

Check out <a href="https://infinyon.github.io/fluvio-client-python/fluvio.html" ////////////////////////////////////
target="_blank">Python API</a> reference guide for additional usage information
and documentation.
