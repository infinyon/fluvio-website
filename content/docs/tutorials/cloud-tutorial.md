---
title: Using InfinyOn Cloud Backend — Connectors and SmartModules
menu: InfinyOn Cloud Tutorial
weight: 60
---

This tutorial will walk you through setting up an InfinyOn account, using Fluvio,
and learning the basics of connectors and SmartModules.

## Basic Setup

<img src="../images/Fluvio-Cloud-Flow.png"
     alt="Signup flow for InfinyOn Cloud"
	 style="justify: center; max-width: 500px" />

There are three main steps for setting up for the Cloud: Installing the CLI,
registering for a Cloud account, and finally linking the two together.

The following sections will walk you through how to do that.

### Install Fluvio CLI

Download the Fluvio CLI with the following command.

%copy first-line%
```bash
$ curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```

{{< partial file="docs/path-env.html" >}}

### Create InfinyOn Cloud Account
Head over to the [InfinyOn Cloud sign up page](https://infinyon.cloud).

Currently, there are two mutually exclusive ways to create your Fluvio account:
Google sign in, and Email registration.

##### Signing up with Google

If you wish to avoid creating a new username and password for InfinyOn Cloud, then click on the blue **Sign up with Google** button.

<img src="../images/google-signup-part1.jpg"
     alt="A screenshot of step one of Signing up to InfinyOn with Google — click on the sign in button"
     style="justify: center; max-width: 340px" />

The link will take you to a Google login prompt if you are not already logged into Google.

<img src="../images/google-signup-part2.jpg"
     alt="a screenshot of step two of Signing up to InfinyOn with Google — associate account"
     style="justify: center; max-width: 340px" />

Finally, Google will provide a confirmation screen confirming that you want to log into InfinyOn Cloud with your Google account.

##### Create an InfinyOn Cloud Account

<img src="../images/cloud-signup.jpg"
     alt="A screenshot of the InfinyOn new account form, with Name, Organization, Email, and Password fields"
     style="justify: center; max-width: 340px" />

After filling out the form, you'll be greeted with a success message requesting
that you confirm your email address. You'll need to complete this step in order
to continue.

~> The link in the email will time out after a few minutes.

<img src="../images/cloud-verification.jpg"
     alt="A screenshot of the verification email received after completing the signup form, including a verification link"
     style="justify: center; max-width: 500px" />

You should now get a confirmation that your account is ready to use.

<img src="../images/cloud-confirmation.jpg"
     alt="A screenshot of the prompt received after clicking the verification link, saying the account is ready to use"
     style="justify: center; max-width: 340px" />

At this point, you can log in via the Fluvio CLI and start sending and receiving
messages to your Fluvio cluster.

### Link InfinyOn Cloud to Fluvio CLI

Depending which method you chose in account creation, you will have the option of
logging in with OAuth2, or username/password. Please follow the relevant steps
to link the Cloud to your Fluvio CLI.

##### Connecting with OAuth2

Use the command `fluvio cloud login --use-oauth2` to connect to the InfinyOn Cloud.

%copy first-line%
```bash
$ fluvio cloud login --use-oauth2
A web browser has been opened at https://infinyon-cloud.us.auth0.com/activate?user_code=GLMC-QDDJ.
Please proceed with authentication.
```

Fluvio will open a login screen in your browser. If it is unable to do so, copy the
URL provided in the command prompt and enter it into your web browser manually.

Verify that the code matches what was displayed at the end of the URL in the
command prompt, then click the **Confirm** button.

<img src="../images/google-oauth-screen1.jpg"
     alt="screenshot showing verification code that Fluvio is trying to connect with."
     style="justify: center; max-width: 340px" />

If you are not already logged into Google, the next step is to sign in.

<img src="../images/google-oauth-screen2.jpg"
     alt="screenshot showing Google requesting you log in with your google account"
     style="justify: center; max-width: 340px" />

One last verification that you wish to authorize InfinyOn Cloud, click **Accept**.

<img src="../images/google-oauth-screen3.jpg"
     alt="screenshot showing OAuth2 requesting access to your Google account"
     style="justify: center; max-width: 340px" />

Congrats, everything is now set up!

<img src="../images/google-oauth-screen4.jpg"
     alt="screenshot showing confirmation that OAuth2 connection is complete"
     style="justify: center; max-width: 340px" />

##### Connecting with username and password

Use the command `fluvio cloud login` to connect the InfinyOn Cloud to your
Fluvio CLI. It will ask for your account credentials, as seen below.

%copy first-line%
 ```bash
$ fluvio cloud login
InfinyOn Cloud email: John@example.com
Password:
```

## A Quick Introduction to the Fluvio CLI

You can now see Fluvio in action by creating a simple topic, and pushing data to it:

%copy first-line%
```bash
$ fluvio topic create greetings
topic "greetings" created
```

%copy first-line%
```bash
$ echo 'Hello world!' | fluvio produce greetings

```

%copy first-line%
```bash
$ fluvio consume greetings -dB
Hello world!

```

The storage and retrieval of records from the topic as shown above are basic
actions of the Fluvio CLI.

Two core commands of Fluvio you will need to be familiar with are
`fluvio produce <topic>` and `fluvio consume <topic>`.

#### Produce

`fluvio produce` is the main way to get data into the Fluvio database. While
most of the time you may be calling it through an API, here is how to access it
through the CLI.

%copy first-line%
```bash
$ fluvio produce greetings
> test
Ok!
> meow
Ok!
>
```

-> To quit `produce` press `CTRL-C`.

`fluvio produce <topic> [flags]` can take input from stdin, or from a file. The
stdin input can be piped into, [as shown in this tutorial](#an-easy-fluvio-script--bash),
or filled continuously as seen just above. When taking input continuously, it
won't stop until it is terminated by the user.

Some useful option flags to be aware of:

* `-f <file name>`			– uses a file as input to be read and uploaded as multiple records.
* `--raw`					– specifies that the incoming data should be stored as a single record.
* `--key-separator <key>`	– specifies the string delineator to split the input into a key and value pairing.

#### Consume

`fluvio consume` is the main way to read data from the Fluvio database. Either
in scripting, or through the use of the CLI, most interactions will use `consume` in some way.

Here is an example of `consume` in action:

%copy first-line%
```bash
$ fluvio consume greetings -dB
Consuming records from the beginning of topic 'greetings'
Hello world!
test
meow
```

-> To quit `consume` press `CTRL-C`.

`fluvio consume <topic> [flags]` by default prints new records to the terminal as
they enter the database. By default it runs nonstop until quit – the examples
used here all use the `-d` flag to tell it to stop.

Some useful option flags to be aware of:

* `d`						– halts consumption after reaching the end of the records available.
* `T[int]`					– consumes only the T (default 10) most recent records.
* `B[int]`					– starts consuming records B (default 0) after the start of the database.
* `-p[int]`					– reads only from the partition p (default 0).
* `-k`						– displays the key portion of the key and value pairs.
* `-A`						– reads from all partitions available.
* `--smart-module <module>` – runs a [SmartModule](#smartmodules) module on the output, then displays the results.

## An Easy Fluvio Script — Bash

Here is a simple script that pushes timestamped comments to a remote database.

This code generates a message string that contains the current time, and the
contents of the first argument passed to it.

Once done, it checks to see if the hard coded topic exists. If it
does, good, it moves on. If not, it calls upon `fluvio topic create`
to generate the new topic. Then, it makes a call to `fluvio produce`
and sends the timestamped message off to the Cloud.

{{<code file="code/bash/timekeeper.sh" lang="bash" copy=true >}}

##### To Run the Script

Save the above script to a file – here it is named `timekeeper.sh` – and set it
as executable.

You can then run the script with:

%copy first-line%
```bash
$ ./timekeeper.sh 'I love cupcakes'
topic "timekeeper" created
```

Again, you can use `consume` to view the results.

%copy first-line%
```bash
$ fluvio consume timekeeper -dB
Consuming records from the beginning of topic 'timekeeper'
2022-09-01T18:46:12-07:00 : I love cupcakes
```

## Connectors

If you wish to automatically collect information from one source and send it to
Fluvio, Connectors are the way to go. When given the information on the
interface through the Connector configuration file, Fluvio can poll a multitude of
input types.

In this tutorial, we will be looking at the [HTTP Connector](/connectors/sources/http/) setup, connecting
to the `catfacts.ninja` JSON database.

### Connector Config Layout

This is the template YAML connector file. To make it useful, it needs to be
populated – which we will do in the next step. See
[the documentation](/connectors/) for the parameters available for use.

%copy%
```yaml
# connect.yml
version:
name:
# the name that you will see when listing the connectors
type:
# connection type:
# source connections currently support: HTTP, Kafka, MQTT, Postgres -source
# sink connections currently support: Dynabodb, Kafka, Postgres, Slack -sink
topic:
# the topic to produce to or consume from
direction:
# specifies as source or sink
parameters:
  # type specific parameters
  # smart module parameters
  # see the docs for the list of parameters

# producer:
#   linger:
#   batch-size:
#   compression:
# optional content for producer type connector

# consumer:
#   partition:
# optional content for consumer type connector
```

Thankfully, filling it out is simple. For any connection, you need a name,
the connection type, the direction in which it is connecting, and what topic
to connect to.

For the HTTP-specific parameters you will need to specify the link it is
polling, and the interval at which it polls.

%copy%
```yaml
# connect.yml
version: 0.1.0
name: cat-facts
type: http-source
topic: timekeeper-with-connector

direction: source
# HTTP specific parametes
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 30s
```

This creates a connector named `cat-facts`, that reads from the website
`https://catfacts.ninja/fact` every 30 seconds, and produces to the topic
`timekeeper-with-connector`.

### Adding Connectors to the Script

To add Connectors to the existing script, we will have to make some
modifications to it.

{{<code-highlight file="code/bash/timekeeper-connector.sh" lang="bash" copy=true lines=32-43 >}}

##### Running the Script

Save the above connector config in the file named `catfact.yml`. Next, save the
changes to `timekeeper.sh` as a new file. We named it
`timekeeper-connector.sh` to make the distinction easier to see.

%copy first-line%
```bash
$ ./timekeeper-connector.sh "test123"
topic "timekeeper-with-connector" created

```

Nothing much is returned when running, but now the database will be more
interesting. Try checking its contents with `fluvio consume`!

You can stop the connector by deleting it.

%copy first-line%
```bash
$ fluvio connector delete cat-facts
connector "cat-facts" deleted

```

Wait, now the Fluvio instance is getting cluttered! Sure the cat facts are nice,
but now there is JSON mixed in with the timestamped notes!

Fear not, we have *SmartModules* to help with that.

## SmartModules

SmartModules are user defined functions set to run on and modify the inputs/outputs to
a Fluvio database.

Want to filter so that only JSON records that match a specific priority tag are
recorded? A Filter SmartModule can be written to only let through records with
specific values.

You create a SmartModule by using Rust and generating it based on the SmartModule
template available [at the github repository](https://github.com/infinyon/fluvio-smartmodule-template).

### Making a SmartModule

If we want to clean up the timekeeper records and make it so that the messages
are marked separately from the JSON objects, we will need a SmartModule.
Specifically we will need a map SmartModule.

We need to go through some setup steps though.

First, check if `wasm32` is listed by `rustup` as an installed target.

%copy first-line%
```bash
$ rustup target list | grep installed
wasm32-unknown-unknown (installed)
x86_64-unknown-linux-gnu (installed)
```

-> If it is not installed, run `$ rustup target add wasm32-unknown-unknown` to install it.

Next, install `cargo-generate`, this may take a minute or two.

%copy first-line%
```bash
$ cargo install cargo-generate
```

Now you can download the template with `cargo-generate`.

-> You will have to fill in a couple of required arguments below, prefaced by the "🤷" emoji.

%copy first-line%
```bash
cargo generate --git https://github.com/infinyon/fluvio-smartmodule-template
🤷   Project Name : catfacts-map
🔧   Destination: /home/[...]/projects/[...]/catfacts-map ...
🔧   Generating template ...
✔ 🤷   Want to use SmartModule parameters? · true
✔ 🤷   Which type of SmartModule would you like? · map
[1/7]   Done: .cargo/config.toml
[2/7]   Done: .cargo
[3/7]   Done: .gitignore
[4/7]   Done: Cargo.toml
[5/7]   Done: README.md
[6/7]   Done: src/lib.rs
[7/7]   Done: src
🔧   Moving generated files into: `/home/[...]/projects/[...]/catfacts-map`...
💡   Initializing a fresh Git repository
✨   Done! New project created /home/[...]/projects/[...]/catfacts-map

```

There should now be a new directory labeled `catfacts-map` in your working directory.

Now we can edit the `catfacts-map/src/lib.rs` file to get what we need!

%copy%
```rust
use fluvio_smartmodule::{smartmodule, Result, Record, RecordData};
use serde_json::Value;

#[smartmodule(map, params)]
pub fn map(record: &Record, _params: &SmartModuleOpt) -> Result<(Option<RecordData>, RecordData)> {

    let key = RecordData::from(String::from("JSON")).into();
    // the key for the fluvio key value pairs -- setting it to report that it is a JSON key

    let input: Value = serde_json::from_slice(record.value.as_ref())?;
    // pulling the input value from the record
    let fact = &input["fact"];
    // getting just the fact portion of the JSON object

    let output = serde_json::to_string(fact)?;
    // string-ifying the JSON content
    Ok((key, output.into()))
    //returning the new key-value pair
}


#[derive(fluvio_smartmodule::SmartOpt, Default)]
pub struct SmartModuleOpt;
```

Now that we have the SmartModule created, we need to link it to Fluvio, so that
it can be used. Otherwise you would have to remember the entire path to the
`wasm` file.

%copy first-line%
```bash
$ fluvio smart-module create catfacts-map --wasm-file="catfacts-map/target/wasm32-unknown-unknown/release/catfacts_map.wasm"
smart-module "catfacts-map" has been created.
```

You can test that it is working by adding it to the Connector config!

### Connecting to a Connector

To use a SmartModule with a Connector, add it to the Connector config.
Currently you have to specify which type of SmartModule you are using, so it
will be the form of `module-type: name-of-module`.

```yaml
# connect.yml
version: 0.2.0
…
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 30s
  map: catfacts-map
```

-> In the near future the config arguments be updated so that you only specify that you are using a module; Fluvio will soon be able to take care of determining which module type is being used.

Now it is almost ready, you just need to tell Fluvio to update the Connector;
or recreate it if you deleted it earlier. Use the `update` argument for `fluvio
connector` to update the Connector if it still exists.

%copy first-line%
```bash
$ fluvio connector update --config=./catfact.yml
```

If you deleted the Connector, you can easily recreate it.

%copy first-line%
```bash
$ fluvio connector create --config=./catfact.yml
```

At this point you can run `fluvio consume` again and see how things have changed.

%copy first-line%
```bash
$ fluvio consume timekeeper-with-connector -dT 4
Consuming records starting 4 from the end of topic 'timekeeper-with-connector'
"The average lifespan of an outdoor-only cat is about 3 to 5 years while an indoor-only cat can live 16 years or much longer."
"In 1987, cats overtook dogs as the number one pet in America (about 50 million cats resided in 24 million homes in 1986). About 37% of American homes today have at least one cat."
"Cats should not be fed tuna exclusively, as it lacks taurine, an essential nutrient required for good feline health.  Make sure you have the proper Pet supplies to keep your cat happy and healthy."
"Cat paws act as tempetature regulators, shock absorbers, hunting and grooming tools, sensors, and more"
```

## Shell Script Complete

Now, we can make a few minor modifications to  `timekeeper.sh` so that the
messages are keyed. That way, if we use the `-k` flag, we can see the origins
of each message!

{{<code file="code/bash/timekeeper-full.sh" lang="bash" copy=true >}}

Now, if we save this as `timekeeper-full.sh`, and use it to send a few messages
to the Fluvio Topic, we can see the database with the keys active.

%copy first-line%
```bash
$ fluvio consume timekeeper-with-connector -dkT 4
Consuming records starting 4 from the end of topic 'timekeeper-with-connector'
[MSG] 2022-09-15T15:31:07-07:00 : hello world!
[JSON] "The cat appears to be the only domestic companion animal not mentioned in the Bible."
[MSG] 2022-09-15T15:31:26-07:00 : Lettuce see here, any rabbits to be found?
[JSON] "The largest cat breed is the Ragdoll. Male Ragdolls weigh between 12 and 20 lbs (5.4-9.0 k). Females weigh between 10 and 15 lbs (4.5-6.8 k)."
```

And voilà! We have a database that takes inputs from the user and from a website, and displays it all prettylike for whoever is reading it!

<!-- ## Check out these Other Tutorials -->

## References:

[Fluvio CLI Produce](/cli/commands/produce/)

[Fluvio CLI Consume](/cli/commands/consume/)

[Fluvio CLI topic](/cli/commands/topic/)

[Fluvio CLI profile](/cli/installation/profile/)

[Connectors](/connectors/)

[Smart Modules](/smartmodules/)

[Smart Module Rust API](https://docs.rs/fluvio-smartmodule/latest/fluvio_smartmodule/)
