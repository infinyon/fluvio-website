---
title: Using InfinyOn Cloud Backend — Connectors and SmartModules
menu: InfinyOn Cloud Tutorial
weight: 60
---

This tutorial will walk you through setting up an InfinyOn account, using Fluvio,
and learning the basics of connectors and SmartModules.

## Basic Setup

<img src="../images/Fluvio-Cloud-Flow.svg"
     alt="Signup flow for InfinyOn Cloud"
	 style="justify: center; max-width: 500px" />

There are 3 main steps for setting up for the Cloud: Installing the CLI,
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

##### Log in with Google

If you wish to avoid creating a new username and password, then click on the `Sign in with Google` button.

<img src="../images/google-signup-part1.jpg"
     alt="A screenshot of step one of Signing up to InfinyOn with Google — click on the sign in button"
     style="justify: center; max-width: 300px" />

The link will take you to a Google login prompt if you are not logged into google.

<img src="../images/google-signup-part2.jpg"
     alt="a screenshot of step two of Signing up to InfinyOn with Google — associate account"
     style="justify: center; max-width: 300px" />

Finally, it will ask if you are sure you want to log into InfinyOn Cloud.

##### Create an InfinyOn Cloud Account

<img src="../images/cloud-signup.jpg"
     alt="A screenshot of the InfinyOn new account form, with Name, Organization, Email, and Password fields"
     style="justify: center; max-width: 300px" />

After filling out the form, you'll be greeted with a success message telling you
to verify your email. You'll need to complete this step in order to continue.

~> The link in the email will time out after a few minutes.

<img src="../images/cloud-verification.jpg"
     alt="A screenshot of the verification email received after completing the signup form, including a verification link"
     style="justify: center; max-width: 500px" />

You should now get a confirmation that your account is ready to use.

<img src="../images/cloud-confirmation.jpg"
     alt="A screenshot of the prompt received after clicking the verification link, saying the account is ready to use"
     style="justify: center; max-width: 300px" />

At this point, you can log in via the Fluvio CLI and start sending and receiving
messages to your Fluvio cluster.

### Link InfinyOn Cloud to Fluvio CLI

Depending which method you chose to create your account, please follow the relevant
instructions below.

##### Connect with OAuth2

Use the command `fluvio cloud login --use-oauth2` to connect to the InfinyOn Cloud.

%copy first-line%
```bash
$ fluvio cloud login --use-oauth2
A web browser has been opened at https://infinyon-cloud.us.auth0.com/activate?user_code=GLMC-QDDJ.
Please proceed with authentication.
```

It will open a login screen in your webbrowser if possible. If that is not
possible, it will print a URL to you to enter into your web browser.

<img src="../images/google-login-part1.jpg"
     alt="screenshot showing verification code that Fluvio is trying to connect with."
     style="justify: center; max-width: 300px" />

Verify that the code matches what was displayed at the end of the url in the
command prompt, then click the confirm button.

<img src="../images/google-login-part2.jpg"
     alt="screenshot showing Google requesting you log in with your google account"
     style="justify: center; max-width: 300px" />

Next you will have to log into google. If you are already signed in, this step
is automatically skipped.

<img src="../images/google-login-part3.jpg"
     alt="screenshot showing OAuth2 requesting access to your Google account"
     style="justify: center; max-width: 300px" />

One last verification that you wish to connect to InfinyOn Cloud, click accept.

<img src="../images/google-login-confirm.jpg"
     alt="screenshot showing confirmation that OAuth2 connection is complete"
     style="justify: center; max-width: 300px" />

Congrats, everything is now set up!

##### Connect with username and password

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

The storage and retrieval of records from the topic as shown above are the basics
of using the Fluvio CLI.

The Fluvio CLI is currently the easiest way to interact with the Fluvio database.

Two core commands of Fluvio you will need to be familiar with are 
`fluvio produce <topic>` and `fluvio consume <topic>`

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

-> To quit the continuous mode of `produce` press `CTRL-C`.

`fluvio produce <topic> [flags]` takes in either input from stdin, or from a file. The
stdin input can be piped into [like in this tutorial](#an-easy-fluvio-script--bash),
or filled continuously as seen just above. When running in continuous mode, it won't stop
until it is killed by the user.

Some useful option flags to be aware of:

* `-f <file name>` – to use a file as input to be read and uploaded as multiple records.
* `--raw` – to specify that the incoming data should be stored as a single record.
* `--key-separator <key>` - to specify the string delineator to split the input into a key and value pairing.

#### Consume

`fluvio consume` is the main way to read out data from the Fluvio database. Either
in scripting, or through the use of the CLI, most actions will use `consume` in some way.

Here is an example of Consume in action:

%copy first-line%
```bash
$ fluvio consume greetings -dB
Consuming records from the beginning of topic 'greetings'
Hello world!
test
meow
```

-> To quit the continuous mode of `consume` press `CTRL-C`.

`fluvio consume <topic> [flags]` by default prints to the terminal new records as
they appear. By default it runs nonstop until quit, the examples used here all
use the `-d` flag to tell it to stop.

Some useful option flags to be aware of:

* `d` – to halt consumption after reaching the end of the records available.
* `T[int]` – to specify that it should consume only the T(default 10) most recent records.
* `B[int]` – to specify to start consuming records B(default 0) after the start of the database.
* `-p[int]` - to specify to read only from the partition p(default 0). 
* `-k` - to tell Fluvio to display the key and value pairs.
* `-A` - tells Fluvio to read from all partitions available.
* `--smart-module <module>` - To tell Fluvio to run a [SmartModule](#smartmodules) module on the output, then display the results.

## An Easy Fluvio Script — Bash

Here is a simple script that pushes timestamped comments to a remote database.

This code generates a string that contains the current time, and the contents of the first argument passed in.

Once that has happened, it checks to see if the hard coded topic exists. If it
does, good, it moves on. If not, then it calls upon `fluvio topic create`
to generate the new topic. Afterwards, it issues the call to `fluvio produce`
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
Fluvio, Connectors are the way to go. Fluvio, when given the information on the
interface through the Connector configuration file, can poll a multitude of
input types.

In this tutorial, we will be looking at the [HTTP Connector](/connectors/sources/http) setup, connecting
to the `catfacts.ninja` JSON database.

### Connector Config Layout

This is the barebones yaml connector file. It doesn't have any information, or 
sections for smart modules. 

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
  
#optional content for producer type connector
# producer:
#   linger:
#   batch-size:
#   compression:
  
#optional content for consumer type connector
#consumer:
#  partition:
```

To make it useful, it needs populating. See [the documentation](/connectors) for
the parameters available for use.

Thankfully, it's pretty easy at this stage. For any connection, you
need a name, the connection type, the direction in which it is connecting, what
topic to connect to. 

For the HTTP-specific parameters you will need to specify the link it is polling, and the 
interval at which it polls at.

%copy% 
```yaml
# connect.yml
version: 0.3.0
name: cat-facts
type: http-source
topic: timekeeper-with-connector

direction: source
# HTTP specific parametes
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 30s
```

This creates a connector named cat-facts, that produces to the topic greetings

### Adding Connectors to the Script

To add Connectors to the existing script, we shall have to make some modifications 
to it.

{{<code file="code/bash/timekeeper-connector.sh" lang="bash" copy=true >}}

##### Running the Script

Save the changes to `timekeeper.sh`, maybe in a new file. Save the above connector
config in the file named `catfact.yml`.

%copy first-line%
```bash
$ ./timekeeper-connector.sh "test123"
topic "timekeeper-with-connector" created

```

Again, nothing much returned when running; but now the database will be more 
interesting. Try checking its contents with `fluvio consume`!

You can stop the connector by deleting it. 

%copy first-line%
```bash
$ fluvio connector delete cat-facts
connector "cat-facts" deleted

```

Wait, now the Fluvio instance is getting cluttered! How are you supposed to sort 
through this mix of information? Sure the cat facts are nice, but now they are 
mixed in with the timestamped notes!

Fear not, for we have *SmartModules*.

## SmartModules

SmartModules are user defined functions set to run on and modify the inputs to
a Fluvio database. 

Want to filter which partition a JSON object goes in? Write a Map SmartModule
that adds a JSON key to it so it goes with other JSON records. 

Want to filter so that only JSON records that match a specific priority tag are
recorded? A Filter SmartModule can be written to only let through records with
specific values.

You create a SmartModule by using Rust and generating it based on the smartmodule
template available [at the github repository](https://github.com/infinyon/fluvio-smartmodule-template).

### Making a SmartModule

If we want to clean up the timekeeper records and make it so that the messages
are stored separate from the JSON objects, we will need a SmartModule.
Specifically we will need a map SmartModule.

We need to go through some setup steps though.

First, check if `wasm32` is listed by rustup as an installed target.

%copy first-line%
```bash
$ rustup target list | grep installed
wasm32-unknown-unknown (installed)
x86_64-unknown-linux-gnu (installed)
```

-> if it is not installed, run `$ rustup target add wasm32-unknown-unknown` to install it.

Next, install cargo-generate, this may take a minute or two.

%copy first-line%
```bash
$ cargo install cargo-generate
```

Now you can download the template with cargo-generate. 

-> You will have to fill out a couple of requirements below, prefaced by the 🤷 emoji.

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

```bash
$ cd catfacts-map/src
```

Now we can edit the `lib.rs` file to get what we need!

%copy%
```rust
use fluvio_smartmodule::{smartmodule, Result, Record, RecordData};
use serde_json::Value;

#[smartmodule(map, params)]
pub fn map(record: &Record, _params: &SmartModuleOpt) -> Result<(Option<RecordData>, RecordData)> {
    let key = record.key.clone();
    // the key for the fluvio key value pairs

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
_[TODO: find a permanent home for this]_

Now that we have the smart module created, we need to link it to Fluvio, so that
it can be used. Otherwise you would have to remember the entire path to the wasm file.

%copy first-line%
```bash
$ cd ../.. && fluvio smart-module create catfacts-map --wasm-file="catfacts-map/target/wasm32-unknown-unknown/release/catfacts_map.wasm"
smart-module "catfacts-map" has been created.
```

You can test that it is working with:

%copy first-line%
```bash
fluvio consume timekeeper -AB --smart-module catfacts
```

### Connecting to a Connecter

To use a smart module with a connector, add it to the connector config. 
Currently you have to specify which type of smartmodule you are using, so it
will be the form of `module-type: name-of-module`.

```yaml
…
parameters:
  endpoint: https://catfact.ninja/fact
  interval: 30s
  map: catfacts-map
```

-> In the near future the config arguments be updated so that you only specify that you are using a module; the module type will be automatically figured out.

Now it is almost ready, you just need to tell Fluvio to update the connector.
Or recreate it if you deleted it earlier. Use the `update` argument for `fluvio
connector` to update the connector if it still exists.

%copy first-line%
```bash
$ fluvio connector update --config=./catfact.yml
```

If you deleted the connector, you can easily recreate it.

At this point you can run `fluvio consume` again and see how things have changed.

%copy first-line%
```bash
$ fluvio consume timekeeper-with-connector -dT 4
Consuming records starting 4 from the end of topic 'timekeeper-multi'
"The average lifespan of an outdoor-only cat is about 3 to 5 years while an indoor-only cat can live 16 years or much longer."
"In 1987, cats overtook dogs as the number one pet in America (about 50 million cats resided in 24 million homes in 1986). About 37% of American homes today have at least one cat."
"Cats should not be fed tuna exclusively, as it lacks taurine, an essential nutrient required for good feline health.  Make sure you have the proper Pet supplies to keep your cat happy and healthy."
"Cat paws act as tempetature regulators, shock absorbers, hunting and grooming tools, sensors, and more"
```

Now there are a couple things we could do here. The messages could do with some
keys, for instance. Maybe make a filter that can be run on output that displays
either messages or JSON files.



## Check out these Other Tutorials

*Coming Soon*

## References:

[Fluvio CLI Produce](/cli/commands/produce)

[Fluvio CLI Consume](/cli/commands/consume)

[Fluvio CLI topic](/cli/commands/topic)

[Fluvio CLI profile](/cli/installation/profile)

[Connectors](/connectors)

[Smart Modules](/smartmodules)
