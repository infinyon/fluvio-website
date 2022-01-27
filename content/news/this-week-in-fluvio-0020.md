---
title: "This Week in Fluvio #20"
date: 2022-01-26
weight: 20
---
Welcome to This Week in Fluvio, our weekly newsletter
for development updates to [Fluvio open source]. Fluvio is a distributed,
programmable streaming platform written in Rust.

{{< banner >}}

## Local Connector Development

This week we wanted to walkthrough the process of developing a connector. 

We'll approach connector development in this order:
1. [Build and run your client locally]({{<ref "#build-and-run-your-client-locally">}})
2. [Package your Client into Docker Image]({{<ref "#package-your-client-into-docker-image">}})
3. [Load your Connector image into Kubernetes Cluster]({{<ref "#load-your-connector-image-into-kubernetes-cluster">}})
4. [Create a new Connector in Fluvio]({{<ref "#create-a-new-connector-in-fluvio">}})

### Tools needed to follow this guide

You'll need the following tools installed
- Python 3.x
- Docker
- one of our supported Kubernetes distros  (We'll cover k3d + minikube)
- docker-compose (optional)

## Build and Run Your Client Locally

Let's start with building a simple client.

This client is written in Python, but we have client libraries for Rust, Javascript, Java and Go.

%copy%
```python
#!/usr/bin/env python3
#
# get-cat-facts.py
# An example Python-based Fluvio connector

from fluvio import Fluvio
import requests
import time

WAIT_SECONDS = 10
CAT_FACTS_API = 'https://catfact.ninja/fact'
CAT_FACTS_TOPIC = 'cat-facts-random'

if __name__ == '__main__':
    # Before entering event loop
    # Connect to cluster and create a producer before we enter loop
    fluvio = Fluvio.connect()
    producer = fluvio.topic_producer(CAT_FACTS_TOPIC)

    # Event loop
    while True:
        # Get random cat fact
        catfact = requests.get(CAT_FACTS_API)

        # Save fact
        producer.send_string(catfact.text)

        # Print fact to container logs
        print(catfact.text)

        # Be polite and control the rate we send requests to external API
        time.sleep(WAIT_SECONDS)
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

Now that we have verfied that our client works locally, we need to package it for Kubernetes. We do that by defining our data connector runtime environment with a Dockerfile.

We use the `python` base image to keep things simple. 

Then we create a new user `fluvio` with a home directory (in `/home/fluvio`).

This is **required** for all connectors. The Fluvio cluster shares information with the `fluvio` user on startup.

%copy%
```Dockerfile
# Dockerfile
FROM python

# This is required to connect to a cluster
# Connectors run as the `fluvio` user
ENV USER=fluvio
RUN useradd --create-home "$USER"
USER $USER

# Install dependencies
RUN pip install fluvio requests

# Copy our python script into the connector image
COPY get-cat-facts.py /usr/local/sbin/get-cat-facts.py

# Start script on start
ENTRYPOINT /usr/local/sbin/get-cat-facts.py
```

### Option 1: Build and Test the Container
You can build the Docker image with this command.

%copy%
```shell
$ docker build -t infinyon/fluvio-connect-cat-facts .
```

{{< caution >}}
The image name `infinyon/fluvio-connect-cat-facts` will be significant when we create the connector in the Fluvio cluster with `fluvio connector create`.
{{< /caution >}}


Start a the container with this `docker` command

%copy first-line%
```shell
$ docker run -it --rm -v $HOME/.fluvio:/home/fluvio/.fluvio --network host infinyon/fluvio-connect
-cat-facts
```

You can check out `docker run --help` if you want a description of what these do, so I'll describe why you want to use them instead.

| Docker option                           | Why you want it                                                                             |
|-----------------------------------------|---------------------------------------------------------------------------------------------|
| `-it`                                   | Without both `-i` and `-t`, you can't use `ctrl+c` to exit your container                   |
| `--rm`                                  | Prevents accumulating Docker-related mess on your host while you are testing your connector |
| `-v $HOME/.fluvio:/home/fluvio/.fluvio` | Share existing Fluvio config - This is *effectively* what `fluvio connector create` does    |
| `--network host`                        | The default docker network can't reach the host's Fluvio cluster                            |

### Option 2: Build and Test the Container

Alternatively, you can use `docker-compose` to save on typing while you iterate. Below I create a service `pyconnector` with the same details we used for `docker build` and `docker run`.

%copy%
```yaml
# docker-compose.yml
services:
  pyconnector:
    build: .
    image: infinyon/fluvio-connect-cat-facts
    network_mode: host
    volumes:
      - $HOME/.fluvio:/home/fluvio/.fluvio
```

Build the docker image with this command

%copy first-line%
```shell
$ docker-compose build 
```

Run the `docker-compose` service `pyconnector`.

%copy first-line%
```shell
$ docker-compose run --rm pyconnector
Creating pyconnect_pyconnector_run ... done
{"fact":"Cats can jump up to 7 times their tail length.","length":46}
{"fact":"A cat can jump 5 times as high as it is tall.","length":45}
```

## Load Your Connector Image into Kubernetes Cluster

By default, connectors will attempt to pull images from the internet. However, development images need to be testable locally before making them available publicly. So we pre-load the connector images.

For k3d:

%copy first-line%
```shell
$ k3d image import infinyon/fluvio-connect-cat-facts
```

For minikube:

%copy first-line%
```shell
$ minikube image load infinyon/fluvio-connect-cat-facts
```

## Create a new Connector in Fluvio

Last step for testing our connector is verifying that it runs in the Fluvio cluster. We will create the config file and run the CLI command

### The Connector config

```yaml
# connector-config.yml
version: dev
name: cat-facts-connector
type: cat-facts
create_topic: true
topic: cat-facts-random
direction: source
```

| Connector config option | Description                                                                                                                                                              |
|-------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `version`               | This value must be  `dev`  for local development.                                                                                                                        |
| `name`                  | This value is the name you see in  `fluvio connector list`                                                                                                               |
| `type`                  | The value of this name will be used for tagging image before loading into Kubernetes <br><br> Connector image names follow the pattern: `infinyon/fluvio-connect-{type}` |
| `create_topic`          | Set this `true`/`false` based on whether you want a topic automatically created for your connector                                                                       |
| `topic`                 | A topic will be created with this value When `create_topic` is `true`                                                                                                    |
| `direction`             | This is some metadata about the direction of data flow. Currently the only option is `source`.    

Lastly, create the connector

%copy first-line%
```shell
$ fluvio connector create --config example-connector.yml
```

%copy first-line%
```shell
$ fluvio connector list
 NAME                 STATUS
 cat-facts-connector  Running
 ```

We can look at the container logs, and verify the topic has our records.

%copy first-line%
```shell
$ fluvio connector logs cat-facts-connector
{"fact":"Cats eat grass to aid their digestion and to help them get rid of any fur in their stomachs.","length":92}
{"fact":"When a cat drinks, its tongue - which has tiny barbs on it - scoops the liquid up backwards.","length":92}
{"fact":"Cats and kittens should be acquired in pairs whenever possible as cat families interact best in pairs.","length":102}
```

And again, to verify we check the contents of the topic. We see the last 3 rows match.

%copy first-line%
```shell
$ fluvio consume cat-facts-random -B
Consuming records from the beginning of topic 'cat-facts-random'
{"fact":"Cats bury their feces to cover their trails from predators.","length":59}
{"fact":"Cats step with both left legs, then both right legs when they walk or run.","length":74}
{"fact":"Cats can jump up to 7 times their tail length.","length":46}
{"fact":"A cat can jump 5 times as high as it is tall.","length":45}
{"fact":"Cats eat grass to aid their digestion and to help them get rid of any fur in their stomachs.","length":92}
{"fact":"When a cat drinks, its tongue - which has tiny barbs on it - scoops the liquid up backwards.","length":92}
{"fact":"Cats and kittens should be acquired in pairs whenever possible as cat families interact best in pairs.","length":102}
```


---

Get in touch with us on [Github Discussions] or join [our Discord channel] and come say hello!

For the full list of changes this week, be sure to check out [our CHANGELOG].

Until next week!

[Fluvio open source]: https://github.com/infinyon/fluvio
[our CHANGELOG]: https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md
[our Discord channel]: https://discordapp.com/invite/bBG2dTz
[Github Discussions]: https://github.com/infinyon/fluvio/discussions