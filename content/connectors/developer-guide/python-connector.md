---
title: Custom Local Connector with Python
weight: 20
---

Fluvio supports multiple programming languages for developing your own custom connectors. In you'd like to submit your connector to the InfinyOn Connector catalog, send us a note on [Discord](https://discord.gg/zHsWBt5Z2n).

In chapter, we'll give an step-by-step example on how to develop a Python Connector:

1. [Build and run your client locally]({{<ref "#build-and-run-your-client-locally">}})
2. [Package your Client into Docker Image]({{<ref "#package-your-client-into-docker-image">}})
3. [Start Connector]({{<ref "#start-the-connector">}})
4. [Verify data in Fluvio topic]({{<ref "#verify-topic-data-from-packaged-connector">}})

### Tools needed to follow this example

You'll need the following tools installed
- [Python 3.x](https://www.python.org/downloads/)
- [pip](https://pip.pypa.io/en/stable/installation/)
- [python-fluvio](https://github.com/infinyon/fluvio-client-python)
- [python-requests](https://requests.readthedocs.io/en/latest/)
- [Docker](https://www.docker.com/get-started)
- [Rust development environment](https://www.rust-lang.org/learn/get-started)
    - This may be needed to install the `fluvio` python package

## Build and Run Your Client Locally

Let's start with building a simple client.

This client is written in [Python], but we have client libraries for [Rust], [Javascript], [Java] and [Go] (with community support).

[Python]: {{<ref "/api/official/python/installation.md">}}
[Rust]: {{<ref "/api/official/rust/installation.md">}}
[Javascript]: {{<ref "/api/official/node/installation.md">}}
[Java]: {{<ref "/api/official/java/installation.md">}}
[Go]: {{<ref "/api/community/go.md">}}

{{<code file="code-blocks/connectors/developer-guide/python-connector/get-cat-facts.py" lang="python" copy=true >}}

Before we run this code, we need to create the fluvio topic that our client produces data to

%copy first-line%
```shell
$ fluvio topic create cat-facts-random
```

In order to test our connector application, we'll need to install libraries in our development environment.

Install the [fluvio](https://pypi.org/project/fluvio/) and [requests](https://pypi.org/project/requests/) python packages using `pip` (or `pip3`):

%copy first-line%
```shell
$ pip install fluvio requests
```

Running the Python code prints out a new cat fact every 10 seconds

%copy first-line%
```shell
$ python3 ./get-cat-facts.py
{"fact":"Cats bury their feces to cover their trails from predators.","length":59}
{"fact":"Cats step with both left legs, then both right legs when they walk or run.","length":74}
```

And we verify that these records have made it to the Fluvio cluster by consuming from the topic.

%copy first-line%
```shell
$ fluvio consume cat-facts-random -B
Consuming records from the beginning of topic 'cat-facts-random'
{"fact":"Cats bury their feces to cover their trails from predators.","length":59}
{"fact":"Cats step with both left legs, then both right legs when they walk or run.","length":74}
```

## Package Your Client into Docker Image

This section is optional, but is recommended if you want to share your connectors with others. Packaging with Docker will allow you to bundle in any libraries or tools.

The result will enable another user to start the connector locally without additional setup (assuming they have Docker and Fluvio on their machine).

We use the `python` base image to keep things simple.

Then we create a new user `fluvio` with a home directory (in `/home/fluvio`).

This is **required** for all connectors. The Fluvio cluster shares information with the `fluvio` user on startup.

{{<code file="code-blocks/connectors/developer-guide/python-connector/Dockerfile" lang="dockerfile" copy=true >}}

### Build and Test the Container
You can build the Docker image with this command.

%copy first-line%
```shell
$ docker build -t cat-facts-connector .
```

The image should have been created

%copy first-line%
```shell
% docker image list cat-facts-connector
REPOSITORY            TAG       IMAGE ID       CREATED          SIZE
cat-facts-connector   latest    9a48639f8bac   17 minutes ago   2.66GB
```

In needed, create your topic *before* starting your container

%copy first-line%
```shell
$ fluvio topic create cat-facts-random
```

### Start the connector
Start a the container with this `docker` command

%copy first-line%
```shell
% docker run --init -it --rm -v $HOME/.fluvio:/home/fluvio/.fluvio --network host cat-facts-connector
{"fact":"The first cat show was organized in 1871 in London. Cat shows later became a worldwide craze.","length":93}
...
<CTRL>-C
```

You can check out `docker run --help` if you want a description of what these do, so I'll describe why you want to use them instead.

| Docker option                           | Why you want it                                                                             |
|-----------------------------------------|---------------------------------------------------------------------------------------------|
| `--init`                                | This will ensure that your container will be able to support signals to exit                |
| `-it`                                   | Without both `-i` and `-t`, you can't send `ctrl+c` to exit signals your container          |
| `--rm`                                  | Prevents accumulating Docker-related mess on your host while you are testing your connector |
| `-v $HOME/.fluvio:/home/fluvio/.fluvio` | Share existing Fluvio config - This is *effectively* what `fluvio connector create` does    |
| `--network host`                        | The default docker network can't reach the host's Fluvio cluster                            |

We can look at topic, and verify our records.

### Verify topic data from packaged connector

%copy first-line%
```shell
% fluvio consume cat-facts-random -B
Consuming records from the beginning of topic 'cat-facts-random'
{"fact":"The first cat show was organized in 1871 in London. Cat shows later became a worldwide craze.","length":93}
{"fact":"A cat's cerebral cortex contains about twice as many neurons as that of dogs. Cats have 300 million neurons, whereas dogs have about 160 million. See, cats rule, dogs drool!","length":173}
```

## Conclusion

We just walked through how to develop a Fluvio connector from scratch, using the Fluvio python library.

Also we demonstrated how to package your connector with Docker, how to start the Docker-based connector, and verified data in our topic.

You are now prepared to build your own connectors. This one was for Python, but the pattern should be the same for any of our other language bindings or CLI.