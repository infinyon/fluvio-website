---
title: '"Hello, World! 🎉" in Python'
toc: true
---
{{< lang-selector >}}

In this tutorial, you will learn how to create a topic, build a producer/consumer in Typescript, and sends a "Hello, World! 🎉" message.

## Prerequisites

Before starting on this tutorial, you'll need to have completed the following

- Install [Python](#check-python) (**3.6** or above)
- Have the Fluvio CLI (version  `0.7.1` or greater) installed <sup>[1]</sup>
- Have access to a Fluvio cluster.

See our [getting started] guide for more details on getting set up.

[getting started]: /docs/getting-started

-> [1]: If you need to, you can update the Fluvio CLI by using `fluvio update`.

### Create a Topic using the Fluvio CLI

In Fluvio, we send all of our messages to something called a Topic, which
is like a category for related messages. For this tutorial, we'll create
a topic called `hello-python` using the following command:

```bash
$ fluvio topic create hello-python
```

### Check Python

A Fluvio environment for Python requires Python **3.6** or above.

#### Install Python

Python installation varies depending on your operating system.

|   Operating System     |         Instructions           |
|------------------------|--------------------------------|
| MacOS                  | There's an official installer from <a href="https://www.python.org/downloads/mac-osx/" target="_blank">python.org</a> to install on **macOS** but can also download python via [`brew`](https://docs.brew.sh/Homebrew-and-Python).  |
| Linux                  | There's an official installer from <a href="https://www.python.org/downloads/source/" target="_blank">python.org</a>. It's also available in nearly every package manager. [This guide helps with ubuntu](https://docs.python-guide.org/starting/install3/linux/).

To check your python version run the command `python --version`.

## Writing the Application

The following sections will setup your project and walk through writing the application files.

### Installing Project Dependencies

Run the following script to setup your project for development:

```bash
$ mkdir fluvio-python-demo && \
cd fluvio-python-demo && \
python -m venv venv && \
source venv/bin/activate && \
pip install fluvio && \
touch producer.py consumer.py
```

This willl create a directory `fluvio-python-demo` with a [Virtual
Environment](https://docs.python.org/3/tutorial/venv.html) in it in the `venv`
sub directory. This will then install the fluvio package from
[pypi.org](https://pypi.org/project/fluvio/) in the virtual environment.


Your working directory should now contain the following files:

```bash
$ ls
consumer.py  producer.py  venv
```

### Writing the `producer.py` File

Write the following code in your `producer.py` file.


```Python
from fluvio import Fluvio
fluvio = Fluvio.connect()

producer = fluvio.topic_producer("hello-python")
partition = 0
while True:
    line = input('> ')
    producer.send_record(line, partition)
```

##### This code performs the following actions:

- _Import `fluvio`;_
- _Create a new Fluvio Client Instance;_
- _Create a connection to a local Fluvio Cluster;_
- _Create a new topic producer for `hello-python`;_
- _Listen for input typed into the terminal;_
- _Send typed input to the fluvio cluster;_


### Writing the `consumer.py` File

Write the following code in your `consumer.ts` file.

```python
from fluvio import Fluvio
partition = 0
consumer = fluvio.partition_consumer("hello-python")
for i in consumer.stream(0):
    print("Received message: %s" % i)
```

##### This code performs the following actions:

- _Import `fluvio` module;_
- _Create a new Fluvio Client Instance;_
- _Create a connection to a local Fluvio Cluster;_
- _Create a new topic consumer for `hello-python`;_
- _Listen for events sent by a topic producer;_

## Running the Demo

Now that the code is written, we're ready to run our `Hello, World! 🎉` example. Run the following commands in separate terminals.

### Running the Producer

Run the following command in the working directory:

```bash
$ ./venv/bin/python ./producer.py
```

```bash
Fluvio Producer created, waiting for input:

>
```

Great! Now type `Hello, World! 🎉` into your terminal window:

```bash
Hello, World! 🎉
```

<br/>
<hr/>

### Running the Consumer

Open a new terminal and run the following command:

```bash
$ ./venv/bin/python ./consumer.py
```

```bash
Fluvio Consumer created, listening for events:


Received message: Hello, World! 🎉
```

## Congratulations!

You've now completed the Fluvio "Hello, World! 🎉" tutorial.

Head over to the Fluvio Python documentation to learn more about the library
and available options.

## Read the `fluvio` Docs

Checkout <a href="https://infinyon.github.io/fluvio-client-python/fluvio.html"
target="_blank">Python API</a> reference guide for additional usage information
and documentation.
