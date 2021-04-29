---
title: '"Hello, World! 🎉" in Java'
toc: true
---
{{< lang-selector >}}

In this tutorial, you will learn how to create a topic, build a producer/consumer in Java, and sends a "Hello, World! 🎉" message.

## Prerequisites: Install Fluvio, Java and Gradle

Before starting this tutorial, make sure you have
[followed the Fluvio getting started guide] and have installed the Fluvio CLI
and have access to a Fluvio cluster, either in Fluvio Cloud or running locally
on your machine. Then, you'll need to make sure you have Java and Gradle installed.

[followed the Fluvio getting started guide]: /docs/getting-started

The Fluvio Java library should work with any version of Java greater than
version 8, but we have specifically tested it on Java 8, 11, 15, and 16.
In order to see whether you have the Java Development Kit (JDK) installed
and to see what version it is, run the following:

```bash
$ javac --version
javac 16.0.1      # You might have something different
```

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

### Setup PATH and JAVA_HOME

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

Notice the `credentials` block. In order to download libraries from GitHub Packages,
we need to provide a GitHub username (the `GITHUB_ACTOR`) as well as something called
a "Personal Access Token" (the `GITHUB_TOKEN`). Let's see how to create the token on
the GitHub website.

#### Creating a Personal Access Token

In order to create a token, click on your user profile on the GitHub website and go to
the "Settings" page.

<img 
    src="/tutorials/images/java/settings.png"
    alt="In the profile dropdown, click on the settings item" />

Then, click on "Developer settings" in the left-hand menu.

<img
    src="/tutorials/images/java/developer-settings.png"
    alt="In settings, click on Developer Settings" />

Then, click on "Personal access tokens" and "Generate new token".

<img
    style="width:600px;"
    src="/tutorials/images/java/personal-access-token.png"
    alt="In developer settings, click on personal access tokens" />

You'll be prompted to re-enter your password for GitHub, then you'll find a page where
you can grant permissions to your new token. The only permission this token needs is
`read:packages`, so leave all the other fields blank. Then, click Generate Token.

<img
    style="height:800px;"
    src="/tutorials/images/java/new-token.png"
    alt="Add the read packages permission to your token" />

On the next page, you'll be presented with a token that will allow you to download
packages from GitHub Packages. Make sure to copy that token, and let's place it in a
new file called `.env` in the root of your project.

```bash
export GITHUB_ACTOR="Your GitHub username"
export GITHUB_TOKEN="Your GitHub token"
```

And as one last step let's set the following permission on the `.env` file:

```bash
$ chmod +x .env
```

This pattern of saving tokens and other environment variables to a `.env` file is a
common practice in projects that need to grant access to secret data. It is important
not to commit this file to git, because you don't want to share your `.env` file with
anybody else.

From now on, whenever you want to run your project, start by "sourcing" the `.env` file
to load those variables into your shell:

```bash
$ source .env
```

To make sure it worked, let's build and run the project one more time.

```bash
$ ./gradlew run
Starting a Gradle Daemon (subsequent builds will be faster)

> Task :app:run
Hello World!

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
```

If that didn't work for you, double check that you followed all the steps correctly.
Once you have that working, continue on to see how to produce and consume records in
our new app!

### Writing the Application

Now we're ready to write our Java app. In our source file,
`app/src/main/java/fluvio/java/app/App.java`, open an editor and write the following:

```java
package fluvio.java.app;

import com.infinyon.fluvio.Fluvio;
import com.infinyon.fluvio.TopicProducer;
import com.infinyon.fluvio.PartitionConsumer;
import com.infinyon.fluvio.PartitionConsumerStream;
import com.infinyon.fluvio.Offset;
import com.infinyon.fluvio.Record;

public class App {
    public static void main(String[] args) throws Exception {
        Fluvio fluvio = Fluvio.connect();
        TopicProducer producer = fluvio.topic_producer("hello-java");
        PartitionConsumer consumer = fluvio.partition_consumer("hello-java", 0);

        for (int i = 0; i < 10; i++) {
            producer.send(String.valueOf(i).getBytes(), ("Hello " + i).getBytes());
        }

        PartitionConsumerStream stream = consumer.stream(Offset.beginning());
        for (int i = 0; i < 10; i++) {
            Record record = stream.next();
            System.out.printf("Consumed record, key=%s, value=%s\n", record.key_string(), record.value_string());
        }
    }
}
```

## Running the Demo

In order to send and receive records from Fluvio, we need to first create a Topic:

```bash
$ fluvio topic create hello-java
```

This is the name of the topic we used in our Java code, so make sure you use the same name!

Next, let's open up a Fluvio consumer on the command-line so that we can see the records
being sent to our topic:

```bash
$ fluvio consume hello-java -B --key-value
```

This terminal will stay open and start printing out records as they are sent to our topic.

Next, let's use another terminal to run the Java project. Open up your project directory
again and run gradle again.

```bash
$ ./gradlew run

> Task :app:run
Consumed record, key=0, value=Hello 0
Consumed record, key=1, value=Hello 1
Consumed record, key=2, value=Hello 2
Consumed record, key=3, value=Hello 3
Consumed record, key=4, value=Hello 4
Consumed record, key=5, value=Hello 5
Consumed record, key=6, value=Hello 6
Consumed record, key=7, value=Hello 7
Consumed record, key=8, value=Hello 8
Consumed record, key=9, value=Hello 9

BUILD SUCCESSFUL in 722ms
2 actionable tasks: 2 executed
```

In your other terminal window with the Fluvio consumer, you should have seen your records appear:

```bash
$ fluvio consume hello-java -B --key-value
[0] Hello 0
[1] Hello 1
[2] Hello 2
[3] Hello 3
[4] Hello 4
[5] Hello 5
[6] Hello 6
[7] Hello 7
[8] Hello 8
[9] Hello 9
```

## Congratulations!

You've now completed the Fluvio "Hello, World! 🎉" tutorial for Java.

Head over to the Fluvio Java documentation to learn more about the library
and available options.

## Read the `fluvio` Docs

Check out <a href="https://infinyon.github.io/fluvio-client-python/fluvio.html" ////////////////////////////////////
target="_blank">Python API</a> reference guide for additional usage information
and documentation.
