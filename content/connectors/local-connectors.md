---
title: Run connectors locally 
menu: Local connectors
---

## Local Connectors

Local Connectors are deployed using Docker. Each connector is packaged into
a container, allowing for easy and portable execution. When running a local connector, configurations
are passed to it using command-line arguments, given at the end of the `docker run` command.

One of the primary ways Local connectors are different from Managed connectors is that
we have to manually set up our Fluvio profile for Local connectors. Fluvio profiles live
in the `~/.fluvio/config` file, and each profile describes how to connect to a specific
Fluvio cluster. Therefore, in order to use a Local connector, we need to give it access to
our `~/.fluvio/config` file.

Let's go ahead and check out what this looks like. First, create a topic for our connector
to pump data into.

%copy first-line%
```bash
$ fluvio topic create cat-facts
```

Now, let's try running the `http` connector in a docker container using the following command:

%copy%
```bash
docker run -d --name="my-http" \
    -v"$HOME/.fluvio/config:/home/fluvio/.fluvio/config" \
    -t infinyon/fluvio-connect-http:latest \
    -- \
    --endpoint="https://catfact.ninja/fact" \
    --fluvio-topic="cat-facts" \
    --interval=10
```

What we're doing here is setting up the HTTP connector to fetch a new cat fact every 10
seconds and produce it to our Fluvio topic `cat-facts`. Here are some important points to
understand about this command:

- `-v"$HOME/.fluvio/config:/home/fluvio/.fluvio/config"`
  - This part of the command is what shares our `~/.fluvio/config` file with the connector
    so that it has access to our Fluvio profiles. The connector will connect using the "active"
    profile in the config, which you can view using `fluvio profile`
- `-t infinyon/fluvio-connect-http:latest`
  - Specifies which docker image should be used to launch this connector. Since we're using
    the HTTP connector, we give it the full name of the container on Docker Hub.
- The rest of the arguments are specific to the HTTP connector, and you can read more about
  them on [the HTTP connector docs page][1].

You should be able to see the cat facts start rolling in, we can check this
by opening a consumer in another terminal window.

```bash
$ fluvio consume cat-facts -B
{"fact":"A cat almost never meows at another cat, mostly just humans. Cats typically will spit, purr, and hiss at other cats.","length":116}
{"fact":"In one stride, a cheetah can cover 23 to 26 feet (7 to 8 meters).","length":65}
{"fact":"Phoenician cargo ships are thought to have brought the first domesticated cats to Europe in about 900 BC.","length":105}
```

If we want to view the logs of our local connector to ensure it's running properly, we can
use the `docker logs` command, like so:

%copy first-line%
```bash
$ docker logs -f my-http
```

And finally, when we're done with our connector, we can stop it from running using `docker kill`:
