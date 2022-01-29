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
- [Python 3.x](https://www.python.org/downloads/)
- [pip](https://pip.pypa.io/en/stable/installation/)
- [python-fluvio](https://github.com/infinyon/fluvio-client-python)
- [python-requests](https://docs.python-requests.org/en/latest/)
- [Docker](https://www.docker.com/get-started)
- Kubernetes distros ([k3d](https://k3d.io/) or [minikube](https://minikube.sigs.k8s.io/docs/start/))

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

Before we run this code, we need to create the fluvio topic that our client produces data to

%copy first-line%
```shell
$ fluvio topic create cat-facts-random
```

Install fluvio package:

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

Now that we have verfied that our client works locally, we need to package it for Kubernetes. We do that by defining our data connector runtime environment with a Dockerfile.

We use the `python` base image to keep things simple. 

Then we create a new user `fluvio` with a home directory (in `/home/fluvio`).

This is **required** for all connectors. The Fluvio cluster shares information with the `fluvio` user on startup.

%copy%
```Dockerfile
# Dockerfile
FROM python

# Copy our python script into the connector image
COPY get-cat-facts.py /usr/local/sbin/get-cat-facts.py
RUN chmod +x /usr/local/sbin/get-cat-facts.py

# This is required to connect to a cluster
# Connectors run as the `fluvio` user
ENV USER=fluvio
RUN useradd --create-home "$USER"
USER $USER

# Install dependencies
RUN pip install fluvio requests

# Start script on start
ENTRYPOINT get-cat-facts.py
```

### Build and Test the Container
You can build the Docker image with this command.

%copy first-line%
```shell
$ docker build -t infinyon/fluvio-connect-cat-facts .
```

The image should have been created

%copy first-line%
```shell
$ docker images
REPOSITORY                          TAG            IMAGE ID       CREATED         SIZE
infinyon/fluvio-connect-cat-facts   latest         08ced64017f0   5 seconds ago   936MB
...
```

{{< caution >}}
The image name `infinyon/fluvio-connect-cat-facts` will be significant when we create the connector in the Fluvio cluster with `fluvio connector create`.
{{< /caution >}}


Start a the container with this `docker` command

%copy first-line%
```shell
$ docker run -it --rm -v $HOME/.fluvio:/home/fluvio/.fluvio --network host infinyon/fluvio-connect-cat-facts
{"fact":"In the 1750s, Europeans introduced cats into the Americas to control pests.","length":75}
...
<CTRL>-C
```

You can check out `docker run --help` if you want a description of what these do, so I'll describe why you want to use them instead.

| Docker option                           | Why you want it                                                                             |
|-----------------------------------------|---------------------------------------------------------------------------------------------|
| `-it`                                   | Without both `-i` and `-t`, you can't use `ctrl+c` to exit your container                   |
| `--rm`                                  | Prevents accumulating Docker-related mess on your host while you are testing your connector |
| `-v $HOME/.fluvio:/home/fluvio/.fluvio` | Share existing Fluvio config - This is *effectively* what `fluvio connector create` does    |
| `--network host`                        | The default docker network can't reach the host's Fluvio cluster                            |

## Load Your Connector Image into Kubernetes Cluster

By default, connectors will attempt to pull images from the internet. However, development images need to be testable locally before making them available publicly. So we pre-load the connector images.

{{< tabs tabTotal="2" tabID="1" tabName1="k3d" tabName2="minikube">}}

{{< tab tabNum="1">}}

Let's create a cluster called `fluvio`

%copy first-line%
```shell
$ k3d cluster create fluvio
```

And import the image (use the name of your cluster)

%copy first-line%
```shell
$ k3d image import infinyon/fluvio-connect-cat-facts --cluster fluvio
```

The image should have been created

%copy first-line%
```shell
$ docker exec k3d-fluvio-server-0 sh -c "ctr image list -q"
docker.io/infinyon/fluvio-connect-cat-facts:latest
...
```

{{</ tab >}}

{{< tab tabNum="2">}}

Load image to `minikube`:

%copy first-line%
```shell
$ minikube image load infinyon/fluvio-connect-cat-facts
```

{{</ tab >}}

{{</ tabs >}}




## Create a new Connector in Fluvio

Last step for testing our connector is verifying that it runs in the Fluvio cluster. We will create the config file and run the CLI command

### The Connector config

Create a connector configuration file `example-connector.yaml`:

%copy%

```yaml
# example-connector.yaml
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
| `name`                  | A unique name for this connector. <br><br> It will be displayed in `fluvio connector list`                                                                                                               |
| `type`                  | The value of this name will be used for tagging image before loading into Kubernetes. <br><br> Connector image names follow the pattern: `infinyon/fluvio-connect-{type}` |
| `create_topic`          | Set this `true`/`false` based on whether you want a topic automatically created for your connector. (Ignored if topic already created)                                                                       |
| `topic`                 | The name of the `topic` where the connector will publish the data records. .                                                                            |
| `direction`             | The metadata that defines the direction of data flow (`source` or `sink`).<br><br> This is a `source` connector.

Lastly, create the connector

%copy first-line%
```shell
$ fluvio connector create --config example-connector.yaml
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